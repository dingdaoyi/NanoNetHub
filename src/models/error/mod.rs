use std::fmt::{Display, Formatter};

/// 平台统一异常
#[derive(Debug)]
pub enum ServerError {
    ///常规异常
    Message(String),
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self { ServerError::Message(msg) => {
            write!(f, "通用异常: {}", msg)
        } }
    }
}

impl std::error::Error for ServerError {}
