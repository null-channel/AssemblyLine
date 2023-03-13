use sqlx::sqlite::SqlitePool;

pub struct ConnectionManager {
    pub pool: SqlitePool,
}

impl ConnectionManager {
    pub async fn new(url: String) -> anyhow::Result<Self> {
        let pool = SqlitePool::connect(&url).await?;
        Ok(Self { pool })
    }
}
