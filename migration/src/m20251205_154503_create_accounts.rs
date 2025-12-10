use sea_orm_migration::prelude::*;

use crate::{
    m20251204_112805_create_institutions::Institutions,
    m20251204_152312_create_customers::Customers,
    m20251205_151223_create_account_types::AccountTypes,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let accs = Table::create()
            .table(Accounts::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Accounts::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(Accounts::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(Accounts::CustomerId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(Accounts::AccountTypeId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(Accounts::AccountNumber)
                    .string()
                    .unique_key(),
            )
            .col(ColumnDef::new(Accounts::AccountName).string())
            .col(ColumnDef::new(Accounts::Currency).json_binary())
            .col(
                ColumnDef::new(Accounts::CurrentBalance)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(Accounts::AvailableBalance)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(Accounts::LedgerBalance)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(Accounts::HoldBalance)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(Accounts::Status)
                    .custom("acc_type_status")
                    .default("ACTIVE"),
            )
            .col(ColumnDef::new(Accounts::ActivationDate).timestamp_with_time_zone())
            .col(ColumnDef::new(Accounts::DormancyDate).timestamp_with_time_zone())
            .col(ColumnDef::new(Accounts::FrozenAt).timestamp_with_time_zone())
            .col(ColumnDef::new(Accounts::FrozenReason).string())
            .col(
                ColumnDef::new(Accounts::IsOverdraftAllowable)
                    .boolean()
                    .default(false),
            )
            .col(
                ColumnDef::new(Accounts::OverdraftLimit)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(Accounts::OverdraftUsed)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(ColumnDef::new(Accounts::ParentAccountId).big_integer())
            .col(ColumnDef::new(Accounts::Tags).array(ColumnType::Text))
            .col(ColumnDef::new(Accounts::CustomFields).json_binary())
            .col(ColumnDef::new(Accounts::ClosureReason).string())
            .col(ColumnDef::new(Accounts::ClosedAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(Accounts::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Accounts::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Accounts::Table, Accounts::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Accounts::Table, Accounts::CustomerId)
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Accounts::Table, Accounts::AccountTypeId)
                    .to(AccountTypes::Table, AccountTypes::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Accounts::Table, Accounts::ParentAccountId)
                    .to(Accounts::Table, Accounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(accs).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Accounts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Accounts {
    Table,
    Id,
    InstitutionId,
    CustomerId,
    AccountTypeId,
    AccountNumber,
    AccountName,
    Currency,
    CurrentBalance,
    AvailableBalance,
    LedgerBalance,
    HoldBalance,
    Status,
    ActivationDate,
    DormancyDate,
    FrozenAt,
    FrozenReason,
    FrozenBy,
    IsOverdraftAllowable,
    OverdraftLimit,
    OverdraftUsed,
    ParentAccountId,
    Tags,
    CustomFields,
    CreatedBy,
    ClosedBy,
    ClosureReason,
    ClosedAt,
    CreatedAt,
    UpdatedAt,
}
