use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
    m20251207_183348_create_wallet_providers::WalletProviders,
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
                "CREATE TYPE wallet_reconciliation_status AS ENUM ('PENDING', 'MATCHED', 'UNMATCHED')".to_string(),
            ))
            .await?;

        let recon = Table::create()
            .table(WalletReconciliations::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(WalletReconciliations::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(WalletReconciliations::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(WalletReconciliations::WalletProviderId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(WalletReconciliations::ReconciliationDate).date())
            .col(ColumnDef::new(WalletReconciliations::ProviderOpeningBalance).decimal_len(20, 4))
            .col(ColumnDef::new(WalletReconciliations::ProviderClosingBalance).decimal_len(20, 4))
            .col(ColumnDef::new(WalletReconciliations::ProviderTotalCredits).decimal_len(20, 4))
            .col(ColumnDef::new(WalletReconciliations::ProviderTotalDebits).decimal_len(20, 4))
            .col(ColumnDef::new(WalletReconciliations::ProviderTransactionCount).integer())
            .col(
                ColumnDef::new(WalletReconciliations::DifferenceAmount)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(WalletReconciliations::Status)
                    .custom("wallet_reconciliation_status")
                    .default("PENDING"),
            )
            .col(ColumnDef::new(WalletReconciliations::ReconciledAt).timestamp_with_time_zone())
            .col(ColumnDef::new(WalletReconciliations::DiscrepancyDetails).json_binary())
            .col(ColumnDef::new(WalletReconciliations::ReconciledBy).big_integer())
            .col(
                ColumnDef::new(WalletReconciliations::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(WalletReconciliations::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        WalletReconciliations::Table,
                        WalletReconciliations::InstitutionId,
                    )
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        WalletReconciliations::Table,
                        WalletReconciliations::WalletProviderId,
                    )
                    .to(WalletProviders::Table, WalletProviders::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        WalletReconciliations::Table,
                        WalletReconciliations::ReconciledBy,
                    )
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(recon).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WalletReconciliations::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum WalletReconciliations {
    Table,
    Id,
    InstitutionId,
    WalletProviderId,
    ReconciliationDate,
    ProviderOpeningBalance,
    ProviderClosingBalance,
    ProviderTotalCredits,
    ProviderTotalDebits,
    ProviderTransactionCount,
    DifferenceAmount,
    Status,
    ReconciledBy,
    ReconciledAt,
    DiscrepancyDetails,
    CreatedAt,
    UpdatedAt,
}
