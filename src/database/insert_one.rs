use anyhow::Result;
use tokio_postgres::Client;

use super::model::{Account, Message, Role};

#[async_trait::async_trait]
pub trait InsertOne<T>
where
    T: Sized,
{
    async fn insert_one(client: &Client, data: T) -> Result<()>;
}

pub struct InsertOneAccount;

#[async_trait::async_trait]
impl InsertOne<Account> for InsertOneAccount {
    async fn insert_one(client: &Client, data: Account) -> Result<()> {
        let role: i16 = match data.role {
            Role::Admin => 0,
            Role::User => 1,
        };
        client
            .query(
                "INSERT INTO accounts (id, username, password_hash, role, created_at) VALUES ($1, $2, $3, $4, $5)",
                &[&data.id, &data.username, &data.password_hash, &role, &data.created_at],
            )
            .await?;
        Ok(())
    }
}

pub struct InsertOneMessage;

#[async_trait::async_trait]
impl InsertOne<Message> for InsertOneMessage {
    async fn insert_one(client: &Client, data: Message) -> Result<()> {
        client.query(
            "INSERT INTO messages (id, account_id, content, is_edited ,created_at) VALUES ($1, $2, $3, $4, $5)", 
            &[&data.id, &data.account_id, &data.content, &data.is_edited, &data.created_at],
        )
        .await?;
        Ok(())
    }
}
