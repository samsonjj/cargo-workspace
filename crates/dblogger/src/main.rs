use async_trait::async_trait;
use dblogger::{get_pool, DbLogger};
use sqlx::{self, postgres::PgPoolOptions, prelude::FromRow, PgPool, Pool, Postgres};

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

use async_once::AsyncOnce;
use lazy_static::lazy_static;

#[async_trait]
impl DbLogger for SystemMessageRow {
    async fn gen_log_with_pool(&self, pool: &PgPool) -> MyResult<()> {
        sqlx::query("INSERT INTO system_messages (message) VALUES ($1)")
            .bind(self.message.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn gen_create_table_with_pool(pool: &PgPool) -> MyResult<()> {
        let result = sqlx::query(
            "CREATE TABLE IF NOT EXISTS system_messages (
                id SERIAL PRIMARY KEY,
                message TEXT NOT NULL
            )",
        )
        .execute(pool)
        .await?;

        sqlx::query(
            "ALTER TABLE system_messages
            ADD COLUMN IF NOT EXISTS
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();",
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

impl SystemMessageRow {
    async fn get(id: i32) -> MyResult<Self> {
        let result = sqlx::query_as::<_, Self>("SELECT * FROM system_messages WHERE id = $1")
            .bind(id)
            .fetch_one(&get_pool().await?)
            .await?;
        Ok(result)
    }
}

#[derive(FromRow, Debug)]
struct SystemMessageRow {
    pub id: i32,
    pub message: String,
    pub created_at: Option<sqlx::types::chrono::DateTime<chrono::Local>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    SystemMessageRow::gen_create_table().await?;
    SystemMessageRow {
        id: 0,
        message: "Hello!".to_string(),
        created_at: None,
    }
    .gen_log()
    .await?;
    println!("{:?}", SystemMessageRow::get(1).await?);

    Ok(())
}
