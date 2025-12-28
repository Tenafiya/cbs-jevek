use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
    m20251212_195551_create_integration_providers::IntegrationProviders,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the INET extension if not exists
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"".to_string(),
            ))
            .await?;

        let integration_api_keys = Table::create()
            .table(IntegrationApiKeys::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(IntegrationApiKeys::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(IntegrationApiKeys::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(IntegrationApiKeys::ProviderId).big_integer())
            .col(
                ColumnDef::new(IntegrationApiKeys::KeyName)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(IntegrationApiKeys::ApiKeyEncrypted)
                    .text()
                    .not_null(),
            )
            .col(ColumnDef::new(IntegrationApiKeys::ApiSecretEncrypted).text())
            .col(ColumnDef::new(IntegrationApiKeys::KeyIdentifier).string())
            .col(ColumnDef::new(IntegrationApiKeys::Permissions).json_binary())
            .col(ColumnDef::new(IntegrationApiKeys::AllowedIpAddresses).custom("INET[]"))
            .col(
                ColumnDef::new(IntegrationApiKeys::KeyStatus)
                    .string()
                    .default("ACTIVE"),
            )
            .col(
                ColumnDef::new(IntegrationApiKeys::IssuedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(ColumnDef::new(IntegrationApiKeys::ValidUntil).timestamp_with_time_zone())
            .col(
                ColumnDef::new(IntegrationApiKeys::UsageCount)
                    .integer()
                    .default(0),
            )
            .col(ColumnDef::new(IntegrationApiKeys::LastUsedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(IntegrationApiKeys::CreatedBy).big_integer())
            .col(
                ColumnDef::new(IntegrationApiKeys::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(IntegrationApiKeys::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(IntegrationApiKeys::Table, IntegrationApiKeys::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(IntegrationApiKeys::Table, IntegrationApiKeys::ProviderId)
                    .to(IntegrationProviders::Table, IntegrationProviders::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(IntegrationApiKeys::Table, IntegrationApiKeys::CreatedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .index(
                Index::create()
                    .name("idx_integration_api_keys_provider")
                    .table(IntegrationApiKeys::Table)
                    .col(IntegrationApiKeys::ProviderId),
            )
            .index(
                Index::create()
                    .name("idx_integration_api_keys_status")
                    .table(IntegrationApiKeys::Table)
                    .col(IntegrationApiKeys::KeyStatus),
            )
            .index(
                Index::create()
                    .name("idx_integration_api_keys_expiry")
                    .table(IntegrationApiKeys::Table)
                    .col(IntegrationApiKeys::ValidUntil),
            )
            .to_owned();

        manager.create_table(integration_api_keys).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IntegrationApiKeys::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum IntegrationApiKeys {
    Table,
    Id,
    InstitutionId,
    ProviderId,
    KeyName,
    ApiKeyEncrypted,
    ApiSecretEncrypted,
    KeyIdentifier,
    Permissions,
    AllowedIpAddresses,
    KeyStatus,
    IssuedAt,
    ValidUntil,
    UsageCount,
    LastUsedAt,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
