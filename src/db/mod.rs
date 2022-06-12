use super::*;
use async_trait::async_trait;
mod sheet;

#[async_trait]
pub trait Database<T> {
    async fn retrieve(&self) -> Result<Vec<T>, ()>;
    async fn rewrite(&self, new: Vec<T>) -> Result<(), ()>;
    async fn append(&self, new: T) -> Result<(), ()>;
    async fn update(&self, id: u64, values: T) -> Result<(), ()>;
    async fn log(&self, time: DateTime<Utc>, update: String) -> Result<(), ()>;
}

pub async fn run() -> impl Database<Book> {
    sheet::SheetDatabase::new().await
}