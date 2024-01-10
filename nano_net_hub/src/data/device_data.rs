use async_trait::async_trait;
use serde::Serialize;
use sqlx::FromRow;
use sqlx::types::Json;
use driver_common::Value;
use crate::models::ServerError;

pub struct TsData {
    pub timestamp: i64,
    pub value: Value,
    pub device_id: i32,
    pub unit: Option<String>,
    pub unit_name: Option<String>,
}

pub struct TsQuery {
    pub timestamp_start: i64,
    pub timestamp_end: i64,
    pub device_id: i32,
}


#[derive(Debug, FromRow, Serialize)]
pub struct TsdbResult {
    pub timestamp: i64,
    pub value: Json<Value>,
}

#[async_trait]
pub trait Tsdb {
    /// 插入数据
    async fn insert(&self, data: TsData) -> Result<(), ServerError>;

    async fn query(&self, query: TsQuery) -> Result<Vec<TsdbResult>, ServerError>;
}
