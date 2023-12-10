use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct LoginQuery {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct UserInfo {
    pub username: String,
    pub token: String,
    pub avatar: Option<String>,
}

impl UserInfo {
    pub fn default() -> Self {
        return Self {
            username: "admin".to_string(),
            token: "aaa231".to_string(),
            avatar: None,
        };
    }
}