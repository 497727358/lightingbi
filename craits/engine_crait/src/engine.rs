use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[async_trait]
pub trait Engine {
    type Block;

    async fn ddl_str(&self, ddl: &str) -> Result<(), Box<dyn Error>>;

    async fn query_str(&self, sql: &str) -> Result<Self::Block, Box<dyn Error>>;
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum Engine_Type {
    ClickHouse,
    ElasticSearch,
}
