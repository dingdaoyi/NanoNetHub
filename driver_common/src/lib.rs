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
    CastError(String),
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
            DriverError::CastError(msg) => {
                write!(f, "转换错误: {:?}", msg)
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
    pub fn into_inner<T>(self) -> Option<T> where T: TryFrom<Value, Error=DriverError> {
        self.try_into().ok()
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::INT(value)
    }
}

impl TryFrom<Value> for i32 {
    type Error = DriverError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::INT(value) => Ok(value),
            _ => Err(DriverError::CastError("类型转换错误".to_string())),
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::BOOL(value)
    }
}

impl TryFrom<Value> for bool {
    type Error = DriverError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::BOOL(value) => Ok(value),
            _ => Err(DriverError::CastError("类型转换错误".to_string())),
        }
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::DOUBLE(value)
    }
}

impl TryFrom<Value> for f64 {
    type Error = DriverError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::DOUBLE(value) => Ok(value),
            _ => Err(DriverError::CastError("类型转换错误".to_string())),
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::STRING(value)
    }
}

impl TryFrom<Value> for String {
    type Error = DriverError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::STRING(value) => Ok(value),
            _ => Err(DriverError::CastError("类型转换错误".to_string())),
        }
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

