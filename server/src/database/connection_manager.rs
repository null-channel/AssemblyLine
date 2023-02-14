use sqlx::postgres::PgPool;
use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use sqlx::any::AnyPool;
use sqlx::pool::{PoolConnection};
use sqlx::database::Database;
use sqlx::error::Error;

pub struct ConnectionManager {
    pool: AnyPool,
}

impl ConnectionManager {
    pub async fn new(url:String) -> anyhow::Result<Self> {
        let pool = AnyPool::connect(&url).await?;
        Ok(Self { pool })
    }
}