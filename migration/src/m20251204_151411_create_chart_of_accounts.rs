use sea_orm_migration::prelude::*;

use crate::m20251204_112805_create_institutions::Institutions;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let ch_accs = Table::create()
            .table(ChartOfAccounts::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(ChartOfAccounts::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(ChartOfAccounts::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(ChartOfAccounts::AccountCode)
                    .string()
                    .unique_key(),
            )
            .col(ColumnDef::new(ChartOfAccounts::AccountName).string())
            .col(ColumnDef::new(ChartOfAccounts::AccountType).string())
            .col(ColumnDef::new(ChartOfAccounts::ParentAccountId).big_integer())
            .col(
                ColumnDef::new(ChartOfAccounts::IsActive)
                    .boolean()
                    .default(false),
            )
            .col(
                ColumnDef::new(ChartOfAccounts::IsSystemAccount)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(ChartOfAccounts::CurrencyCode).string())
            .col(
                ColumnDef::new(ChartOfAccounts::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(ChartOfAccounts::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(ChartOfAccounts::Table, ChartOfAccounts::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(ch_accs).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ChartOfAccounts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ChartOfAccounts {
    Table,
    Id,
    InstitutionId,
    AccountCode,
    AccountName,
    AccountType,
    ParentAccountId,
    IsActive,
    IsSystemAccount,
    CurrencyCode,
    CreatedAt,
    UpdatedAt,
}
