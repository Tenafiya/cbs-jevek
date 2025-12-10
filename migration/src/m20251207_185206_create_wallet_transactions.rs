use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions,
    m20251205_193221_create_transactions::Transactions, m20251207_184227_create_wallets::Wallets,
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
                "CREATE TYPE wallet_trans_direction AS ENUM ('IN', 'OUT')".to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE wallet_trans_type AS ENUM ('TOPUP', 'WITHDRAWAL', 'P2P', 'MERCHANT_PAYMENT', 'BANK_TRANSFER')".to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE wallet_trans_status AS ENUM ('PENDING', 'PROCESSING', 'PAID', 'FAILED', 'CANCELLED')".to_string(),
            ))
            .await?;

        let wal_trans = Table::create()
            .table(WalletTransactions::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(WalletTransactions::Id)
                    .big_integer()
                    .primary_key()
                    .not_null(),
            )
            .col(
                ColumnDef::new(WalletTransactions::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(WalletTransactions::WalletId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(WalletTransactions::TransactionReference)
                    .string()
                    .not_null()
                    .unique_key(),
            )
            .col(
                ColumnDef::new(WalletTransactions::ExternalReference)
                    .string()
                    .not_null()
                    .unique_key(),
            )
            .col(
                ColumnDef::new(WalletTransactions::TransactionDirection)
                    .custom("wallet_trans_direction")
                    .not_null(),
            )
            .col(
                ColumnDef::new(WalletTransactions::Amount)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(WalletTransactions::FeeAmount)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(WalletTransactions::Currency)
                    .json_binary()
                    .not_null(),
            )
            .col(ColumnDef::new(WalletTransactions::CounterPartyPhone).string())
            .col(ColumnDef::new(WalletTransactions::CounterPartyName).string())
            .col(ColumnDef::new(WalletTransactions::CounterPartyWalletId).string())
            .col(ColumnDef::new(WalletTransactions::TransactionType).custom("wallet_trans_type"))
            .col(ColumnDef::new(WalletTransactions::Status).custom("wallet_trans_status"))
            .col(ColumnDef::new(WalletTransactions::PostedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(WalletTransactions::BankTransactionId).big_integer())
            .col(ColumnDef::new(WalletTransactions::ProviderResponse).json_binary())
            .col(ColumnDef::new(WalletTransactions::ProviderStatusCode).string())
            .col(
                ColumnDef::new(WalletTransactions::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(WalletTransactions::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(WalletTransactions::Table, WalletTransactions::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(WalletTransactions::Table, WalletTransactions::WalletId)
                    .to(Wallets::Table, Wallets::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        WalletTransactions::Table,
                        WalletTransactions::BankTransactionId,
                    )
                    .to(Transactions::Table, Transactions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(wal_trans).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WalletTransactions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum WalletTransactions {
    Table,
    Id,
    InstitutionId,
    WalletId,
    TransactionReference,
    ExternalReference,
    TransactionDirection,
    Amount,
    FeeAmount,
    Currency,
    CounterPartyPhone,
    CounterPartyName,
    CounterPartyWalletId,
    TransactionType,
    Status,
    PostedAt,
    BankTransactionId,
    ProviderResponse,
    ProviderStatusCode,
    CreatedAt,
    UpdatedAt,
}
