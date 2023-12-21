use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use crate::config::option_serialize::deserialize_option_string;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct CreateProperty {
    pub product_id: i32,
    pub identifier: String,
    pub property_name: String,
    pub description: Option<String>,
    pub data_schema: Json<DataSchema>,
}

#[derive(Debug, Deserialize)]
pub struct PropertyQuery {
    pub product_id: i32,
    #[serde(deserialize_with = "deserialize_option_string")]
    pub search_param: Option<String>,
}

/// 属性
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct Property {
    pub property_id: i32,
    pub product_id: i32,
    pub identifier: String,
    pub property_name: String,
    pub description: Option<String>,
    pub data_schema: Json<DataSchema>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Enum {
    key: i32,
    value: String,
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
    #[serde(rename = "Float")]
    Float {
        unit: String,
        min: f64,
        max: f64,
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


#[cfg(test)]
mod testing {
    use sqlx::types::Json;
    use crate::models::Property;
    use crate::models::tls::property::{CreateProperty, Enum};

    #[test]
    pub fn test_property_json() {
        use crate::models::tls::property::DataSchema;
        use serde_json::json;

        let data_type = DataSchema::Integer {
            len: 1,
            unit: "1".to_string(),
            min: 1,
            max: 1,
            unit_name: "1".to_string(),
        };
        let properties = Property {
            property_id: 0,
            product_id: 1,
            identifier: "1".to_string(),
            property_name: "1".to_string(),
            description: Option::from("1".to_string()),
            data_schema: Json(data_type),
        };

        let json = json!(properties);
        println!("{}", json);
    }

    #[test]
    pub fn test_bool_json() {
        use crate::models::tls::property::DataSchema;
        use serde_json::json;

        let properties = CreateProperty {
            product_id: 1,
            identifier: "1".to_string(),
            property_name: "1".to_string(),
            description: Option::from("1".to_string()),
            data_schema: Json(DataSchema::Boolean {
                bool_false: "错误".to_string(),
                bool_true: "正确".to_string(),
            }),

        };

        let json = json!(properties);
        println!("{}", json);
    }

    #[test]
    pub fn test_date_time_json() {
        use crate::models::tls::property::DataSchema;
        use serde_json::json;

        let properties = CreateProperty {
            product_id: 1,
            identifier: "1".to_string(),
            property_name: "1".to_string(),
            description: Option::from("1".to_string()),
            data_schema: Json(DataSchema::DateTime),
        };

        let json = json!(properties);
        println!("{}", json);
    }

    #[test]
    pub fn test_enum_json() {
        use crate::models::tls::property::DataSchema;
        use serde_json::json;

        let properties = CreateProperty {
            product_id: 1,
            identifier: "1".to_string(),
            property_name: "1".to_string(),
            description: Option::from("1".to_string()),
            data_schema: Json(DataSchema::Enum(vec![Enum {
                key: 1,
                value: "1".to_string(),
            }])),
        };
        let json = json!(properties);
        println!("{}", json);
    }
}