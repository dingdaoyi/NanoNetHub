use driver_common::device_service::ConfigProtocol;
use driver_common::utils::generate_random_string;
use driver_common::DriverError;
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::{AsyncClient, MqttOptions};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::task;

#[derive(Debug, Clone, Default)]
struct MqttConfig {
    client_id: String,
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>,
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

        let username = value.get("username").cloned();
        let password = value.get("password").cloned();

        Ok(MqttConfig {
            client_id,
            host: host.into(),
            port,
            username,
            password,
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct MqttConsumer {
    config: MqttConfig,
    client: Arc<Mutex<Option<AsyncClient>>>,
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
        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
        client
            .subscribe("hello/rumqtt", QoS::AtMostOnce)
            .await
            .unwrap();
        self.client.lock().unwrap().replace(client);
        task::spawn(async move {
            loop {
                let notification = eventloop.poll().await.unwrap();
                println!("Received = {:?}", notification);
            }
        });
        Ok(())
    }
}
