use axum::{Json, Router};
use axum::routing::get;
use crate::models::ServerError;
use crate::server::handler::base::Controller;
use crate::server::handler::device_handler::DeviceHandler;
use crate::server::handler::product_handler::ProductHandler;
use crate::server::handler::property_handler::PropertyHandler;
use crate::server::handler::service_handler::ServiceHandler;
use crate::server::handler::unit_handler::UnitHandler;
use crate::server::handler::user_handler::UserHandler;

pub fn server_router() -> Router {
    Router::new()
        .nest("/api", routers())
}

//api
fn routers() -> Router {
    need_auth_routers().merge(no_need_auth_routers())
}

async fn health() -> Result<Json<String>, ServerError> {
    Ok(Json("success".to_string()))
}

//需要权限认证的路由
fn need_auth_routers() -> Router {
    Router::new()
}

//不需要权限认证的路由
fn no_need_auth_routers() -> Router {
    Router::new()
        .route("/health", get(health))
        .merge(ProductHandler::default().router())
        .merge(UserHandler::default().router())
        .merge(UnitHandler::default().router())
        .merge(PropertyHandler::default().router())
        .merge(ServiceHandler::default().router())
        .merge(DeviceHandler::default().router())
}
