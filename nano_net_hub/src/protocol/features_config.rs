use crate::models::ServerError;
use driver_common::device_service::{CommandParam, CommandResponse, ConfigProtocol, DeviceService, EventData};
use std::collections::HashMap;
use std::sync::{mpsc, OnceLock};
use std::sync::mpsc::Sender;
use driver_common::DriverError;
use crate::protocol::data_event_handler::handle_event;

static DEVICE_SERVICES: OnceLock<HashMap<&'static str, Box<dyn DeviceService + 'static>>> = OnceLock::new();

pub async fn config_protocol(config: HashMap<String, String>) -> Result<(), ServerError> {
    let mut map = HashMap::new();
    let (tx, rx) = mpsc::channel::<EventData>();
    #[cfg(feature = "mqtt")]
    {
        let device_service = init_mqtt_driver(config, tx.clone()).await?;
        map.insert("mqtt", device_service);
    }
    DEVICE_SERVICES.get_or_init(|| map);
    handle_event(rx);
    Ok(())
}

pub async fn device_service_command(name: &str, command_param: CommandParam) -> Result<CommandResponse, DriverError> {
    let map = DEVICE_SERVICES.get().unwrap();
    let service = map.get(name).unwrap();
    let res = service.command(command_param).await?;
    Ok(res)
}

#[cfg(feature = "mqtt")]
pub async fn init_mqtt_driver(config: HashMap<String, String>, tx: Sender<EventData>) -> Result<Box<dyn DeviceService>, ServerError> {
    use mqtt_driver::MqttConsumer;
    
    let mut mqtt_consumer = MqttConsumer::default();
    mqtt_consumer.config(config)?;
    mqtt_consumer.set_event_sender(tx);
    mqtt_consumer.start().await?;
    Ok(Box::new(mqtt_consumer))
}
