use driver_common::device_service::{CommandParam, CommandResponse, ConfigProtocol, DeviceService, EventData, ProtocolData};
use driver_common::utils::generate_random_string;
use driver_common::DriverError;
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::{AsyncClient, Event, EventLoop, Incoming, MqttOptions};
use std::collections::HashMap;
use bytes::Bytes;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use async_trait::async_trait;
use rumqttc::v5::mqttbytes::v5::{Filter, PublishProperties};
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;
use tokio::sync::oneshot::Sender;
use tokio::task;
use std::str;

const STATIC_TOPIC: &str = "NanoNetHub";

#[derive(Debug)]
enum MqttTopic {
    Event(String, String),
    CommandReply(String, String),
}

impl From<Bytes> for MqttTopic {
    fn from(value: Bytes) -> Self {
        let data: Vec<&str> = value.split(|&c| c == b'/').map(|s| str::from_utf8(s).unwrap_or_default()).collect();
        match data.as_slice()[data.len() - 3..] {
            [product_key, device_code, "event"] => MqttTopic::Event(product_key.to_string(), device_code.to_string()),
            [product_key, device_code, "command_reply"] => MqttTopic::CommandReply(product_key.to_string(), device_code.to_string()),
            _ => MqttTopic::Event("default_product_key".to_string(), "default_device_code".to_string()),
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ProtocolParam {
    header: ProtocolHeader,
    body: Option<ProtocolBody>,
}


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ProtocolBody {
    identifier: String,
    data: Vec<ProtocolData>,
}

impl ProtocolBody {
    pub fn new(identifier: String, data: Vec<ProtocolData>) -> Self {
        Self { identifier, data }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ProtocolHeader {
    parent: Option<Vec<String>>,
    device_code: Option<String>,
    product_key: Option<String>,
    timestamp: u64,
    message_id: u32,
}

impl From<CommandParam> for ProtocolParam {
    fn from(value: CommandParam) -> Self {
        let parent = value.parent_code;
        let parent = if parent.is_empty() {
            None
        } else {
            Some(parent)
        };
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let body = if let Some(identifier) = value.identifier {
            Some(ProtocolBody::new(identifier, value.params))
        } else {
            None
        };
        CommandParam::increment_message_id();
        Self {
            header: ProtocolHeader {
                parent,
                device_code: None,
                product_key: None,
                timestamp,
                message_id: CommandParam::increment_message_id(),
            },
            body,
        }
    }
}


impl From<ProtocolParam> for EventData {
    fn from(value: ProtocolParam) -> Self {
        let header = value.header;
        let device_code = header.device_code.unwrap_or("".into());
        let product_key = header.product_key.unwrap_or("".into());
        let (identifier, params) = value.body.map_or(("".to_string(), vec![]), |body| {
            (body.identifier, body.data)
        });
        Self {
            device_code,
            product_key,
            identifier,
            params,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct MqttConfig {
    client_id: String,
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>,
    command_default_timeout: u64,
    topic_prefix: Option<String>,
}

impl TryFrom<HashMap<String, String>> for MqttConfig {
    type Error = DriverError;

    fn try_from(value: HashMap<String, String>) -> Result<Self, Self::Error> {
        let client_id = value
            .get("client_id")
            .cloned()
            .unwrap_or_else(|| format!("nethub_{}", generate_random_string(5)));

        let host = value
            .get("host")
            .cloned()
            .ok_or_else(|| DriverError::MissingField("host".to_string()))?;

        let port = value
            .get("port")
            .and_then(|port_str| port_str.parse().ok())
            .ok_or_else(|| DriverError::MissingField("port".to_string()))?;
        let command_default_timeout = value
            .get("command_default_timeout")
            .and_then(|port_str| port_str.parse().ok());

        let username = value.get("username").cloned();
        let password = value.get("password").cloned();
        let topic_prefix = value.get("topic_prefix").cloned();

        Ok(MqttConfig {
            client_id,
            host,
            port,
            username,
            password,
            command_default_timeout: command_default_timeout.unwrap_or(10),
            topic_prefix,
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct MqttConsumer {
    config: MqttConfig,
    client: Arc<Mutex<Option<AsyncClient>>>,
    sender: Option<std::sync::mpsc::Sender<EventData>>,
    response_que: Arc<Mutex<HashMap<u32, Sender<CommandResponse>>>>,
}

#[async_trait]
impl DeviceService for MqttConsumer {
    async fn command(&self, data: CommandParam) -> Result<CommandResponse, DriverError> {
        let (tx, rx) = oneshot::channel();
        self.publish(data, tx).await?;
        // 等待响应，设置一个超时时间，防止无限等待
        let response = tokio::time::timeout(Duration::from_secs(self.config.command_default_timeout), rx).await;
        match response {
            Ok(Ok(response)) => Ok(response),
            Ok(Err(_)) | Err(_) => Err(DriverError::TimeoutError),
        }
    }

    fn set_event_sender(&mut self, sender: std::sync::mpsc::Sender<EventData>) {
        self.sender = Some(sender);
    }
}

impl ConfigProtocol for MqttConsumer {
    fn config(&mut self, config: HashMap<String, String>) -> Result<(), DriverError> {
        let res = MqttConfig::try_from(config)?;
        self.config = res;
        Ok(())
    }
}

impl MqttConsumer {
    pub fn new() -> Self {
        Self::default()
    }
    pub async fn start(&self) -> Result<(), DriverError> {
        tracing::info!("start mqtt consumer");
        let config = self.config.clone();
        let mut mqttoptions = MqttOptions::new(config.client_id, config.host, config.port);
        mqttoptions.set_keep_alive(Duration::from_secs(5));
        if config.username.is_some() && config.password.is_some() {
            mqttoptions.set_credentials(config.username.unwrap(), config.password.unwrap());
        }
        let (client, event_loop) = AsyncClient::new(mqttoptions, 10);
        self.subscribe_init_client(client).await;
        self.run_subscribe_task(event_loop);
        Ok(())
    }

    fn run_subscribe_task(&self, mut eventloop: EventLoop) {
        let response_que = self.response_que.clone();
        let sender = self.sender.clone().unwrap();
        task::spawn(async move {
            loop {
                let notification = eventloop.poll().await;
                if let Ok(notification) = notification {
                    match notification {
                        Event::Incoming(Incoming::Publish(data)) => {
                            let bytes = data.topic;
                            let topic = MqttTopic::from(bytes);
                            let bytes = data.payload;
                            let res = serde_json::from_slice::<ProtocolParam>(&bytes);
                            if let Ok(res) = res {
                                match topic {
                                    MqttTopic::Event(product_key, device_code) => {
                                        Self::handle_event(sender.clone(), res, product_key, device_code);
                                    }
                                    MqttTopic::CommandReply(product_key, device_code) => {
                                        Self::handle_command_reply_message(response_que.clone(), res, product_key, device_code);
                                    }
                                }
                            }
                        }

                        eve => {
                            tracing::debug!("default data: {:?}",eve);
                        }
                    }
                }
            }
        });
    }

    fn handle_event(sender: std::sync::mpsc::Sender<EventData>, mut res: ProtocolParam, product_key: String, device_code: String) {
        tracing::debug!("event data: {:?},product_key:{},device_code:{}",res,product_key,device_code);
        res.header.product_key = Some(product_key);
        res.header.device_code = Some(device_code);
        let event_data = EventData::from(res);
        if let Err(msg) = sender.send(event_data) {
            tracing::debug!("event has closed: {:?}", msg);
        }
    }

    fn handle_command_reply_message(response_que: Arc<Mutex<HashMap<u32, Sender<CommandResponse>>>>, data: ProtocolParam, product_key: String, device_code: String) {
        tracing::debug!("command reply: {:?},product_key:{},device_code:{}",data,product_key,device_code);
        let message_id = data.header.message_id;
        let mut guard = response_que.lock().unwrap();
        if guard.contains_key(&message_id) {
            let tx = guard.remove(&message_id);
            if let Some(tx) = tx {
                let mut command_response = CommandResponse::default();
                if let Some(body) = data.body {
                    command_response.identifier = Some(body.identifier);
                };
                if let Err(msg) = tx.send(command_response) {
                    tracing::debug!("command has closed: {:?}", msg);
                }
            }
        }
    }

    async fn subscribe_init_client(&self, client: AsyncClient) {
        let prefix = self.config.topic_prefix.clone().unwrap_or(STATIC_TOPIC.to_string());
        let topics = vec![
            Filter::new(format!("{}/+/+/event", prefix), QoS::AtMostOnce),
            Filter::new(format!("{}/+/+/command_reply", prefix), QoS::AtLeastOnce),
        ];
        client
            .subscribe_many(topics)
            .await
            .unwrap();
        self.client.lock().unwrap().replace(client);
    }
    async fn publish(&self, data: CommandParam, tx: oneshot::Sender<CommandResponse>) -> Result<(), DriverError> {
        let client = &self.client.lock().unwrap().clone().unwrap();
        let protocol_param = ProtocolParam::from(data.clone());
        let message_id = protocol_param.header.message_id;
        self.response_que.lock().unwrap().insert(message_id, tx);
        let protocol_param = serde_json::to_vec(&protocol_param)
            .map_err(|e| DriverError::SerializationError(e.to_string()))?;
        let mut properties = PublishProperties::default();
        properties.response_topic = Some("hello/rumqtt".into());
        client.publish_with_properties(format!("/{}/{}/service", data.product_key, data.device_code), QoS::AtMostOnce,
                                       false, protocol_param, properties).await
            .map_err(|e| DriverError::ProtocolError(e.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod testing {
    use regex::Regex;
    use super::*;
    use tokio::sync::oneshot;

    #[tokio::test]
    pub async fn test1() {
        let prefix = Some("your_prefix".to_string()); // replace with your actual prefix
        let prefix = STATIC_TOPIC.to_string();

        let topics = vec![
            Bytes::from(format!("{}/+/+/event", &prefix)),
            Bytes::from(format!("{}/+/+/command_reply", &prefix)),
        ];

        // Extracting topic parts without prefix using regex
        for filter in topics {
            let re = Regex::new(&format!("^{}", &prefix)).expect("Invalid regex pattern");
            let topic: MqttTopic = filter.into();
            println!("{:?}", topic);
        }
    }
}
