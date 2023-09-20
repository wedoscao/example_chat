use anyhow::Result;
use tokio_postgres::Row;

pub trait FromRow {
    fn from_row(row: Row) -> Result<Self>
    where
        Self: Sized;
}

mod account;
mod message;

pub use account::*;
pub use message::*;
