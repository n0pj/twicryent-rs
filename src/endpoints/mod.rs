pub mod follows;
pub mod user;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Data<T> {
    pub data: T,
    pub meta: Option<Meta>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    pub result_count: u64,
    pub next_token: Option<String>,
    pub previous_token: Option<String>,
}
