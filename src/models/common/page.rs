use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PaginationRequest {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub size: u32,
}

impl PaginationRequest {
    pub fn limit(&self) -> u32 {
        self.size
    }
    pub fn offset(&self) -> u32 {
        self.page * self.size
    }
}

fn default_page() -> u32 {
    0
}

fn default_limit() -> u32 {
    10
}

#[derive(Debug, Serialize)]
pub struct PaginationResponse<T> {
    pub total: u32,
    pub data: Vec<T>,
}

impl<T> PaginationResponse<T> {
    pub fn new(data: Vec<T>, total: u32) -> Self {
        Self { total, data }
    }
}

enum SqlCondition<T>
    where T: ToString, {
    Equal(String, T),
    Like(String, T),
}
