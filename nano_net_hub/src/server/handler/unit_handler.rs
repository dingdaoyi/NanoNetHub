use axum::{Json, Router};
use axum::routing::get;
use crate::config::database::get_conn;
use crate::models::{R, ServerError};
use crate::models::tls::unit::Unit;
use crate::server::handler::base::Controller;

#[derive(Default)]
pub struct UnitHandler;

impl Controller for UnitHandler {
    fn router(&self) -> Router {
        Router::new()
            .route("/unit", get(Self::list))
    }
}

impl UnitHandler {
    ///查询列表
    async fn list() -> Result<Json<R<Vec<Unit>>>, ServerError> {
        let units = sqlx::query_as::<_, Unit>(
            r#"
        select * from tb_unit
        "#)
            .fetch_all(&get_conn())
            .await?;
        Ok(Json(R::success_with_data(units)))
    }
}