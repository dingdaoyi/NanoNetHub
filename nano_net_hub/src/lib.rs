use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

use crate::config::database::DataSourceType;
use crate::models::ServerError;
use crate::protocol::config_protocol;
use crate::server::{Server, ServerConfig};
use sqlx::sqlite::SqliteRow;
use sqlx::{Pool, Sqlite};
use toml::Value;

mod config;
pub mod models;
mod protocol;
mod server;
mod data;

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
    let mut file = File::open("config.toml").expect("Unable to open the file");
    let mut toml_str = String::new();
    file.read_to_string(&mut toml_str)
        .expect("Unable to read the file");
    // 解析配置文件内容
    let toml_value: Value = toml::from_str(&toml_str).expect("Unable to parse TOML");
    let host = toml_value["mqtt"]["host"].as_str().unwrap();
    let port = toml_value["mqtt"]["port"].as_str().unwrap();
    let username = toml_value["mqtt"]["username"].as_str();
    let password = toml_value["mqtt"]["password"].as_str();
    let mut config = HashMap::new();
    config.insert("host", host);
    config.insert("port", port);
    if let Some(username) = username {
        config.insert("username", username);
    }
    if let Some(password) = password {
        config.insert("password", password);
    }
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
