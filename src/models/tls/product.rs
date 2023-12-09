use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::config::date_format;
use crate::models::common::page::PaginationRequest;

/// 产品
#[derive(Debug, FromRow,Serialize)]
pub struct Product {
    pub id: i32,
    #[serde( with = "date_format")]
    pub create_time: Option<chrono::NaiveDateTime>,
    pub deleted: bool,
    pub product_name: String,
    pub description: Option<String>,
}

/// 产品
#[derive(Debug, Deserialize)]
pub struct CreateProduct {
    pub product_name: String,
    pub description: Option<String>,
}
/// 产品
#[derive(Debug, Deserialize)]
pub struct UpdateProduct {
    pub id: i32,
    pub product_name: String,
    pub description: Option<String>,
}


#[derive(Debug,Deserialize)]
pub struct ProductQuery{
    pub base_query:PaginationRequest,
    pub product_name: Option<String>,
}