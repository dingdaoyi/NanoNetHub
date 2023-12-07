use std::env;
use once_cell::sync::OnceCell;
use sqlx::{Pool, Sqlite, SqlitePool};
use crate::models::ServerError;

static POOLS: OnceCell<Pool<Sqlite>> = OnceCell::new();

/// 初始化连接
pub async fn init_database() -> Result<(), ServerError> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap();
    println!("建立数据连接: {}", database_url);
    let pool = SqlitePool::connect(&database_url)
        .await
        .map_err(|_| ServerError::Message("连接数据库错误".into()))?;
    POOLS
        .set(pool)
        .map_err(|_| ServerError::Message("重复设置数据库连接".to_string()))
}

/// 获取连接
pub fn get_conn() -> Pool<Sqlite> {
    POOLS.get().unwrap().clone()
}
