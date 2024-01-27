use crate::{DriverError, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::mpsc::Sender;
use async_trait::async_trait;

static MESSAGE_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProtocolMessage {
    #[serde(rename = "Command")]
    Command(CommandParam),
    #[serde(rename = "CommandResponse")]
    Response(CommandResponse),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommandParam {
    pub parent_code: Vec<String>,
    pub device_code: String,
    pub group_code: i32,
    pub product_key: String,
    pub identifier: Option<String>,
    pub params: Vec<ProtocolData>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommandResponse {
    pub parent_code: Vec<String>,
    pub device_code: String,
    pub group_code: i32,
    pub identifier: Option<String>,
    pub params: Vec<ProtocolData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventData {
    pub device_code: String,
    pub product_key: String,
    pub identifier: String,
    pub params: Vec<ProtocolData>,
}

impl CommandParam {
    pub fn increment_message_id() -> u32 {
        MESSAGE_ID_COUNTER.fetch_add(1, Ordering::SeqCst)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProtocolData {
    pub identifier: String,
    pub value: Value,
}

impl ProtocolData {
    pub fn new(identifier: String, value: Value) -> Self {
        Self { identifier, value }
    }
}


#[async_trait]
pub trait DeviceService: Sync + Send {
    /// 指令下发
    async fn command(&self, data: CommandParam) -> Result<CommandResponse, DriverError>;

    /// 设置接收者
    fn set_event_sender(&mut self, sender: Sender<EventData>);
}

pub trait ConfigProtocol {
    fn config(&mut self, config: HashMap<String, String>) -> Result<(), DriverError>;
}
