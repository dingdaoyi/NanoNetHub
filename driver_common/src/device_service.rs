use crate::{DriverError, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

static MESSAGE_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProtocolMessage {
    #[serde(rename = "Command")]
    Command(CommandParam),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandParam {
    pub parent_code: Vec<String>,
    pub device_code: String,
    pub group_code: i32,
    pub identifier: Option<String>,
    pub params: Vec<ProtocolData>,
}

impl CommandParam {
    pub fn increment_message_id() -> u32 {
        MESSAGE_ID_COUNTER.fetch_add(1, Ordering::SeqCst)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProtocolData {
    identifier: String,
    value: Value,
}

impl ProtocolData {
    pub fn new(identifier: String, value: Value) -> Self {
        Self { identifier, value }
    }
}

pub trait DeviceCommand {
    fn param(&self) -> &CommandParam;
}

pub trait DeviceService {
    type Command: DeviceCommand;
    fn invoke(&self, command: &Self::Command) -> Result<(), DriverError> {
        let command_param = command.param();
        println!("command_param: {:?}", command_param);
        Ok(())
    }

    fn get_bytes(&self) -> Result<Vec<u8>, DriverError>;
}

pub trait ConfigProtocol {
    fn config(&mut self, config: HashMap<&str, &str>) -> Result<(), DriverError> {
        let config: HashMap<String, String> = config
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<HashMap<String, String>>();
        self.set_config(config)
    }
    fn set_config(&mut self, config: HashMap<String, String>) -> Result<(), DriverError>;
}
