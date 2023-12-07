
/// 产品
#[derive(sqlx::FromRow, Debug)]
pub struct Product {
    pub id: i32,
    pub tenant_id: i32,
    pub create_time: chrono::NaiveDateTime,
    pub deleted: i32,
    pub product_name: String,
    pub description: String,
}