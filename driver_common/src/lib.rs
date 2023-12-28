use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub mod device_service;
pub mod utils;

#[derive(Debug)]
pub enum DriverError {
    // 未连接
    NotConnect,
    // 序列化错误
    SerializationError(String),
    MissingField(String),
}

impl Display for DriverError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DriverError::NotConnect => {
                write!(f, "没有内容",)
            }
            DriverError::SerializationError(value) => {
                write!(f, "序列化异常: {:?}", value)
            }
            DriverError::MissingField(msg) => {
                write!(f, "缺失配置字段: {:?}", msg)
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    INT(i32),
    BOOL(bool),
    STRING(String),
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::INT(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::BOOL(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::STRING(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::STRING(value.to_string())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::INT(value) => {
                write!(f, "{}", value)
            }
            Value::BOOL(value) => {
                write!(f, "{}", value)
            }
            Value::STRING(value) => {
                write!(f, "{}", value)
            }
        }
    }
}
