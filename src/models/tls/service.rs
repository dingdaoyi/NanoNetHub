use std::fmt::Display;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use sqlx::types::Json;
use crate::config::option_serialize::deserialize_option_string;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Service {
    pub service_id: i32,
    pub product_id: i32,
    pub identifier: String,
    pub service_name: String,
    pub service_type: ServiceType,
    pub description: String,
    pub properties: Json<Vec<PropertyRef>>,
    //只有响应才有
    pub command_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub enum ServiceType {
    #[serde(rename = "EventReport")]
    EventReport,
    #[serde(rename = "Command")]
    Command,
    // 参数是命令的id
    #[serde(rename = "CommandResponse")]
    CommandResponse,
}

impl Display for ServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceType::EventReport => write!(f, "EventReport"),
            ServiceType::Command => write!(f, "Command"),
            ServiceType::CommandResponse => write!(f, "CommandResponse"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PropertyRef {
    pub property_id: i32,
    pub serial: u16,
}

///创建服务信息
#[derive(sqlx::FromRow, Debug, Deserialize)]
pub struct CreateService {
    pub product_id: i32,
    pub identifier: String,
    pub service_name: String,
    pub service_type: ServiceType,
    pub description: String,
    pub properties: Json<Vec<PropertyRef>>,
    //只有响应才有
    pub command_id: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ServiceQuery {
    pub product_id: i32,
    pub service_types: Vec<ServiceType>,
    #[serde(deserialize_with = "deserialize_option_string")]
    pub search_param: Option<String>,
}


#[derive(sqlx::FromRow, Debug)]
pub struct ServiceProperty {
    pub service_id: i32,
    pub property_id: i32,
    pub serial: i32,
}

#[cfg(test)]
mod testing {
    use serde_json::json;
    use super::*;

    #[test]
    fn test_service_type() {
        let service = Service {
            service_id: 0,
            product_id: 0,
            identifier: "press".to_string(),
            service_name: "press".to_string(),
            service_type: ServiceType::CommandResponse,
            description: "press".to_string(),
            properties: Json(vec![PropertyRef {
                property_id: 20,
                serial: 2,
            }]),
            command_id: None,
        };
        let res = json!(service);
        println!("{}", res)
    }
}