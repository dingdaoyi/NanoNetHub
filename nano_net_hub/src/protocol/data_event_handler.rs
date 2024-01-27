use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::time::SystemTime;
use chrono::{Local, NaiveDateTime};
use driver_common::device_service::{EventData, ProtocolData};
use driver_common::Value;
use crate::data::device_data::TsData;
use crate::data::get_tsdb;
use crate::models::device::Device;
use crate::models::Property;
use crate::models::tls::property::DataSchema;
use crate::server::handler::device_handler::DeviceHandler;
use crate::server::handler::product_handler::ProductHandler;
use crate::server::handler::property_handler::PropertyHandler;
use crate::server::handler::service_handler::ServiceHandler;

pub(super) fn handle_event(rx: Receiver<EventData>) {
    tokio::task::spawn(async move {
        loop {
            let result = rx.recv();
            match result {
                Ok(event_data) => handle_event_data(event_data).await,
                Err(msg) => {
                    tracing::error!("接收数据错误:{}",msg);
                }
            }
        }
    });
}

async fn handle_event_data(event_data: EventData) {
    tracing::debug!("接收到数据:{:?}",event_data);
    if let Some(product) = ProductHandler::product_by_product_key(event_data.product_key).await {
        if let Some(device) = DeviceHandler::get_by_device(&event_data.device_code, product.id).await {
            if let Some(service) = ServiceHandler::service_identifier(&event_data.identifier, product.id).await {
                tracing::debug!("service:{:?}",service);
                let properties = service.properties.0;
                let properties = PropertyHandler::list_by_ids(properties.into_iter()
                    .map(|p| p.property_id).collect()).await;
                let properties_map = properties.into_iter()
                    .map(|p| (p.identifier.clone(), p))
                    .collect::<HashMap<String, Property>>();
                let params = event_data.params;
                for ProtocolData { identifier, value } in params.iter() {
                    if let Some(property) = properties_map.get(identifier) {
                        match &property.data_schema.0 {
                            DataSchema::Integer {
                                len,
                                unit,
                                min,
                                max,
                                unit_name
                            } => {
                                match value.clone().into_inner::<i32>() {
                                    None => {
                                        tracing::debug!("数据解析和服务定义不一致:{}",value);
                                    }
                                    Some(value) => {
                                        if value < *min as i32 {
                                            tracing::error!("低位告警:{}",value);
                                            continue;
                                        }
                                        if value > *max as i32 {
                                            tracing::error!("高告警:{}",value);
                                            continue;
                                        }
                                        save_property(device.id, Some(unit.clone()), Some(unit_name.clone()), value.into()).await;
                                    }
                                }
                            }
                            DataSchema::String { unit, unit_name } => {
                                save_property(device.id, Some(unit.clone()), Some(unit_name.clone()), value.clone()).await;
                            }
                            DataSchema::VaryString { unit_name, unit, len } => {
                                save_property(device.id, Some(unit.clone()), Some(unit_name.clone()), value.clone()).await;
                            }
                            DataSchema::Boolean { bool_false, bool_true } => {
                                save_property(device.id, None, None, value.clone()).await;
                            }
                            DataSchema::DateTime => {
                                save_property(device.id, None, None, value.clone()).await;
                            }
                            DataSchema::Enum(enums) => {
                                let key: Option<i32> = value.clone().into_inner();
                                if let Some(key) = key {
                                    let enum_value = enums.iter().find(|e| e.key == key);
                                    if let Some(enum_value) = enum_value {
                                        save_property(device.id, None, None, enum_value.value.clone().into()).await;
                                    }
                                }
                            }
                            DataSchema::Double {
                                unit,
                                min,
                                max,
                                unit_name
                            } => {
                                match value.clone().into_inner::<f64>() {
                                    None => {
                                        tracing::debug!("数据解析和服务定义不一致:{}",value);
                                    }
                                    Some(value) => {
                                        if value < *min {
                                            tracing::error!("低位告警:{}",value);
                                            continue;
                                        }
                                        if value > *max {
                                            tracing::error!("高告警:{}",value);
                                            continue;
                                        }
                                        save_property(device.id, Some(unit.clone()), Some(unit_name.clone()), value.into()).await;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn save_property(device_id: i32, unit: Option<String>, unit_name: Option<String>, value: Value) {
    let tsdb = get_tsdb();
    let local_now = Local::now();

    // 将当前本地时间转换为 NaiveDateTime
    let naive_now: NaiveDateTime = local_now.naive_local();

    let res = tsdb.insert(TsData {
        create_time: naive_now,
        value,
        device_id,
        unit,
        unit_name,
    }).await;
    if let Err(error) = res {
        tracing::error!("保存数据错误:{}",error);
    }
}