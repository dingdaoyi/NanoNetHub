pub mod database;
pub mod date_format;
pub mod option_serialize;
mod server_config;

use std::fs::File;
use std::io::Read;
pub use server_config::ServerConfig;
pub use server_config::MqttConfig;
pub use server_config::Config;
pub use server_config::AuthConfig;

pub fn get_config() -> Config {
    let mut toml_str = String::new();
    File::open("config.toml")
        .expect("Unable to open the file")
        .read_to_string(&mut toml_str)
        .expect("Unable to read the file");
    toml::from_str(&toml_str).unwrap()
}