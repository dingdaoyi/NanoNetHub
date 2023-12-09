
#[derive(sqlx::FromRow, Debug)]
pub struct Service {
    pub service_id: i32,
    pub product_id: i32,
    pub identifier: String,
    pub service_name: String,
    pub service_type: String,
    pub description: String,
    pub properties: String,
}

#[derive(sqlx::FromRow, Debug)]
pub struct ServiceProperty {
    pub service_id: i32,
    pub property_id: i32,
    pub serial: i32,
}