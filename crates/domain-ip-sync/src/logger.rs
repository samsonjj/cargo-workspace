use dblogger::{async_trait, sqlx, DbTime};
use shared::*;

pub use dblogger::DbLogger;

pub struct DomainIpSyncRow {
    pub id: u32,
    pub hosted_zone_id: String,
    pub domain: String,
    pub ip_address: String,
    pub timestamp: DbTime,
}

#[async_trait]
impl DbLogger for DomainIpSyncRow {
    async fn gen_log_with_pool(&self, pool: &sqlx::PgPool) -> ResultV1<()> {
        sqlx::query(
            "INSERT INTO domain_ip_sync (domain, ip_address, hosted_zone_id) VALUES ($1, $2, $3)",
        )
        .bind(self.domain.to_string())
        .bind(self.ip_address.to_string())
        .bind(self.hosted_zone_id.to_string())
        .execute(pool)
        .await?;
        Ok(())
    }

    async fn gen_create_table_with_pool(pool: &sqlx::PgPool) -> ResultV1<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS domain_ip_sync (
                id SERIAL PRIMARY KEY,
                domain TEXT NOT NULL,
                ip_address TEXT NOT NULL
            )",
        )
        .execute(pool)
        .await?;

        sqlx::query(
            "ALTER TABLE domain_ip_sync
            ADD COLUMN IF NOT EXISTS
                hosted_zone_id TEXT",
        )
        .execute(pool)
        .await?;

        sqlx::query(
            "ALTER TABLE domain_ip_sync
            ADD COLUMN IF NOT EXISTS
                ts TIMESTAMPTZ NOT NULL DEFAULT NOW();",
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
