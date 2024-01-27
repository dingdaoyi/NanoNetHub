use std::collections::HashMap;
use async_trait::async_trait;
use sqlx::types::Json;
use crate::data::device_data::{TsData, Tsdb, TsdbLastResult, TsdbResult, TsQuery};
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
        sqlx::query("INSERT INTO tb_device_data (create_time, value, device_id, unit, unit_name) VALUES (?, ?, ?, ?, ?)")
            .bind(data.create_time)
            .bind(Json(data.value))
            .bind(data.device_id)
            .bind(data.unit.unwrap_or("".into()))
            .bind(data.unit_name.unwrap_or("".into()))
            .execute(&self.conn)
            .await?;
        Ok(())
    }

    async fn query(&self, TsQuery { timestamp_start, timestamp_end, device_id, identifier }: TsQuery) -> Result<Vec<TsdbResult>, ServerError> {
        let mut sql = String::from("select create_time,unit,unit_name,value from tb_device_data where device_id=? and  create_time>=? and create_time<=?  ");
        if let Some(identifier) = identifier {
            sql = format!("{} and identifier= '{}' ", sql, identifier);
        }
        let result = sqlx::query_as::<_, TsdbResult>(&sql)
            .bind(device_id)
            .bind(timestamp_start)
            .bind(timestamp_end)
            .fetch_all(&self.conn)
            .await?;
        Ok(result)
    }

    async fn query_last(&self, device_id: i32) -> Result<HashMap<String, TsdbResult>, ServerError> {
        let result = sqlx::query_as::<_, TsdbLastResult>(r#"
                    SELECT create_time,identifier,value,unit,unit_name
                    FROM (
                             SELECT create_time,identifier,value,unit,unit_name,ROW_NUMBER()
                                     OVER (
                                        PARTITION BY device_id,identifier
                                        ORDER BY create_time DESC
                                        ) AS row_num
                             FROM tb_device_data where device_id=?
                         ) AS t
                    WHERE row_num = 1
        "#)
            .bind(device_id)
            .fetch_all(&self.conn)
            .await?;
        Ok(result.into_iter().map(|item| item.into_tsdb_result()).collect())
    }
}