use sea_orm_migration::prelude::*;
use sea_orm::Statement;

use crate::m20251204_112805_create_institutions::Institutions;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let report_analytics_cache = Table::create()
            .table(ReportAnalyticsCache::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(ReportAnalyticsCache::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(ReportAnalyticsCache::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(ReportAnalyticsCache::ReportType)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(ReportAnalyticsCache::CacheKey)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(ReportAnalyticsCache::CachedData)
                    .json_binary()
                    .not_null(),
            )
            .col(ColumnDef::new(ReportAnalyticsCache::DataHash).string())
            .col(
                ColumnDef::new(ReportAnalyticsCache::GeneratedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(ReportAnalyticsCache::ExpiresAt)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(ColumnDef::new(ReportAnalyticsCache::GenerationTimeMs).integer())
            .col(ColumnDef::new(ReportAnalyticsCache::DataSizeBytes).integer())
            .foreign_key(
                ForeignKey::create()
                    .from(
                        ReportAnalyticsCache::Table,
                        ReportAnalyticsCache::InstitutionId,
                    )
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(report_analytics_cache).await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                    ALTER TABLE report_analytics_cache
                    ADD CONSTRAINT unique_rac_insti_type_key_date
                    UNIQUE (institution_id, report_type, cache_key, generated_at);
                "#
                .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                CREATE INDEX idx_analytics_cache_expiry ON report_analytics_cache(expires_at);
            "#
                .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                CREATE INDEX idx_analytics_cache_key ON report_analytics_cache(cache_key);
            "#
                .to_string(),
            ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ReportAnalyticsCache::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ReportAnalyticsCache {
    Table,
    Id,
    InstitutionId,
    ReportType,
    CacheKey,
    CachedData,
    DataHash,
    GeneratedAt,
    ExpiresAt,
    GenerationTimeMs,
    DataSizeBytes,
}
