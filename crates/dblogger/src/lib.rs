use secrets;
use shared::*;

use async_once::AsyncOnce;
pub use async_trait::async_trait;
use lazy_static::lazy_static;
pub use sqlx;
use sqlx::{postgres::PgPoolOptions, PgPool};

lazy_static! {
    static ref POOL: AsyncOnce<PgPool> = AsyncOnce::new(async { get_pool().await.unwrap() });
}

pub type DbTime = Option<sqlx::types::chrono::DateTime<chrono::Local>>;

pub async fn get_pool() -> ResultV1<PgPool> {
    let password = secrets::get_database_password()?;
    let something = PgPoolOptions::new()
        .max_connections(5)
        .connect(format!("postgres://postgres:{password}@localhost/postgres").as_str())
        .await?;
    Ok(something)
}

#[async_trait]
pub trait DbLogger {
    async fn gen_log_with_pool(&self, pool: &PgPool) -> ResultV1<()>;
    async fn gen_create_table_with_pool(pool: &PgPool) -> ResultV1<()>;
    async fn gen_create_table() -> ResultV1<()> {
        let pool = get_pool().await?;
        Self::gen_create_table_with_pool(&pool).await
    }
    async fn gen_log(&self) -> ResultV1<()> {
        let pool = get_pool().await?;
        Self::gen_create_table_with_pool(&pool).await?;
        self.gen_log_with_pool(&pool).await
    }
}
