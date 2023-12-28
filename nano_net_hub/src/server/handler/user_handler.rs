use axum::{Json, Router};
use axum::routing::post;
use crate::models::{R, ServerError};
use crate::models::common::user::{LoginQuery, UserInfo};
use crate::server::handler::base::Controller;

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
    async fn login(
        Json(LoginQuery { username, password }): Json<LoginQuery>,
    ) -> Result<Json<R<UserInfo>>, ServerError> {
        tracing::debug!("用户登录:{}",username);
        if username.eq_ignore_ascii_case("admin") && password.eq_ignore_ascii_case("123456") {
            return Ok(Json(R::success_with_data(UserInfo::default())));
        }
        Ok(Json(R::success()))
    }
}