use std::env;

use anyhow::{bail, Result};
use chrono::{DateTime, Local};
use pbkdf2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};

use crate::database::model::FromRow;

#[derive(Debug)]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug)]
pub struct Account {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub role: Role,
    pub created_at: DateTime<Local>,
}

impl FromRow for Account {
    fn from_row(row: tokio_postgres::Row) -> Result<Self>
    where
        Self: Sized,
    {
        let id = row.try_get::<&str, String>("id")?;
        let username = row.try_get::<&str, String>("username")?;
        let password_hash = row.try_get::<&str, String>("password_hash")?;
        let role = match row.try_get::<&str, i16>("role")? {
            0 => Role::Admin,
            1 => Role::User,
            _ => bail!("invalid role"),
        };
        let created_at = row.try_get::<&str, DateTime<Local>>("created_at")?;
        Ok(Self {
            id,
            username,
            password_hash,
            role,
            created_at,
        })
    }
}

impl Account {
    fn hash_password(password: &str) -> Result<String> {
        dotenv::dotenv()?;
        let salt = match SaltString::from_b64(&env::var("SALT")?) {
            Ok(salt) => salt,
            Err(err) => bail!("{}", err),
        };
        match Pbkdf2.hash_password(password.as_bytes(), &salt) {
            Ok(password_hash) => Ok(password_hash.to_string()),
            Err(err) => bail!("{}", err),
        }
    }

    pub fn new(username: String, password: &str, role: Role) -> Result<Self> {
        let id = cuid2::create_id();
        let password_hash = Self::hash_password(password)?;
        let created_at = Local::now();
        Ok(Self {
            id,
            username,
            password_hash,
            role,
            created_at,
        })
    }

    pub fn verify(&self, password: &str) -> Result<()> {
        let password_hash = match PasswordHash::new(&self.password_hash) {
            Ok(password_hash) => password_hash,
            Err(err) => bail!("{}", err),
        };
        if let Err(err) = Pbkdf2.verify_password(password.as_bytes(), &password_hash) {
            bail!("{}", err)
        };
        Ok(())
    }
}
