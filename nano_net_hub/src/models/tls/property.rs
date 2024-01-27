use serde::{Deserialize, Serialize};
use sqlx::Type;
use sqlx::types::Json;
use crate::config::option_serialize::deserialize_option_string;

// 指标类型
#[derive(Debug, Serialize, Deserialize, Type)]
pub enum PropertyType {
    #[serde(rename = "Property")]
    Property,
    #[serde(rename = "Tag")]
    Tag,
    #[serde(rename = "Param")]
    Param,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct CreateProperty {
    pub product_id: i32,
    pub identifier: String,
    pub property_name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub property_type: PropertyType,
    pub data_schema: Json<DataSchema>,
}

#[derive(Debug, Deserialize)]
pub struct PropertyQuery {
    pub product_id: i32,
    #[serde(deserialize_with = "deserialize_option_string")]
    pub search_param: Option<String>,
}

/// 属性
#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Property {
    pub property_id: i32,
    pub product_id: i32,
    pub identifier: String,
    pub property_name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub property_type: PropertyType,
    pub data_schema: Json<DataSchema>,
}

impl Property {
    pub fn get_type(&self) -> u16 {
        self.data_schema.0.get_type()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Enum {
    pub key: i32,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DataSchema {
    #[serde(rename = "Integer")]
    Integer {
        len: usize,
        unit: String,
        min: i64,
        max: i64,
        unit_name: String,
    },
    #[serde(rename = "String")]
    String {
        unit: String,
        unit_name: String,
    },
    #[serde(rename = "VaryString")]
    VaryString {
        len: usize,
        unit: String,
        unit_name: String,
    },
    #[serde(rename = "Boolean")]
    Boolean {
        bool_false: String,
        bool_true: String,
    },
    #[serde(rename = "DateTime")]
    DateTime,
    #[serde(rename = "Enum")]
    Enum(Vec<Enum>),
    #[serde(rename = "Double")]
    Double {
        unit: String,
        min: f64,
        max: f64,
        unit_name: String,
    },
}

impl DataSchema {
    pub fn get_type(&self) -> u16 {
        match self {
            DataSchema::Integer { .. } => 1,
            DataSchema::String { .. } => 2,
            DataSchema::VaryString { .. } => 3,
            DataSchema::Boolean { .. } => 4,
            DataSchema::DateTime => 5,
            DataSchema::Enum(_) => 6,
            DataSchema::Double { .. } => 7,
        }
    }
}
