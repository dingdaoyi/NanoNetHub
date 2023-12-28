use std::collections::HashMap;
use std::env;

use crate::config::database::DataSourceType;
use crate::models::ServerError;
use crate::protocol::config_protocol;
use crate::server::{Server, ServerConfig};
use sqlx::sqlite::SqliteRow;
use sqlx::{Pool, Sqlite};

mod config;
pub mod models;
mod protocol;
mod server;

// config sqlite db
pub type SqlPool = Pool<Sqlite>;
pub type SqlRow = SqliteRow;

pub async fn run_server() -> Result<(), ServerError> {
    DataSourceType::Sqlite.init_pool().await?;
    setup_logger().await?;
    start_protocol_driver().await?;
    Server::new(ServerConfig::from("0.0.0.0:3121"))
        .start()
        .await?;
    Ok(())
}

async fn start_protocol_driver() -> Result<(), ServerError> {
    let mut config = HashMap::new();
    config.insert("host", "mqtt.diweiyunlian.cn");
    config.insert("port", "2840");
    config.insert("username", "xiaohuoshuan");
    config.insert("password", "xiaohuoshuan");
    config_protocol(config).await?;
    Ok(())
}

// 初始化日志
async fn setup_logger() -> Result<(), ServerError> {
    env::set_var("RUST_LOG", "debug");
    // 设置用户缓存
    tracing_subscriber::fmt::init();
    Ok(())
}
