use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::m20251205_154503_create_accounts::Accounts;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE acc_limit_type AS ENUM ('DAILY_DEBIT', 'DAILY_CREDIT', 'DAILY_TRANSACTION_COUNT', 'WEEKLY_DEBIT', 'MONTHLY_DEBIT')".to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE acc_reset_freq_type AS ENUM ('DAILY', 'WEEKLY', 'MONTHLY')"
                    .to_string(),
            ))
            .await?;

        let acc_limits = Table::create()
            .table(AccountLimits::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(AccountLimits::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(AccountLimits::AccountId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AccountLimits::LimitType)
                    .custom("acc_limit_type")
                    .not_null(),
            )
            .col(ColumnDef::new(AccountLimits::LimitAmount).decimal_len(20, 4))
            .col(ColumnDef::new(AccountLimits::LimitCount).integer())
            .col(
                ColumnDef::new(AccountLimits::CurrentUsage)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(AccountLimits::CurrentCount)
                    .integer()
                    .default(0),
            )
            .col(
                ColumnDef::new(AccountLimits::ResetFrequency)
                    .custom("acc_reset_freq_type")
                    .default("DAILY"),
            )
            .col(
                ColumnDef::new(AccountLimits::LastResetAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(AccountLimits::IsActive)
                    .boolean()
                    .default(true),
            )
            .col(ColumnDef::new(AccountLimits::EffectiveFrom).timestamp_with_time_zone())
            .col(ColumnDef::new(AccountLimits::EffectiveTo).timestamp_with_time_zone())
            .col(
                ColumnDef::new(AccountLimits::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(AccountLimits::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AccountLimits::Table, AccountLimits::AccountId)
                    .to(Accounts::Table, Accounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(acc_limits).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AccountLimits::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AccountLimits {
    Table,
    Id,
    AccountId,
    LimitType,
    LimitAmount,
    LimitCount,
    CurrentUsage,
    CurrentCount,
    ResetFrequency,
    LastResetAt,
    IsActive,
    EffectiveFrom,
    EffectiveTo,
    CreatedAt,
    UpdatedAt,
}
