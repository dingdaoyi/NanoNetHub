use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub(crate) id: i32,
    pub(crate) username: String,
    pub(crate) name: Option<String>,
    pub(crate) password: Option<String>,
    pub(crate) email: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub(crate) username: String,
    pub(crate) name: Option<String>,
    pub(crate) email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserQuery {
    pub(crate) username: Option<String>
}
