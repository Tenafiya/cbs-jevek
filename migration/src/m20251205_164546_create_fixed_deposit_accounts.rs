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
                "CREATE TYPE fd_rollover_type AS ENUM ('NONE', 'PRINCIPAL', 'PRINCIPAL_AND_INTEREST')".to_string(),
            ))
            .await?;

        let fd = Table::create()
            .table(FixedDepositAccounts::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(FixedDepositAccounts::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(FixedDepositAccounts::AccountId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(FixedDepositAccounts::DepositAmount)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(FixedDepositAccounts::TenureDays)
                    .integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(FixedDepositAccounts::InterestRate)
                    .decimal_len(10, 6)
                    .not_null(),
            )
            .col(
                ColumnDef::new(FixedDepositAccounts::MaturityDate)
                    .date()
                    .not_null(),
            )
            .col(
                ColumnDef::new(FixedDepositAccounts::MaturityAmount)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(FixedDepositAccounts::RolloverType)
                    .custom("fd_rollover_type")
                    .default("NONE"),
            )
            .col(ColumnDef::new(FixedDepositAccounts::RolloverToAccountId).big_integer())
            .col(
                ColumnDef::new(FixedDepositAccounts::IsEarlyWithdrawalAllowed)
                    .boolean()
                    .default(false),
            )
            .col(
                ColumnDef::new(FixedDepositAccounts::EarlyWithdrawalPenaltyRate).decimal_len(10, 6),
            )
            .col(
                ColumnDef::new(FixedDepositAccounts::Status)
                    .custom("acc_type_status")
                    .default("ACTIVE"),
            )
            .col(
                ColumnDef::new(FixedDepositAccounts::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(FixedDepositAccounts::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(FixedDepositAccounts::Table, FixedDepositAccounts::AccountId)
                    .to(Accounts::Table, Accounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        FixedDepositAccounts::Table,
                        FixedDepositAccounts::RolloverToAccountId,
                    )
                    .to(Accounts::Table, Accounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(fd).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FixedDepositAccounts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum FixedDepositAccounts {
    Table,
    Id,
    AccountId,
    DepositAmount,
    TenureDays,
    InterestRate,
    MaturityDate,
    MaturityAmount,
    RolloverType,
    RolloverToAccountId,
    IsEarlyWithdrawalAllowed,
    EarlyWithdrawalPenaltyRate,
    Status,
    CreatedAt,
    UpdatedAt,
}
