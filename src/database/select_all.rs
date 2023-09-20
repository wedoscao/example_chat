use anyhow::Result;
use tokio_postgres::Client;

use super::model::{Account, FromRow, Message};

#[async_trait::async_trait]
pub trait SelectAll<T>
where
    T: Sized,
{
    async fn select_all(client: &Client) -> Result<Vec<T>>;
}

pub struct SelectAllAccount;

#[async_trait::async_trait]
impl SelectAll<Account> for SelectAllAccount {
    async fn select_all(client: &Client) -> Result<Vec<Account>> {
        let rows = client.query("SELECT * FROM accounts", &[]).await?;
        let mut accounts = Vec::<Account>::with_capacity(rows.len());
        for row in rows {
            let account = Account::from_row(row)?;
            accounts.push(account)
        }
        Ok(accounts)
    }
}

pub struct SelectAllMessage;

#[async_trait::async_trait]
impl SelectAll<Message> for SelectAllMessage {
    async fn select_all(client: &Client) -> Result<Vec<Message>> {
        let rows = client.query("SELECT * FROM messages", &[]).await?;
        let mut messages = Vec::<Message>::with_capacity(rows.len());
        for row in rows {
            let message = Message::from_row(row)?;
            messages.push(message)
        }
        Ok(messages)
    }
}
