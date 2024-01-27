use std::env;
use crate::config::database::DataSourceType;
use crate::models::ServerError;
use crate::protocol::config_protocol;
use crate::server::Server;
use sqlx::sqlite::SqliteRow;
use sqlx::{Pool, Sqlite};
use crate::config::{get_config, MqttConfig};

mod config;
pub mod models;
mod protocol;
mod server;
mod data;

// config sqlite db
pub type SqlPool = Pool<Sqlite>;
pub type SqlRow = SqliteRow;

pub async fn run_server() -> Result<(), ServerError> {
    let config = get_config();
    DataSourceType::Sqlite.init_pool().await?;
    setup_logger().await?;
    #[cfg(feature = "mqtt")]
    start_protocol_driver(config.mqtt).await?;
    Server::new(config.server)
        .start()
        .await?;
    Ok(())
}

async fn start_protocol_driver(mqtt_config: Option<MqttConfig>) -> Result<(), ServerError> {
    let res = mqtt_config.unwrap().get_map();
    config_protocol(res).await?;
    Ok(())
}

// 初始化日志
async fn setup_logger() -> Result<(), ServerError> {
    env::set_var("RUST_LOG", "debug");
    // 设置用户缓存
    tracing_subscriber::fmt::init();
    Ok(())
}
