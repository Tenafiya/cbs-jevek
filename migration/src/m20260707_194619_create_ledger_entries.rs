use sea_orm::Statement;
use sea_orm_migration::prelude::*;

use crate::{
    m20251204_112805_create_institutions::Institutions,
    m20251204_151411_create_chart_of_accounts::ChartOfAccounts,
    m20251205_154503_create_accounts::Accounts, m20251205_193221_create_transactions::Transactions,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let ledger = Table::create()
            .table(LedgerEntries::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(LedgerEntries::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(LedgerEntries::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(LedgerEntries::TransactionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(LedgerEntries::AccountId).big_integer())
            .col(ColumnDef::new(LedgerEntries::GlAccountId).big_integer())
            .col(
                ColumnDef::new(LedgerEntries::EntryType)
                    .custom("transaction_type")
                    .not_null(),
            )
            .col(
                ColumnDef::new(LedgerEntries::Amount)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(LedgerEntries::CurrencyCode)
                    .string()
                    .not_null(),
            )
            .col(ColumnDef::new(LedgerEntries::RunningBalance).big_integer())
            .col(ColumnDef::new(LedgerEntries::Description).string())
            .col(ColumnDef::new(LedgerEntries::Reference).string())
            .col(ColumnDef::new(LedgerEntries::ValueDate).timestamp())
            .col(ColumnDef::new(LedgerEntries::PostedAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(LedgerEntries::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LedgerEntries::Table, LedgerEntries::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LedgerEntries::Table, LedgerEntries::TransactionId)
                    .to(Transactions::Table, Transactions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LedgerEntries::Table, LedgerEntries::AccountId)
                    .to(Accounts::Table, Accounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LedgerEntries::Table, LedgerEntries::GlAccountId)
                    .to(ChartOfAccounts::Table, ChartOfAccounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(ledger).await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                    ALTER TABLE ledger_entries
                    ADD CONSTRAINT ledger_entry_account_check
                    CHECK (
                        account_id IS NOT NULL
                        OR gl_account_id IS NOT NULL
                    );
                "#
                .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                CREATE INDEX idx_ledger_entries_transaction ON ledger_entries(transaction_id);
            "#
                .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                CREATE INDEX idx_ledger_entries_gl_account ON ledger_entries(gl_account_id);
            "#
                .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                CREATE INDEX idx_ledger_entries_account ON ledger_entries(account_id);
            "#
                .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                CREATE INDEX idx_ledger_entries_posted_at ON ledger_entries(posted_at);
            "#
                .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                CREATE INDEX idx_ledger_entries_value_date ON ledger_entries(value_date);
            "#
                .to_string(),
            ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LedgerEntries::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum LedgerEntries {
    Table,
    Id,
    InstitutionId,
    TransactionId,
    AccountId,
    GlAccountId,
    EntryType,
    Amount,
    CurrencyCode,
    RunningBalance,
    Description,
    Reference,
    ValueDate,
    PostedAt,
    CreatedAt,
}
