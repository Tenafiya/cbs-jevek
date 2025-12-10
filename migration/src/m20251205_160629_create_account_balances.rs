use sea_orm_migration::prelude::*;

use crate::m20251205_154503_create_accounts::Accounts;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let bal = Table::create()
            .table(AccountBalances::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(AccountBalances::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(AccountBalances::AccountId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AccountBalances::BalanceDate)
                    .date()
                    .unique_key(),
            )
            .col(
                ColumnDef::new(AccountBalances::OpeningBalance)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(AccountBalances::TotalCredits)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(AccountBalances::TotalDebits)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(AccountBalances::ClosingBalance)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(AccountBalances::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(AccountBalances::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AccountBalances::Table, AccountBalances::AccountId)
                    .to(Accounts::Table, Accounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(bal).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AccountBalances::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AccountBalances {
    Table,
    Id,
    AccountId,
    BalanceDate,
    OpeningBalance,
    TotalDebits,
    TotalCredits,
    ClosingBalance,
    CreatedAt,
    UpdatedAt,
}
