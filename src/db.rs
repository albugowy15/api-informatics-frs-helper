use std::env;

use anyhow::{Context, Ok, Result};
use sqlx::{mysql::MySqlPoolOptions, MySql, MySqlPool, Pool};
pub type DbPool = MySqlPool;

pub struct DbConnection {
    pub pool: Pool<MySql>,
}

impl DbConnection {
    pub async fn new() -> Result<Self> {
        let database_url = env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .with_context(|| "Error opening database connection")?;

        Ok(Self { pool })
    }
}
