use axum::{Json, Router};
use axum::extract::{Path, Query};
use axum::routing::{get, post};
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use driver_common::device_service::{CommandParam, CommandResponse};
use driver_common::Value;
use crate::config::database::get_conn;
use crate::data::device_data::{TsdbResult, TsQuery};
use crate::data::get_tsdb;
use crate::models::{PaginationResponse, R, ServerError};
use crate::models::common::sqlx_page::{Condition, PageSqlBuilder};
use crate::models::device::{CreateDevice, Device, DeviceQuery};
use crate::models::tls::property::PropertyQuery;
use crate::protocol::device_service_command;
use crate::server::handler::base::Controller;
use crate::server::handler::product_handler::ProductHandler;
use crate::server::handler::property_handler::PropertyHandler;
use crate::config::date_format;

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
            .route("/device/shadows/:device_id", get(Self::device_shadows))
            .route("/device/command", get(Self::command))
            .route("/device/logs", post(Self::device_logs))
    }
}

impl DeviceHandler {
    pub async fn get_by_device(device_code: &str, product_id: i32) -> Option<Device> {
        sqlx::query_as::<_, Device>("select *  from tb_device where device_code = ? and product_id = ?")
            .bind(device_code)
            .bind(product_id)
            .fetch_optional(&get_conn())
            .await
            .unwrap_or(None)
    }

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


    ///设备分页查询
    async fn device_shadows(device_id: Path<i32>) -> Result<Json<R<Vec<DeviceShadow>>>, ServerError> {
        let Json(R { data, .. }) = Self::details(device_id).await?;
        match data {
            None => {
                return Ok(Json(R::bad_request("设备不存在".to_string())));
            }
            Some(device) => {
                let Json(R { data, .. }) = PropertyHandler::property_list(Query(PropertyQuery { product_id: device.product_id, search_param: None })).await?;
                tracing::debug!("设备属性列表{:?}", data);
                match data {
                    None => {
                        Ok(Json(R::success_with_data(vec![])))
                    }
                    Some(property) => {
                        let values = get_tsdb().query_last(device.id).await?;
                        let mut shadows = vec![];
                        for property in property {
                            let data_type = property.get_type();
                            if let Some(result) = values.get(&property.identifier) {
                                let value = result.value.0.clone();
                                let shadow = DeviceShadow {
                                    device_code: device.device_code.clone(),
                                    property_name: property.property_name,
                                    description: property.description,
                                    identifier: property.identifier,
                                    value,
                                    unit: result.unit.clone().unwrap_or("".into()),
                                    icon: property.icon,
                                    unit_name: result.unit_name.clone().unwrap_or("".into()),
                                    data_type,
                                };
                                shadows.push(shadow);
                            }
                        }
                        Ok(Json(R::success_with_data(shadows)))
                    }
                }
            }
        }
    }


    ///设备分页查询
    async fn device_logs(Json(query): Json<DeviceLogQuery>) -> Result<Json<R<Vec<TsdbResult>>>, ServerError> {
        let values = get_tsdb().query(query.into()).await?;
        Ok(Json(R::success_with_data(values)))
    }
}

#[derive(Debug, Serialize)]
struct DeviceShadow {
    device_code: String,
    property_name: String,
    description: Option<String>,
    identifier: String,
    value: Value,
    unit: String,
    icon: Option<String>,
    unit_name: String,
    data_type: u16,
}

#[derive(Deserialize, Debug)]
struct DeviceLogQuery {
    device_id: i32,
    identifier: String,
    #[serde(with = "date_format")]
    timestamp_start: Option<chrono::NaiveDateTime>,
    #[serde(with = "date_format")]
    timestamp_end: Option<chrono::NaiveDateTime>,
}

impl From<DeviceLogQuery> for TsQuery {
    fn from(query: DeviceLogQuery) -> Self {
        Self {
            timestamp_start: query.timestamp_start.unwrap_or(Local::now().naive_local()),
            timestamp_end: query.timestamp_end.unwrap_or(Local::now().naive_local()),
            device_id: query.device_id,
            identifier: Some(query.identifier),
        }
    }
}