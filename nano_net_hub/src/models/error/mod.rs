use crate::models::R;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use driver_common::DriverError;
use std::fmt::{Display, Formatter};
use std::io::Error;
use axum::extract::multipart::MultipartError;

/// 平台统一异常
#[derive(Debug)]
pub enum ServerError {
    ///常规异常
    Message(String),
    IoError(String),
    SqlxError(sqlx::Error),
    DriverError(DriverError),
    AuthError(AuthError),
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::Message(msg) => {
                write!(f, "通用异常: {}", msg)
            }
            ServerError::IoError(msg) => {
                write!(f, "IO异常: {}", msg)
            }
            ServerError::SqlxError(error) => {
                write!(f, "IO异常: {:?}", error)
            }
            ServerError::DriverError(error) => {
                write!(f, "IO异常: {:?}", error)
            }
            ServerError::AuthError(error) => {
                write!(f, "IO异常: {:?}", error)
            }
        }
    }
}

impl std::error::Error for ServerError {}


#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl From<AuthError> for ServerError {
    fn from(value: AuthError) -> Self {
        ServerError::AuthError(value)
    }
}

impl From<MultipartError> for ServerError {
    fn from(value: MultipartError) -> Self {
        ServerError::IoError(value.to_string())
    }
}

impl From<std::io::Error> for ServerError {
    fn from(value: Error) -> Self {
        ServerError::IoError(value.to_string())
    }
}

impl From<DriverError> for ServerError {
    fn from(value: DriverError) -> Self {
        ServerError::DriverError(value)
    }
}

impl From<sqlx::Error> for ServerError {
    fn from(value: sqlx::Error) -> Self {
        ServerError::SqlxError(value)
    }
}

/// 异常统一转换为response
impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ServerError::Message(msg) => {
                tracing::error!("{}", msg);
                (StatusCode::BAD_REQUEST, R::<String>::fail(msg))
            }
            ServerError::IoError(msg) => {
                tracing::error!("{}", msg);
                (StatusCode::BAD_REQUEST, R::<String>::fail(msg))
            }
            ServerError::SqlxError(error) => {
                tracing::error!("{}", error);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    R::<String>::fail(error.to_string()),
                )
            }
            ServerError::DriverError(error) => {
                tracing::error!("{:?}", error);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    R::<String>::fail(error.to_string()),
                )
            }
            ServerError::AuthError(_) => {
                (StatusCode::UNAUTHORIZED, R::<String>::fail_with_code(
                    401, "认证失败".to_string()))
            }
        };
        let body = Json(error_message);

        (status, body).into_response()
    }
}
