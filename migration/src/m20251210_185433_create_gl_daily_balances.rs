use sea_orm_migration::prelude::*;

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
    m20251204_151411_create_chart_of_accounts::ChartOfAccounts,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let gl_daily_balances = Table::create()
            .table(GlDailyBalances::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(GlDailyBalances::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(GlDailyBalances::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(GlDailyBalances::AccountId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(GlDailyBalances::BalanceDate)
                    .date()
                    .not_null(),
            )
            .col(
                ColumnDef::new(GlDailyBalances::OpeningBalance)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(GlDailyBalances::TotalDebits)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(GlDailyBalances::TotalCredits)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(GlDailyBalances::TransactionCount)
                    .integer()
                    .default(0),
            )
            .col(
                ColumnDef::new(GlDailyBalances::ClosingBalance)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(GlDailyBalances::IsReconciled)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(GlDailyBalances::ReconciledBy).big_integer())
            .col(ColumnDef::new(GlDailyBalances::ReconciledAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(GlDailyBalances::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(GlDailyBalances::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(GlDailyBalances::Table, GlDailyBalances::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(GlDailyBalances::Table, GlDailyBalances::AccountId)
                    .to(ChartOfAccounts::Table, ChartOfAccounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(GlDailyBalances::Table, GlDailyBalances::ReconciledBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .index(
                Index::create()
                    .name("idx_gl_daily_balances")
                    .table(GlDailyBalances::Table)
                    .col(GlDailyBalances::InstitutionId)
                    .col(GlDailyBalances::AccountId)
                    .col(GlDailyBalances::BalanceDate)
                    .unique(),
            )
            .index(
                Index::create()
                    .name("idx_gl_balances_date")
                    .table(GlDailyBalances::Table)
                    .col(GlDailyBalances::BalanceDate),
            )
            .index(
                Index::create()
                    .name("idx_gl_balances_reconciled")
                    .table(GlDailyBalances::Table)
                    .col(GlDailyBalances::IsReconciled),
            )
            .to_owned();

        manager.create_table(gl_daily_balances).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GlDailyBalances::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum GlDailyBalances {
    Table,
    Id,
    InstitutionId,
    AccountId,
    BalanceDate,
    OpeningBalance,
    TotalDebits,
    TotalCredits,
    TransactionCount,
    ClosingBalance,
    IsReconciled,
    ReconciledBy,
    ReconciledAt,
    CreatedAt,
    UpdatedAt,
}
