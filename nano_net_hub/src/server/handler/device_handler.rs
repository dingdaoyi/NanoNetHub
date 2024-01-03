use axum::{Json, Router};
use axum::extract::Path;
use axum::routing::{get, post};
use driver_common::device_service::{CommandParam, CommandResponse};
use crate::config::database::get_conn;
use crate::models::{PaginationResponse, R, ServerError};
use crate::models::common::sqlx_page::{Condition, PageSqlBuilder};
use crate::models::device::{CreateDevice, Device, DeviceQuery};
use crate::protocol::device_service_command;
use crate::server::handler::base::Controller;
use crate::server::handler::product_handler::ProductHandler;

#[derive(Default)]
pub struct DeviceHandler;

impl Controller for DeviceHandler {
    fn router(&self) -> Router {
        Router::new()
            .route("/device", post(Self::create_device)
                .put(Self::update_device))
            .route("/device/page", post(Self::device_page))
            .route("/device/:id", post(Self::delete_device)
                .get(Self::details))
            .route("/device/command", get(Self::command))
    }
}

impl DeviceHandler {
    ///添加设备
    async fn create_device(
        Json(device): Json<CreateDevice>,
    ) -> Result<Json<R<bool>>, ServerError> {
        let exists = ProductHandler::exists(device.product_id).await?;
        if !exists {
            return Ok(Json(R::bad_request("产品不存在".to_string())));
        }
        let res = sqlx::query(
            "INSERT INTO tb_device ( product_id,parent_id,device_code,device_name,device_info) VALUES (?, ?, ?, ?,?)",
        )
            .bind(device.product_id)
            .bind(device.parent_id)
            .bind(device.device_code)
            .bind(device.device_name)
            .bind(device.device_info)
            .execute(&get_conn())
            .await?;
        Ok(Json(R::success_with_data(res.rows_affected() > 0)))
    }

    ///添加设备
    async fn update_device(
        Json(device): Json<Device>,
    ) -> Result<Json<R<bool>>, ServerError> {
        let exists = Self::exists(device.id).await?;
        if !exists {
            return Ok(Json(R::bad_request("设备不存在".to_string())));
        }
        let res = sqlx::query(
            "update tb_device set device_code=?,device_name=?,device_info=? where id=?",
        )
            .bind(device.device_code)
            .bind(device.device_name)
            .bind(device.device_info)
            .bind(device.id)
            .execute(&get_conn())
            .await?;
        Ok(Json(R::success_with_data(res.rows_affected() > 0)))
    }

    async fn exists(id: i32) -> Result<bool, ServerError> {
        let res = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT * FROM tb_device WHERE id = ?)")
            .bind(id)
            .fetch_one(&get_conn())
            .await?;
        Ok(res)
    }

    async fn command() -> Result<Json<CommandResponse>, ServerError> {
        let mut command = CommandParam::default();
        command.device_code = "123".into();
        command.product_key = "aaa".into();
        let res = device_service_command("mqtt", command).await?;
        Ok(Json(res))
    }

    ///设备详情
    async fn details(Path(id): Path<i32>) -> Result<Json<R<Device>>, ServerError> {
        let res = sqlx::query_as::<_, Device>("SELECT * FROM tb_device WHERE id = ?")
            .bind(id)
            .fetch_one(&get_conn())
            .await?;
        Ok(Json(R::success_with_data(res)))
    }

    ///删除设备
    async fn delete_device(
        Path(id): Path<i32>,
    ) -> Result<Json<R<bool>>, ServerError> {
        let res = sqlx::query("delete  from tb_device where id = ?")
            .bind(id)
            .execute(&get_conn())
            .await?;
        Ok(Json(R::success_with_data(res.rows_affected() > 0)))
    }

    ///设备分页查询
    async fn device_page(
        Json(DeviceQuery { base_query, device_code, product_id, parent_id, device_name }): Json<DeviceQuery>,
    ) -> Result<Json<R<PaginationResponse<Device>>>, ServerError> {
        let mut builder = PageSqlBuilder::builder("tb_device", &base_query);
        if let Some(product_id) = product_id {
            builder = builder.condition(Condition::Equal("product_id", product_id.into()));
        }
        if let Some(parent_id) = parent_id {
            builder = builder.condition(Condition::Equal("parent_id", parent_id.into()));
        }
        if let Some(device_code) = device_code {
            builder = builder.condition(Condition::Equal("device_code", device_code.into()));
        }
        if let Some(device_name) = device_name {
            builder = builder.condition(Condition::Like("device_name", device_name.into()));
        }
        builder.build().execute()
            .await
            .map(|value|
                Json(R::success_with_data(value)))
    }
}