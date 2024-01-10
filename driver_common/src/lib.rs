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
    ProtocolError(String),
    TimeoutError,
    ClientNotInitialized,
}

impl Display for DriverError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DriverError::NotConnect => {
                write!(f, "没有内容", )
            }
            DriverError::SerializationError(value) => {
                write!(f, "序列化异常: {:?}", value)
            }
            DriverError::MissingField(msg) => {
                write!(f, "缺失配置字段: {:?}", msg)
            }
            DriverError::ProtocolError(msg) => {
                write!(f, "协议传输错误: {:?}", msg)
            }

            DriverError::TimeoutError => {
                write!(f, "请求超时")
            }
            DriverError::ClientNotInitialized => {
                write!(f, "客户端未初始化")
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    INT(i32),
    DOUBLE(f64),
    BOOL(bool),
    STRING(String),
}

// 直接提取值的方法：
impl Value {
    pub fn into_int(self) -> Option<i32> {
        match self {
            Self::INT(value) => Some(value),
            _ => None,
        }
    }
    pub fn into_bool(self) -> Option<bool> {
        match self {
            Self::BOOL(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_double(self) -> Option<f64> {
        match self {
            Self::DOUBLE(value) => Some(value),
            _ => None,
        }
    }
    pub fn into_string(self) -> Option<String> {
        match self {
            Self::STRING(value) => Some(value),
            _ => None,
        }
    }
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

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::DOUBLE(value)
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
            Value::DOUBLE(value) => {
                write!(f, "{}", value)
            }
        }
    }
}

