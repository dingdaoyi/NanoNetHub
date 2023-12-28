use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Unit {
    pub id: i32,
    pub unit: String,
    pub unit_name: String,
    pub unit_description: String,
}