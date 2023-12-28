use tracing::info;
use crate::models::ServerError;
use crate::server::routers::server_router;

pub mod server_config;
pub mod routers;
pub mod handler;

pub use server_config::ServerConfig;
pub struct Server {
    config: ServerConfig,
}

impl Server {
    pub fn new(config: ServerConfig) -> Self {
        Self { config }
    }
    pub async fn start(&self) -> Result<(), ServerError> {
        let listener = tokio::net::TcpListener::bind(self.config.addr()).await.unwrap();
        info!("服务启动成功:{}",self.config.addr());
        axum::serve(listener, server_router()).await?;
        Ok(())
    }
}
