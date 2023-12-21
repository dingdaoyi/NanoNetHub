use crate::config::database::get_conn;
use crate::models::tls::service::{CreateService, ServiceQuery};
use crate::models::tls::TlsSequence;
use crate::models::{ServerError, Service, R};
use crate::server::handler::base::Controller;
use crate::server::handler::product_handler::ProductHandler;
use axum::extract::{Path, Query};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use sqlx::{Acquire, Executor};

#[derive(Default)]
pub struct ServiceHandler;

impl Controller for ServiceHandler {
    fn router(&self) -> Router {
        Router::new()
            .route("/service", post(Self::create_service))
            .route(
                "/service/:service_id",
                delete(Self::delete_service).put(Self::update_service),
            )
            .route("/service/list", post(Self::list_service))
    }
}

impl ServiceHandler {
    ///删除服务
    async fn delete_service(Path(service_id): Path<i32>) -> Result<Json<R<String>>, ServerError> {
        sqlx::query("delete  from tb_service where service_id = ?")
            .bind(service_id)
            .execute(&get_conn())
            .await?;
        Ok(Json(R::success()))
    }

    ///创建服务
    async fn create_service(
        Json(service): Json<CreateService>,
    ) -> Result<Json<R<String>>, ServerError> {
        let exists = ProductHandler::exists(service.product_id).await?;
        if !exists {
            return Ok(Json(R::fail("产品不存在".to_string())));
        }
        let service_id = TlsSequence::service(service.product_id).next().await?;
        let mut conn = get_conn().acquire().await?;
        let mut transaction = conn.begin().await?;
        let res = transaction.execute(sqlx::query(
            "INSERT INTO tb_service (service_id, product_id,identifier , service_name, service_type, description,  properties,command_id) VALUES (?, ?, ?, ?,?,?,?,?)",
        )
            .bind(service_id)
            .bind(service.product_id)
            .bind(service.identifier)
            .bind(service.service_name)
            .bind(service.service_type)
            .bind(service.description)
            .bind(&service.properties)
            .bind(service.command_id))
            .await?;
        if res.rows_affected() < 1 {
            transaction.rollback().await?;
            return Ok(Json(R::fail("创建失败".to_string())));
        }
        // 添加关联关系,用于查询
        for property in service.properties.iter() {
            transaction.execute(sqlx::query(
                "INSERT INTO tb_service_property (service_id, property_id,product_id, serial) VALUES (?, ?, ?, ?)",
            )
                .bind(service_id)
                .bind(property.property_id)
                .bind(service.product_id)
                .bind(property.serial)
            ).await?;
        }
        transaction.commit().await?;
        Ok(Json(R::success()))
    }
    ///服务列表
    async fn list_service(
        Json(ServiceQuery {
                 product_id,
                 service_types,
                 search_param,
             }): Json<ServiceQuery>,
    ) -> Result<Json<R<Vec<Service>>>, ServerError> {
        let mut sql = "select * from  tb_service where product_id = ?".to_string();
        if let Some(search_param) = search_param {
            sql = format!(
                "{} and ( identifier = '{}' or service_name like '%{}%')",
                sql, search_param, search_param
            )
        }
        if !service_types.is_empty() {
            sql = format!(
                "{} and service_type in ({})",
                sql,
                service_types
                    .iter()
                    .map(|s| format!("'{}'", s))
                    .collect::<Vec<String>>()
                    .join(",")
            )
        }
        let data = sqlx::query_as::<_, Service>(sql.as_str())
            .bind(product_id)
            .fetch_all(&get_conn())
            .await?;
        Ok(Json(R::success_with_data(data)))
    }

    async fn update_service(Json(service): Json<Service>) -> Result<Json<R<String>>, ServerError> {
        let mut conn = get_conn().acquire().await?;
        let mut transaction = conn.begin().await?;
        let res = transaction
            .execute(
                sqlx::query(
                    r#"
            update tb_service set identifier=?,
             service_name =?,
             service_type =?,
             description =?,
             properties =?,
             command_id =?
             where service_id=?
            "#,
                )
                    .bind(service.identifier)
                    .bind(service.service_name)
                    .bind(service.service_type)
                    .bind(service.description)
                    .bind(&service.properties)
                    .bind(service.command_id)
                    .bind(service.service_id)
            )
            .await?;
        if res.rows_affected() < 1 {
            transaction.rollback().await?;
            return Ok(Json(R::fail("修改失败".to_string())));
        }
        // 删除关联关系
        transaction
            .execute(sqlx::query(
                "delete from tb_service_property where service_id = ?",
            )
                .bind(service.service_id))
            .await?;

        // 添加关联关系,用于查询
        for property in service.properties.iter() {
            transaction.execute(sqlx::query(
                "INSERT INTO tb_service_property (service_id, property_id,product_id, serial) VALUES (?, ?, ?, ?)",
            )
                .bind(service.service_id)
                .bind(property.property_id)
                .bind(service.product_id)
                .bind(property.serial)
            ).await?;
        }
        transaction.commit().await?;

        Ok(Json(R::success()))
    }
}
