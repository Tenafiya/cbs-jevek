use sea_orm_migration::prelude::*;

use crate::{
    m20251204_112805_create_institutions::Institutions,
    m20251204_152312_create_customers::Customers,
    m20251207_183348_create_wallet_providers::WalletProviders,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let wallets = Table::create()
            .table(Wallets::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Wallets::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(Wallets::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(Wallets::CustomerId).big_integer().not_null())
            .col(
                ColumnDef::new(Wallets::WalletProviderId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(Wallets::WalletNumber)
                    .string()
                    .not_null()
                    .unique_key(),
            )
            .col(ColumnDef::new(Wallets::WalletName).string())
            .col(ColumnDef::new(Wallets::Currency).json_binary())
            .col(
                ColumnDef::new(Wallets::AvailableBalance)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(Wallets::LedgerBalance)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(Wallets::HoldBalance)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(Wallets::Status)
                    .custom("wallet_prov_status")
                    .default("ACTIVE"),
            )
            .col(ColumnDef::new(Wallets::IsPrimary).boolean().default(false))
            .col(ColumnDef::new(Wallets::ProviderCustomerId).string())
            .col(ColumnDef::new(Wallets::ProviderWalletId).string())
            .col(ColumnDef::new(Wallets::ProviderMetadata).json_binary())
            .col(
                ColumnDef::new(Wallets::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Wallets::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Wallets::Table, Wallets::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Wallets::Table, Wallets::CustomerId)
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Wallets::Table, Wallets::WalletProviderId)
                    .to(WalletProviders::Table, WalletProviders::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(wallets).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Wallets::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Wallets {
    Table,
    Id,
    InstitutionId,
    CustomerId,
    WalletProviderId,
    WalletNumber,
    WalletName,
    Currency,
    AvailableBalance,
    LedgerBalance,
    HoldBalance,
    Status,
    IsPrimary,
    ProviderCustomerId,
    ProviderWalletId,
    ProviderMetadata,
    CreatedAt,
    UpdatedAt,
}
