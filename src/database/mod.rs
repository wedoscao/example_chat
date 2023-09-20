use anyhow::Result;
use tokio_postgres::{Client, NoTls};

use self::{insert_one::InsertOne, select_all::SelectAll};

pub struct Database {
    client: Client,
}

impl Database {
    pub async fn new(config: &str) -> Result<Self> {
        let (client, connection) = tokio_postgres::connect(config, NoTls).await?;
        tokio::spawn(async {
            if let Err(err) = connection.await {
                eprintln!("Database connection error: {}", err)
            }
        });
        Ok(Database { client })
    }

    pub async fn select_all<T, S>(&self, _: S) -> Result<Vec<T>>
    where
        T: Sized,
        S: SelectAll<T>,
    {
        S::select_all(&self.client).await
    }

    pub async fn create_one<T, S>(&self, data: T, _: S) -> Result<()>
    where
        T: Sized,
        S: InsertOne<T>,
    {
        S::insert_one(&self.client, data).await
    }
}

pub mod insert_one;
pub mod model;
pub mod select_all;
