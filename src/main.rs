use nano_net_hub::models::ServerError;
use nano_net_hub::run_server;

#[tokio::main]
async  fn main()->Result<(),ServerError> {
    run_server().await
}
