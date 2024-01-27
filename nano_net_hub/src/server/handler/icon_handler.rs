use axum::extract::{Path, Query};
use axum::{Json, Router};
use axum::routing::{delete, get, post};
use serde::Deserialize;
use crate::config::database::get_conn;
use crate::models::{R, ServerError};
use crate::models::icon::{CreateIcon, Icon};
use crate::server::handler::base::Controller;
use crate::server::handler::file_handler::FileHandler;
use crate::server::handler::property_handler::PropertyHandler;

#[derive(Default)]
pub struct IconHandler;

impl IconHandler {
    ///保存
    async fn save_icon(Json(icon): Json<CreateIcon>) -> Result<Json<R<String>>, ServerError> {
        let res = sqlx::query("INSERT INTO tb_icon (icon,name,default_icon) VALUES (?,?,?)")
            .bind(icon.icon)
            .bind(icon.name)
            .bind(false)
            .execute(&get_conn()).await?;
        if res.rows_affected() > 0 {
            return Ok(Json(R::success()));
        }
        Err(ServerError::Message("添加失败".into()))
    }

    ///保存
    async fn update_icon(Json(icon): Json<Icon>) -> Result<Json<R<String>>, ServerError> {
        let res = sqlx::query("update tb_icon set icon = ?,name = ?,default_icon = ? where id = ?")
            .bind(icon.icon)
            .bind(icon.name)
            .bind(icon.default_icon)
            .bind(icon.id)
            .execute(&get_conn()).await?;
        if res.rows_affected() > 0 {
            return Ok(Json(R::success()));
        }
        Err(ServerError::Message("修改失败".into()))
    }
    ///删除图标
    async fn delete_icon(Path(id): Path<i32>) -> Result<Json<R<String>>, ServerError> {
        let icon = Self::details(id).await?;
        if let Some(icon) = icon {
            if icon.default_icon {
                return Ok(Json(R::bad_request("默认图标不能删除".into())));
            }
            if PropertyHandler::exist_icon(&icon.icon).await {
                return Ok(Json(R::bad_request("该图标已被使用".into())));
            }
            let res = sqlx::query("delete from tb_icon where id = ?")
                .bind(id)
                .execute(&get_conn()).await?;
            if res.rows_affected() > 0 {
                FileHandler::delete_file(&icon.icon).await?;
                return Ok(Json(R::success()));
            }
        };
        Err(ServerError::Message("图标不存在".into()))
    }
    /// 详情
    async fn details(id: i32) -> Result<Option<Icon>, ServerError> {
        let res = sqlx::query_as("select * from  tb_icon where  id = ?")
            .bind(id)
            .fetch_optional(&get_conn()).await?;
        Ok(res)
    }

    ///列表
    async fn list(Query(IconQuery { name }): Query<IconQuery>) -> Result<Json<R<Vec<Icon>>>, ServerError> {
        let query = "select * from tb_icon";
        let res = match name {
            None => {
                sqlx::query_as::<_, Icon>(query)
                    .fetch_all(&get_conn())
                    .await?
            }
            Some(name) => {
                sqlx::query_as::<_, Icon>(&format!("{} where name like '%' || ? || '%'", query))
                    .bind(name)
                    .fetch_all(&get_conn())
                    .await?
            }
        };
        Ok(Json(R::success_with_data(res)))
    }
}

#[derive(Debug, Deserialize)]
struct IconQuery {
    name: Option<String>,
}

impl Controller for IconHandler {
    ///路由
    fn router(&self) -> Router {
        Router::new()
            .route("/icon", post(Self::save_icon)
                .put(Self::update_icon))
            .route("/icon/:id", delete(Self::delete_icon))
            .route("/icon/list", get(Self::list))
    }
}