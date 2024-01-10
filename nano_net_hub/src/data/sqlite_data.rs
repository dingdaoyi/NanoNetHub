use async_trait::async_trait;
use sqlx::types::Json;
use crate::data::device_data::{TsData, Tsdb, TsdbResult, TsQuery};
use crate::models::ServerError;
use crate::SqlPool;

#[derive(Debug)]
pub(crate) struct SqlxTsdb {
    /// SQLite 连接
    conn: SqlPool,
}

impl SqlxTsdb {
    pub fn new(conn: SqlPool) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl Tsdb for SqlxTsdb {
    async fn insert(&self, data: TsData) -> Result<(), ServerError> {
        sqlx::query("INSERT INTO tb_device_data (timestamp, value, device_id, unit, unit_name) VALUES (?, ?, ?, ?, ?)")
            .bind(data.timestamp)
            .bind(Json(data.value))
            .bind(data.device_id)
            .bind(data.unit)
            .bind(data.unit_name)
            .execute(&self.conn)
            .await
            .unwrap();
        Ok(())
    }

    async fn query(&self, query: TsQuery) -> Result<Vec<TsdbResult>, ServerError> {
        let result = sqlx::query_as::<_, TsdbResult>("select timestamp,value from tb_device_data where timestamp>=? and timestamp<=? and device_id=?")
            .bind(query.timestamp_start)
            .bind(query.timestamp_end)
            .bind(query.device_id)
            .fetch_all(&self.conn)
            .await?;
        Ok(result)
    }
}