use axum::{Json, Router};
use axum::extract::{Path, Query};
use axum::routing::{delete, get, post};
use crate::config::database::get_conn;
use crate::models::{Property, R, ServerError};
use crate::models::tls::property::{CreateProperty, PropertyQuery};
use crate::models::tls::TlsSequence;
use crate::server::handler::base::Controller;
use crate::server::handler::product_handler::ProductHandler;

#[derive(Default)]
pub struct PropertyHandler;

impl PropertyHandler {
    pub(crate) async fn list_by_ids(ids: Vec<i32>) -> Vec<Property> {
        let sql = "select * from tb_property where property_id in (?::int4[])";
        // 使用 `query` 方法进行动态拼接 SQL
        let mut query = sqlx::query_as(sql);
        // 迭代 `ids` 并逐个绑定参数
        for id in ids {
            query = query.bind(id);
        }
        query
            .fetch_all(&get_conn())
            .await
            .unwrap_or(vec![])
    }

    ///删除属性
    async fn delete_property(
        Path(property_id): Path<i32>,
    ) -> Result<Json<R<String>>, ServerError> {
        sqlx::query(
            "delete  from tb_property where property_id = ?",
        )
            .bind(property_id)
            .execute(&get_conn())
            .await?;
        Ok(Json(R::success()))
    }

    ///创建属性
    async fn create_property(
        Json(CreateProperty { product_id, identifier, property_name, description, data_schema }): Json<CreateProperty>,
    ) -> Result<Json<R<String>>, ServerError> {
        let exists = ProductHandler::exists(product_id).await?;
        if !exists {
            return Ok(Json(R::bad_request("产品不存在".to_string())));
        }

        sqlx::query(
            "INSERT INTO tb_property ( property_id,product_id,identifier , property_name, description, data_schema) VALUES (?, ?, ?, ?,?,?)",
        )
            .bind(TlsSequence::property(product_id).next().await?)
            .bind(product_id)
            .bind(identifier)
            .bind(property_name)
            .bind(description)
            .bind(data_schema)
            .execute(&get_conn())
            .await?;
        Ok(Json(R::success()))
    }

    ///根据产品查看属性列表
    async fn property_list(
        Query(PropertyQuery { product_id, search_param }): Query<PropertyQuery>,
    ) -> Result<Json<R<Vec<Property>>>, ServerError> {
        let sql = "select * from  tb_property where product_id = ?";
        let sql = if let Some(search_param) = search_param {
            format!("{} and ( identifier = '{}' or property_name like '%{}%')", sql, search_param, search_param)
        } else {
            sql.to_string()
        };
        let res = sqlx::query_as::<_, Property>(&sql)
            .bind(product_id)
            .fetch_all(&get_conn())
            .await?;
        Ok(Json(R::success_with_data(res)))
    }
}

impl Controller for PropertyHandler {
    fn router(&self) -> Router {
        Router::new()
            .route("/property", post(Self::create_property))
            .route("/property", get(Self::property_list))
            .route("/property/:property_id", delete(Self::delete_property))
    }
}
