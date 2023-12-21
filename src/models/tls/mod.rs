use crate::config::database::get_conn;
use crate::models::ServerError;

pub mod product;
pub mod property;
pub mod service;
pub mod unit;

pub struct TlsSequence {
    product_id: i32,
    sequence_type: SequenceType,
}

enum SequenceType {
    Service,
    Property,
}

impl SequenceType {
    pub fn fields(&self) -> (&str, &str) {
        match self {
            SequenceType::Service => ("tb_service", "service_id"),
            SequenceType::Property => ("tb_property", "property_id"),
        }
    }
}

impl TlsSequence {
    fn new(product_id: i32, sequence_type: SequenceType) -> Self {
        Self {
            product_id,
            sequence_type,
        }
    }
    pub fn service(product_id: i32) -> Self {
        Self::new(product_id, SequenceType::Service)
    }
    pub fn property(product_id: i32) -> Self {
        Self::new(product_id, SequenceType::Property)
    }
    pub async fn next(&self) -> Result<i32, ServerError> {
        let (table, field) = self.sequence_type.fields();
        let sql = format!("select {} from {} where product_id = ? order by {} desc limit 1", field, table, field);
        let res = sqlx::query_scalar::<_, i32>(&sql)
            .bind(self.product_id)
            .fetch_optional(&get_conn())
            .await?;
        let sequence = res.unwrap_or(0) + 1;
        Ok(sequence)
    }
}