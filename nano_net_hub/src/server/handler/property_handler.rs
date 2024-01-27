use axum::{Json, Router};
use axum::extract::{Path, Query};
use axum::routing::{delete, get, post};
use sqlx::Error;
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
        let sql = format!("select * from tb_property where property_id in  ({})",
                          ids.iter().map(|_| "?").collect::<Vec<_>>().join(", "));
        let mut query = sqlx::query_as(&sql);
        for id in ids {
            query = query.bind(id);
        }
        let res = query
            .fetch_all(&get_conn())
            .await;
        match res {
            Ok(data) => {
                data
            }
            Err(msg) => {
                tracing::error!("查询报错{}",msg);
                vec![]
            }
        }
    }
    pub async fn exist_icon(icon: &str) -> bool {
        let res: i32 = sqlx::query_scalar("select count(*) from tb_property where icon = ?")
            .bind(icon)
            .fetch_one(&get_conn()).await.unwrap_or(0);
        res > 0
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
        Json(CreateProperty { product_id, identifier, property_name, description, data_schema, icon, property_type }): Json<CreateProperty>,
    ) -> Result<Json<R<String>>, ServerError> {
        let exists = ProductHandler::exists(product_id).await?;
        if !exists {
            return Ok(Json(R::bad_request("产品不存在".to_string())));
        }

        sqlx::query(
            "INSERT INTO tb_property ( property_id,product_id,identifier , property_name, description, data_schema,icon,property_type) VALUES (?, ?, ?, ?,?,?,?,?)",
        )
            .bind(TlsSequence::property(product_id).next().await?)
            .bind(product_id)
            .bind(identifier)
            .bind(property_name)
            .bind(description)
            .bind(data_schema)
            .bind(icon)
            .bind(property_type)
            .execute(&get_conn())
            .await?;
        Ok(Json(R::success()))
    }


    ///创建属性
    async fn update_property(
        Json(Property { property_id, product_id, identifier, property_name, description, data_schema, icon, property_type }): Json<Property>,
    ) -> Result<Json<R<String>>, ServerError> {
        let exists = ProductHandler::exists(product_id).await?;
        if !exists {
            return Ok(Json(R::bad_request("产品不存在".to_string())));
        }

        let res = sqlx::query(r#"
         update tb_property set product_id = ?,
            identifier = ?,
            property_name = ?,
            description = ?,
            data_schema = ?,
            icon = ?,
            property_type = ?
            where property_id = ?
        "#)
            .bind(product_id)
            .bind(identifier)
            .bind(property_name)
            .bind(description)
            .bind(data_schema)
            .bind(icon)
            .bind(property_type)
            .bind(property_id)
            .execute(&get_conn())
            .await?;
        if res.rows_affected() > 0 {
            return Ok(Json(R::success()));
        }
        Ok(Json(R::bad_request("修改失败".into())))
    }


    ///根据产品查看属性列表
    pub async fn property_list(
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
            .await;
        match res {
            Ok(res) => Ok(Json(R::success_with_data(res))),
            Err(error) => {
                tracing::error!("查询属性列表报错{}",error);
                Err(ServerError::from(error))
            }
        }
    }
}

impl Controller for PropertyHandler {
    fn router(&self) -> Router {
        Router::new()
            .route("/property", post(Self::create_property)
                .put(Self::update_property)
                .get(get(Self::property_list)))
            .route("/property/:property_id", delete(Self::delete_property))
    }
}
