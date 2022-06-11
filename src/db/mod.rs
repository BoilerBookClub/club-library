use super::*;
use async_trait::async_trait;
mod sheet;

#[async_trait]
pub trait Database {
    async fn retrieve(&self) -> Result<Vec<Book>, ()>;
    async fn update(&self, new: Vec<Book>) -> Result<(), ()>;
    async fn log_update(&self, time: DateTime<Utc>, update: String) -> Result<(), ()>;
}

pub async fn run() -> impl Database {
    sheet::SheetDatabase::new().await
}