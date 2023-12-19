use axum::{Json, Router};
use axum::extract::{Path, Query};
use axum::routing::{delete, get, post};
use crate::config::database::get_conn;
use crate::models::{Property, R, ServerError};
use crate::models::tls::property::{CreateProperty, PropertyQuery};
use crate::server::handler::base::Controller;

#[derive(Default)]
pub struct PropertyHandler;

impl Controller for PropertyHandler {
    fn router(&self) -> Router {
        Router::new()
            .route("/property", post(Self::create_property))
            .route("/property", get(Self::property_list))
            .route("/property/:property_id", delete(Self::delete_property))
    }
}

impl PropertyHandler {
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
        sqlx::query(
            "INSERT INTO tb_property ( product_id,identifier , property_name, description, data_schema) VALUES (?, ?, ?, ?,?)",
        )
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