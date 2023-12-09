use std::env;
use std::sync::OnceLock;
use sqlx::SqlitePool;
use tracing::info;
use crate::models::ServerError;
use crate::SqlPool;


static POOLS: OnceLock<SqlPool> = OnceLock::new();


/// 初始化连接
pub async fn init_sqlite_pool() -> Result<(), ServerError> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap();
    info!("建立数据连接: {}", database_url);
    let pool = SqlitePool::connect(&database_url)
        .await
        .map_err(|_| ServerError::Message("连接数据库错误".into()))?;
    POOLS
        .set(pool)
        .map_err(|_| ServerError::Message("重复设置数据库连接".to_string()))
}


/// 获取连接
pub fn get_conn() -> SqlPool {
    POOLS.get().unwrap().clone()
}

pub enum DataSourceType {
    _MYSQL,
    _POSTGRESQL,
    Sqlite,
}

impl DataSourceType {
    pub async fn init_pool(&self) -> Result<(), ServerError> {
        match self {
            DataSourceType::Sqlite => init_sqlite_pool().await,
            _ => {}
        }
    }
}