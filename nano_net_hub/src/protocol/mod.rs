use crate::models::ServerError;
use driver_common::device_service::ConfigProtocol;
use std::collections::HashMap;

pub async fn config_protocol(config: HashMap<&str, &str>) -> Result<(), ServerError> {
    #[cfg(feature = "mqtt")]
    init_mqtt_driver(config).await?;
    Ok(())
}

#[cfg(feature = "mqtt")]
pub async fn init_mqtt_driver(config: HashMap<&str, &str>) -> Result<(), ServerError> {
    use mqtt_driver::MqttConsumer;
    let mut mqtt_consumer = MqttConsumer::default();
    mqtt_consumer.config(config)?;
    mqtt_consumer.start().await?;
    Ok(())
}
