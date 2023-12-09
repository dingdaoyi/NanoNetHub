use std::env;

use sqlx::{Pool, Sqlite};
use crate::config::database::DataSourceType;
use crate::models::ServerError;
use crate::server::{Server, ServerConfig};

pub mod models;
mod config;
mod server;

// config sqlite db
pub type SqlPool = Pool<Sqlite>;

pub async fn run_server() -> Result<(), ServerError> {
    DataSourceType::Sqlite.init_pool().await?;
    setup_logger().await?;
    Server::new(ServerConfig::from("0.0.0.0:3121")).start().await?;
    Ok(())
}

// 初始化日志
async fn setup_logger() -> Result<(), ServerError> {
    env::set_var("RUST_LOG", "debug");
    // 设置用户缓存
    tracing_subscriber::fmt::init();
    Ok(())
}