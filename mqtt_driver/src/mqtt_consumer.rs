use driver_common::device_service::{CommandParam, CommandResponse, ConfigProtocol, DeviceService, ProtocolData};
use driver_common::utils::generate_random_string;
use driver_common::DriverError;
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::{AsyncClient, Event, EventLoop, Incoming, MqttOptions};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use async_trait::async_trait;
use rumqttc::v5::mqttbytes::v5::PublishProperties;
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;
use tokio::task;


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
                product_key: None,
                timestamp,
                message_id: CommandParam::increment_message_id(),
            },
            body,
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

        Ok(MqttConfig {
            client_id,
            host: host.into(),
            port,
            username,
            password,
            command_default_timeout: command_default_timeout.unwrap_or(10),
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct MqttConsumer {
    config: MqttConfig,
    client: Arc<Mutex<Option<AsyncClient>>>,
    response_que: Arc<Mutex<HashMap<u32, oneshot::Sender<CommandResponse>>>>,
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
}

impl ConfigProtocol for MqttConsumer {
    fn set_config(&mut self, config: HashMap<String, String>) -> Result<(), DriverError> {
        let res = MqttConfig::try_from(config)?;
        self.config = res;
        Ok(())
    }
}

impl MqttConsumer {
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
        task::spawn(async move {
            loop {
                let notification = eventloop.poll().await;
                if notification.is_ok() {
                    match notification.unwrap() {
                        Event::Incoming(Incoming::Publish(data)) => {
                            tracing::info!("publish data: {:?}",data);
                            let bytes = data.payload;
                            let res = serde_json::from_slice::<ProtocolParam>(&bytes.to_vec());
                            match res {
                                Ok(res) => {
                                    let message_id = res.header.message_id;
                                    let mut guard = response_que.lock().unwrap();
                                    if guard.contains_key(&message_id) {
                                        let tx = guard.remove(&message_id);
                                        if let Some(tx) = tx {
                                            let mut command_response = CommandResponse::default();
                                            if let Some(body) = res.body {
                                                command_response.identifier = Some(body.identifier);
                                            };
                                            let res = tx.send(command_response);
                                            match res {
                                                Ok(_) => {}
                                                Err(msg) => {
                                                    tracing::debug!("command has closed : {:?}",msg);
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(msg) => {
                                    tracing::error!("解析数据错误: {:?}",msg);
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

    async fn subscribe_init_client(&self, client: AsyncClient) {
        client
            .subscribe("hello/rumqtt", QoS::AtMostOnce)
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
