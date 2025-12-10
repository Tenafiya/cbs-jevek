use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::m20251204_112805_create_institutions::Institutions;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE wallet_prov_status AS ENUM ('ACTIVE', 'INACTIVE')".to_string(),
            ))
            .await?;

        let wal_prov = Table::create()
            .table(WalletProviders::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(WalletProviders::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(WalletProviders::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(WalletProviders::ProviderName)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(WalletProviders::ProviderCode)
                    .string()
                    .not_null()
                    .unique_key(),
            )
            .col(ColumnDef::new(WalletProviders::ApiEndpoint).string())
            .col(ColumnDef::new(WalletProviders::EncryptApiKey).string())
            .col(ColumnDef::new(WalletProviders::EncryptedSecretKey).string())
            .col(ColumnDef::new(WalletProviders::WebhookSecret).string())
            .col(
                ColumnDef::new(WalletProviders::Status)
                    .custom("wallet_prov_status")
                    .default("ACTIVE"),
            )
            .col(
                ColumnDef::new(WalletProviders::IsPrimary)
                    .boolean()
                    .default(false),
            )
            .col(
                ColumnDef::new(WalletProviders::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(WalletProviders::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(WalletProviders::Table, WalletProviders::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(wal_prov).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WalletProviders::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum WalletProviders {
    Table,
    Id,
    InstitutionId,
    ProviderName,
    ProviderCode,
    ApiEndpoint,
    EncryptApiKey,
    EncryptedSecretKey,
    WebhookSecret,
    Status,
    IsPrimary,
    CreatedAt,
    UpdatedAt,
}
