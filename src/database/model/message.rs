use anyhow::Result;
use chrono::{DateTime, Local};

use super::FromRow;

#[derive(Debug)]
pub struct Message {
    pub id: String,
    pub account_id: String,
    pub content: String,
    pub is_edited: bool,
    pub created_at: DateTime<Local>,
}

impl FromRow for Message {
    fn from_row(row: tokio_postgres::Row) -> Result<Self>
    where
        Self: Sized,
    {
        let id = row.try_get::<&str, String>("id")?;
        let account_id = row.try_get::<&str, String>("account_id")?;
        let content = row.try_get::<&str, String>("content")?;
        let is_edited = row.try_get::<&str, bool>("is_edited")?;
        let created_at = row.try_get::<&str, DateTime<Local>>("created_at")?;
        Ok(Self {
            id,
            account_id,
            content,
            is_edited,
            created_at,
        })
    }
}

impl Message {
    pub fn new(id: String, account_id: String, content: String) -> Self {
        let is_edited = false;
        let created_at = Local::now();
        Self {
            id,
            account_id,
            content,
            is_edited,
            created_at,
        }
    }
}
