use sea_orm_migration::prelude::*;

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
            .index(
                Index::create()
                    .name("idx_report_analytics_cache")
                    .table(ReportAnalyticsCache::Table)
                    .col(ReportAnalyticsCache::InstitutionId)
                    .col(ReportAnalyticsCache::ReportType)
                    .col(ReportAnalyticsCache::CacheKey)
                    .col(ReportAnalyticsCache::GeneratedAt)
                    .unique(),
            )
            .index(
                Index::create()
                    .name("idx_report_cache_type")
                    .table(ReportAnalyticsCache::Table)
                    .col(ReportAnalyticsCache::ReportType),
            )
            .index(
                Index::create()
                    .name("idx_report_cache_expires")
                    .table(ReportAnalyticsCache::Table)
                    .col(ReportAnalyticsCache::ExpiresAt),
            )
            .index(
                Index::create()
                    .name("idx_report_cache_key")
                    .table(ReportAnalyticsCache::Table)
                    .col(ReportAnalyticsCache::CacheKey),
            )
            .to_owned();

        manager.create_table(report_analytics_cache).await?;

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
