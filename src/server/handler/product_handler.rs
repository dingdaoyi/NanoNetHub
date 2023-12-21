use axum::{Json, Router};
use axum::extract::Path;
use axum::routing::{delete, post};
use chrono::Utc;
use sqlx::{Acquire, Executor};
use crate::config::database::get_conn;
use crate::models::{PaginationResponse, Product, R, ServerError};
use crate::models::common::sqlx_page::{Condition, PageSqlBuilder};
use crate::models::tls::product::{CreateProduct, ProductQuery, UpdateProduct};
use crate::server::handler::base::Controller;

#[derive(Default)]
pub struct ProductHandler;

impl Controller for ProductHandler {
    fn router(&self) -> Router {
        Router::new()
            .route("/product", post(Self::create_product)
                .put(Self::update))
            .route("/product/page", post(Self::page))
            .route("/product/:id", delete(Self::delete)
                .get(Self::details))
    }
}

impl ProductHandler {
    // 创建
    async fn create_product(
        Json(product): Json<CreateProduct>,
    ) -> Result<Json<R<String>>, ServerError> {
        let mut conn = get_conn().acquire().await?;
        let mut transaction = conn.begin().await?;
        transaction
            .execute(
                sqlx::query(
                    "INSERT INTO tb_product ( product_name , description, deleted, create_time) VALUES (?, ?, ?, ?)",
                )
                    .bind(product.product_name)
                    .bind(product.description)
                    .bind(false)
                    .bind(Utc::now()),
            )
            .await?;
        transaction.commit().await?;
        Ok(Json(R::success()))
    }

    // 分页查询
    async fn page(
        Json(ProductQuery { product_name, base_query }): Json<ProductQuery>,
    ) -> Result<Json<R<PaginationResponse<Product>>>, ServerError> {
        let mut builder = PageSqlBuilder::builder("tb_product", &base_query)
            .where_query("deleted=false");
        if let Some(product_name) = product_name {
            // 怎么判断防注入
            builder = builder.conditions(vec![Condition::Like("product_name", product_name.into())]);
        }
        builder.build().execute()
            .await
            .map(|value|
                Json(R::success_with_data(value)))
    }

    // 分页查询
    async fn details(
        Path(id): Path<i32>,
    ) -> Result<Json<R<Option<Product>>>, ServerError> {
        let res = sqlx::query_as(
            "select * from tb_product where id=?"
        )
            .bind(id)
            .fetch_optional(&get_conn())
            .await?;
        Ok(Json(R::success_with_data(res)))
    }

    // 修改
    async fn update(
        Json(product): Json<UpdateProduct>,
    ) -> Result<Json<R<String>>, ServerError> {
        let rows_affected = sqlx::query!(
           r#"
           update tb_product set product_name=?,
            description =?
            where id=?
           "#,
           product.product_name,
           product.description,
           product.id
       ).execute(&get_conn())
            .await?
            .rows_affected();
        match rows_affected > 0 {
            true => {
                Ok(Json(R::success()))
            }
            false => {
                Ok(Json(R::bad_request("插入失败".into())))
            }
        }
    }

    // 逻辑删除
    async fn delete(
        Path(id): Path<i32>,
    ) -> Result<Json<R<String>>, ServerError> {
        let rows_affected = sqlx::query!(
           r#"
           delete from tb_product where id=?
           "#,
          id
       ).execute(&get_conn())
            .await?
            .rows_affected();
        match rows_affected > 0 {
            true => {
                Ok(Json(R::success()))
            }
            false => {
                Ok(Json(R::fail("删除失败".into())))
            }
        }
    }
    pub async fn exists(id: i32) -> Result<bool, ServerError> {
        let res = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT * FROM tb_product WHERE id = ?)")
            .bind(id)
            .fetch_one(&get_conn())
            .await?;
        Ok(res)
    }
}