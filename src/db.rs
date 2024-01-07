use std::env;

use anyhow::{Context, Ok, Result};
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
pub type DbPool = MySqlPool;

pub struct DbConnection;

impl DbConnection {
    pub async fn create_db_connection() -> Result<MySqlPool> {
        let database_url = env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        Ok(pool)
    }
}
