use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mqtt: Option<MqttConfig>,
    pub server: ServerConfig,
    pub auth: AuthConfig,
}


#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    pub username: String,
    pub expire_minutes: usize,
    pub password: String,
    pub jwt_secret: String,
}

#[derive(Debug, Deserialize)]
pub struct MqttConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl MqttConfig {
    pub fn get_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("host".to_string(), self.host.clone());
        map.insert("port".to_string(), format!("{}", self.port));
        map.insert("username".to_string(), self.username.clone().to_string());
        map.insert("password".to_string(), self.password.clone().to_string());
        map
    }
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    //端口号
    port: u16,
    // 地址
    host: String,
}

impl ServerConfig {
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
