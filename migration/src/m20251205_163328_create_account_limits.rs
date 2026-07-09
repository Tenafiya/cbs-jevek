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
                "CREATE TYPE acc_limit_type AS ENUM ('DAILY_DEBIT', 'DAILY_CREDIT', 'DAILY_COUNT', 'WEEKLY_DEBIT', 'WEEKLY_CREDIT', 'WEEKLY_COUNT', 'MONTHLY_DEBIT', 'MONTHLY_CREDIT', 'MONTHLY_COUNT')".to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE acc_limit_unit AS ENUM ('AMOUNT', 'COUNT')".to_string(),
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
            .col(
                ColumnDef::new(AccountLimits::LimitUnit)
                    .custom("acc_limit_unit")
                    .not_null(),
            )
            .col(
                ColumnDef::new(AccountLimits::LimitValue)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AccountLimits::CurrentValue)
                    .big_integer()
                    .not_null(),
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
            .col(
                ColumnDef::new(AccountLimits::EffectiveFrom)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AccountLimits::EffectiveTo)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
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

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                    ALTER TABLE account_limits
                    ADD CONSTRAINT unique_acc_limit_acc_id_limit_type
                    UNIQUE (account_id, limit_type);
                "#
                .to_string(),
            ))
            .await?;

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
    LimitUnit,
    LimitValue,
    CurrentValue,
    LastResetAt,
    IsActive,
    EffectiveFrom,
    EffectiveTo,
    CreatedAt,
    UpdatedAt,
}
