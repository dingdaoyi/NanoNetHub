use std::sync::OnceLock;
use async_trait::async_trait;
use axum::{Json, Router};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::routing::post;
use chrono::{Duration, Utc};
use headers::authorization::{Bearer, Credentials};
use headers::HeaderValue;
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use crate::config::AuthConfig;
use crate::models::{R, ServerError};
use crate::models::common::user::LoginQuery;
use crate::models::error::AuthError;
use crate::server::handler::base::Controller;


static AUTH_CONFIG: OnceLock<AuthConfig> = OnceLock::new();

pub fn set_auth_config(auth: AuthConfig) {
    AUTH_CONFIG
        .set(auth)
        .expect("Auth utils has already been set");
}

pub fn get_auth_config() -> Option<&'static AuthConfig> {
    AUTH_CONFIG.get()
}

#[derive(Default)]
pub struct UserHandler;

impl Controller for UserHandler {
    fn router(&self) -> Router {
        Router::new()
            .route("/login", post(Self::login))
    }
}

impl UserHandler {
    // 创建

    // 用户登录
    pub async fn login(Json(payload): Json<LoginQuery>) -> Result<Json<R<String>>, ServerError> {
        if payload.username.is_empty() || payload.password.is_empty() {
            return Err(ServerError::AuthError(AuthError::MissingCredentials));
        }
        let auth_config = get_auth_config().ok_or(ServerError::AuthError(AuthError::MissingCredentials))?;
        if !(auth_config.username == payload.username && auth_config.password == payload.password) {
            return Err(ServerError::Message("用户名或密码错误".into()));
        }

        let token = Self::generate_jwt_token(
            &payload.username,
            &auth_config.jwt_secret,
            auth_config.expire_minutes,
        )?;
        // 将 JWT 令牌发送到响应中
        Ok(Json(R::success_with_data(token)))
    }

    fn generate_jwt_token(
        username: &str,
        jwt_secret: &str,
        expire_minutes: usize,
    ) -> Result<String, AuthError> {
        let current_time = Utc::now();
        let expiration_time = current_time + Duration::minutes(expire_minutes as i64);

        let claims = Claims {
            username: username.to_owned(),
            exp: expiration_time.timestamp() as usize,
        };

        encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_bytes()),
        )
            .map_err(|_| AuthError::TokenCreation)
    }
}


#[async_trait]
impl<B> FromRequestParts<B> for Claims
    where
        B: Send,
{
    type Rejection = ServerError;

    async fn from_request_parts(parts: &mut Parts, _state: &B) -> Result<Self, Self::Rejection> {
        let token = match parts.headers.get("authorization")
            .and_then(|value|
                Bearer::decode(&HeaderValue::from_bytes(value.as_bytes()).ok()?
                )) {
            Some(token) => format!("{}", token.token()),
            None => return Err(ServerError::AuthError(AuthError::MissingCredentials)),
        };

        let auth_config = get_auth_config().ok_or(ServerError::AuthError(AuthError::MissingCredentials))?;
        let token_data = decode::<Claims>(
            token.as_str(),
            &DecodingKey::from_secret(auth_config.jwt_secret.as_bytes()),
            &Validation::default(),
        )
            .map_err(|error| {
                tracing::error!("转换错误{}", error);
                AuthError::InvalidToken
            })?;

        Ok(token_data.claims)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    username: String,
    exp: usize, // 强制性过期时间，使用 UTC 时间戳
}
