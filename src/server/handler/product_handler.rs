use axum::{Json, Router};
use axum::routing::post;
use chrono::Utc;
use sqlx::{Acquire, Executor};
use crate::config::database::get_conn;
use crate::models::{PaginationResponse, Product, R, ServerError};
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
        let mut sql = String::from("SELECT * FROM tb_product");
        let mut count_sql = String::from("SELECT count(*) FROM tb_product");
        if let Some(product_name) = product_name {
            // 怎么判断防注入
            sql = format!("{} where product_name like %{}", sql, product_name);
            count_sql = format!("{} where product_name like %{}", count_sql, product_name);
        }
        let pool = get_conn();
        sql = format!("{} LIMIT ? OFFSET ?", sql);
        let count_query =
            sqlx::query_scalar::<_, u32>(&count_sql)
                .fetch_one(&pool)
                .await?;

        match count_query {
            0 => {
                Ok(Json(R::success_with_data(PaginationResponse::new(vec![], 0))))
            }
            total => {
                let products = sqlx::query_as::<_, Product>(&sql)
                    .bind(base_query.limit())
                    .bind(base_query.offset())
                    .fetch_all(&pool)
                    .await?;
                Ok(Json(R::success_with_data(PaginationResponse::new(products, total))))
            }
        }
    }

    // 分页查询
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
                Ok(Json(R::fail("插入失败".into())))
            }
        }
    }
}