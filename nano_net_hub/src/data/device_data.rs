use std::collections::HashMap;
use async_trait::async_trait;
use serde::Serialize;
use sqlx::FromRow;
use sqlx::types::Json;
use driver_common::Value;
use crate::models::ServerError;

pub struct TsData {
    pub create_time: chrono::NaiveDateTime,
    pub value: Value,
    pub device_id: i32,
    pub unit: Option<String>,
    pub unit_name: Option<String>,
}

pub struct TsQuery {
    pub timestamp_start: chrono::NaiveDateTime,
    pub timestamp_end: chrono::NaiveDateTime,
    pub device_id: i32,
    pub identifier: Option<String>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct TsdbLastResult {
    pub create_time: chrono::NaiveDateTime,
    pub identifier: String,
    pub value: Json<Value>,
    pub unit: String,
    pub unit_name: String,
}

impl TsdbLastResult {
    pub fn into_tsdb_result(self) -> (String, TsdbResult) {
        (self.identifier, TsdbResult {
            create_time: self.create_time,
            value: self.value,
            unit: Some(self.unit),
            unit_name: Some(self.unit_name),
        })
    }
}

#[derive(Debug, FromRow, Serialize)]
pub struct TsdbResult {
    pub create_time: chrono::NaiveDateTime,
    pub value: Json<Value>,
    pub unit: Option<String>,
    pub unit_name: Option<String>,
}

#[async_trait]
pub trait Tsdb {
    /// 插入数据
    async fn insert(&self, data: TsData) -> Result<(), ServerError>;

    async fn query(&self, query: TsQuery) -> Result<Vec<TsdbResult>, ServerError>;

    async fn query_last(&self, device_id: i32) -> Result<HashMap<String, TsdbResult>, ServerError>;
}
