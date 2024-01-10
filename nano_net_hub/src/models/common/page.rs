use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum Direction {
    /**
     * 排序方式
     */
    ASC,
    DESC,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::ASC
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::ASC => {
                write!(f, "ASC")
            }
            Direction::DESC => {
                write!(f, "DESC")
            }
        }
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct PaginationRequest {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub size: u32,

    #[serde(default = "default_sort_fields")]
    pub sort_fields: Vec<String>,
    #[serde(default = "default_direction")]
    pub direction: Direction,
}

impl PaginationRequest {
    pub fn limit(&self) -> u32 {
        self.size
    }
    pub fn offset(&self) -> u32 {
        (self.page - 1) * self.size
    }
}

fn default_page() -> u32 {
    1
}

fn default_sort_fields() -> Vec<String> {
    vec![]
}

fn default_limit() -> u32 {
    10
}

fn default_direction() -> Direction {
    Direction::DESC
}

#[derive(Debug, Serialize)]
pub struct PaginationResponse<T: Sized> {
    pub total: u32,
    pub data: Vec<T>,
}

impl<T: Sized> PaginationResponse<T> {
    pub fn new(data: Vec<T>, total: u32) -> Self {
        Self { total, data }
    }
}
