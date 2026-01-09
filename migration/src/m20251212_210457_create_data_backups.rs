use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE data_backup_status AS ENUM ('RUNNING', 'FAILED', 'SCHEDULED', 'COMPLETED')".to_string(),
            ))
            .await?;

        let data_backups = Table::create()
            .table(DataBackups::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(DataBackups::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(DataBackups::InstitutionId).big_integer())
            .col(ColumnDef::new(DataBackups::BackupName).string().not_null())
            .col(ColumnDef::new(DataBackups::BackupType).string().not_null())
            .col(
                ColumnDef::new(DataBackups::BackupLocation)
                    .text()
                    .not_null(),
            )
            .col(ColumnDef::new(DataBackups::BackupSizeBytes).big_integer())
            .col(
                ColumnDef::new(DataBackups::BackupStatus)
                    .custom("data_backup_status")
                    .default("RUNNING"),
            )
            .col(
                ColumnDef::new(DataBackups::StartedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(ColumnDef::new(DataBackups::CompletedAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(DataBackups::Verified)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(DataBackups::VerifiedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(DataBackups::ExpiresAt).timestamp_with_time_zone())
            .col(ColumnDef::new(DataBackups::RetentionRule).string())
            .col(ColumnDef::new(DataBackups::CreatedBy).big_integer())
            .col(
                ColumnDef::new(DataBackups::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(DataBackups::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(DataBackups::Table, DataBackups::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(DataBackups::Table, DataBackups::CreatedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .to_owned();

        manager.create_table(data_backups).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DataBackups::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum DataBackups {
    Table,
    Id,
    InstitutionId,
    BackupName,
    BackupType,
    BackupLocation,
    BackupSizeBytes,
    BackupStatus,
    StartedAt,
    CompletedAt,
    Verified,
    VerifiedAt,
    ExpiresAt,
    RetentionRule,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
