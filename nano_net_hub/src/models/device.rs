use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use driver_common::device_service::EventData;
use crate::config::option_serialize::deserialize_option_string;
use crate::models::common::page::PaginationRequest;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Device {
    pub id: i32,
    pub device_code: String,
    pub product_id: i32,
    pub parent_id: Option<i32>,
    pub device_name: Option<String>,
    /// 设备原数据
    pub device_info: Option<Json<HashMap<String, String>>>,
}


#[derive(Debug, Deserialize)]
pub struct CreateDevice {
    pub device_code: String,
    pub product_id: i32,
    pub parent_id: Option<i32>,
    pub device_name: Option<String>,
    pub device_info: Option<Json<HashMap<String, String>>>,
}


#[derive(Debug, Deserialize)]
pub struct DeviceQuery {
    pub base_query: PaginationRequest,
    #[serde(deserialize_with = "deserialize_option_string")]
    pub device_code: Option<String>,
    pub product_id: Option<i32>,
    pub parent_id: Option<i32>,
    #[serde(deserialize_with = "deserialize_option_string")]
    pub device_name: Option<String>,
}
