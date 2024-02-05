use std::path::PathBuf;
use axum::extract::Multipart;
use axum::{Json, Router};
use axum::routing::post;
use chrono::Local;
use futures_util::TryStreamExt;
use rand::distributions::Alphanumeric;
use rand::Rng;
use tokio::fs::create_dir_all;
use tokio::io::AsyncWriteExt;
use tower_http::services::ServeDir;
use crate::models::{R, ServerError};
use crate::server::handler::base::Controller;

#[derive(Default)]
pub struct FileHandler;

impl FileHandler {
    async fn upload_file(mut multipart: Multipart) -> Result<Json<R<String>>, ServerError> {
        // 获取文件字段
        let file_field = multipart.next_field().await?;
        if let Some(field) = file_field {
            let filename = Self::generate_random_filename(field.file_name());
            let path = Local::now().format("%Y%m%d");
            let upload_dir = PathBuf::from(format!("uploads/{}", path)); // Replace with your desired upload directory
            create_dir_all(&upload_dir).await?;
            let filepath = upload_dir.join(&filename);
            field.into_stream()
                .map_err(|e| ServerError::IoError(e.to_string())).try_fold(
                tokio::fs::File::create(filepath).await?,
                |mut file, bytes| async move {
                    file.write_all(&bytes).await?;
                    Ok(file)
                },
            ).await?;
            return Ok(Json(R::success_with_data(format!("/file/{}/{}", path, filename))));
        }
        Ok(Json(R::bad_request("no file".into())))
    }

    fn generate_random_filename(original_filename: Option<&str>) -> String {
        let mut rng = rand::thread_rng();
        let random_string: String = std::iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(10)
            .map(char::from)  // Convert u8 to char before collecting
            .collect();


        let default_name = "unknown".to_string();
        let base_filename = original_filename.unwrap_or(&default_name);
        let binding = PathBuf::from(base_filename);
        let extension = binding.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("dat");
        format!("{}_{}.{}", random_string, chrono::Utc::now().timestamp_nanos_opt().unwrap(), extension)
    }
    pub async fn delete_file(path: &str) -> Result<(), ServerError> {
        // "/api/file/20240116/h1NgEmRABf_1705371062664589000.png"
        let file = path.strip_prefix("/api/file/").unwrap_or("");
        let upload_path = PathBuf::from("uploads");
        let file_path = upload_path.join(file);
        if file_path.exists() {
            tokio::fs::remove_file(file_path).await?;
        }
        Ok(())
    }
}

impl Controller for FileHandler {
    fn router(&self) -> Router {
        Router::new()
            .nest_service("/file", ServeDir::new("uploads"))
            .route("/file/upload", post(Self::upload_file))
    }
}