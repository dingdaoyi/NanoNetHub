use axum::{Json, Router};
use axum::extract::Path;
use axum::routing::{get, post};
use crate::config::database::get_conn;
use crate::models::{Property, R, ServerError};
use crate::models::tls::property::CreateProperty;
use crate::server::handler::base::Controller;

#[derive(Default)]
pub struct PropertyHandler;

impl Controller for PropertyHandler {
    fn router(&self) -> Router {
        Router::new()
            .route("/property", post(Self::create_property))
            .route("/property/:product_id", get(Self::property_list))
    }
}

impl PropertyHandler {
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
        Path(product_id): Path<i32>,
    ) -> Result<Json<R<Vec<Property>>>, ServerError> {
        let res = sqlx::query_as::<_, Property>("select * from  tb_property where product_id = ?")
            .bind(product_id)
            .fetch_all(&get_conn())
            .await?;
        Ok(Json(R::success_with_data(res)))
    }
}