use NanoNetHub::models::ServerError;
use NanoNetHub::run_server;

#[tokio::main]
async  fn main()->Result<(),ServerError> {
    run_server().await
}
