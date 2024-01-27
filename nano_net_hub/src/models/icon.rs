use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Icon {
    pub(crate) id: i32,
    pub(crate) icon: String,
    pub(crate) name: String,
    pub(crate) default_icon: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateIcon {
    pub(crate) icon: String,
    pub(crate) name: String,
}