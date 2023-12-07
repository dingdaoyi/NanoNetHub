/// 属性
#[derive(sqlx::FromRow, Debug)]
pub struct Property {
    pub property_id: i32,
    pub identifier: String,
    pub property_name: String,
    pub description: String,
    pub data_type: String,
    pub data_schema: String,
}
