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

impl From<&str> for ServerConfig {
    fn from(value: &str) -> Self {
        if !value.contains(':') {
            panic!("服务器地址错误:{}", value)
        }
        let (host, port) = value.split_once(':').unwrap();
        Self {
            port: port.parse().unwrap(),
            host: host.to_string(),
        }
    }
}