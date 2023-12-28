mod mqtt_consumer;

use driver_common::device_service::{CommandParam, DeviceCommand, DeviceService, ProtocolData};
use driver_common::DriverError;
pub use mqtt_consumer::MqttConsumer;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

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

pub struct Command(CommandParam);

impl DeviceCommand for Command {
    fn param(&self) -> &CommandParam {
        &self.0
    }
}

impl DeviceService for ProtocolParam {
    type Command = Command;

    fn get_bytes(&self) -> Result<Vec<u8>, DriverError> {
        serde_json::to_vec(self).map_err(|e| DriverError::SerializationError(e.to_string()))
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    use serde::de::Unexpected::Option;

    #[test]
    fn test_serialize() {
        let param = CommandParam {
            parent_code: vec![],
            device_code: "test1".into(),
            group_code: 0,
            identifier: Some("test".into()),
            params: vec![ProtocolData::new("yali".into(), Value::INT(1))],
        };
        let mut param1 = ProtocolParam::from(param.clone());
        param1.invoke(&Command(param)).unwrap();
        let bytes = param1.get_bytes().unwrap();
        println!("{:?}", String::from_utf8(bytes));
    }
}
