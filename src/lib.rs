use crate::config::database::init_database;
use crate::models::ServerError;

pub mod models;
mod config;

pub  async  fn run_server()->Result<(),ServerError>{
    init_database().await?;
    Ok(())
}