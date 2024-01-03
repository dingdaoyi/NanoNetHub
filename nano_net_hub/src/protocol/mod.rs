use crate::models::ServerError;
use driver_common::device_service::{CommandParam, CommandResponse, ConfigProtocol, DeviceService};
use std::collections::HashMap;
use std::sync::OnceLock;
use driver_common::DriverError;

static DEVICE_SERVICES: OnceLock<HashMap<&'static str, Box<dyn DeviceService + 'static>>> = OnceLock::new();

pub async fn config_protocol(config: HashMap<&str, &str>) -> Result<(), ServerError> {
    let mut map = HashMap::new();
    #[cfg(feature = "mqtt")]
    {
        let device_service = init_mqtt_driver(config).await?;
        map.insert("mqtt", device_service);
    }
    DEVICE_SERVICES.get_or_init(|| map);
    Ok(())
}

pub async fn device_service_command(name: &str, command_param: CommandParam) -> Result<CommandResponse, DriverError> {
    let map = DEVICE_SERVICES.get().unwrap();
    let service = map.get(name).unwrap();
    let res = service.command(command_param).await?;
    Ok(res)
}

#[cfg(feature = "mqtt")]
pub async fn init_mqtt_driver(config: HashMap<&str, &str>) -> Result<Box<dyn DeviceService>, ServerError> {
    use mqtt_driver::MqttConsumer;
    let mut mqtt_consumer = MqttConsumer::default();
    mqtt_consumer.config(config)?;
    mqtt_consumer.start().await?;
    Ok(Box::new(mqtt_consumer))
}
