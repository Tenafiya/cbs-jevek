use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
    m20251204_151411_create_chart_of_accounts::ChartOfAccounts,
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
                "CREATE TYPE accrual_reference_type AS ENUM ('LOAN', 'SAVINGS', 'INVESTMENT')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE accrual_type AS ENUM ('INTEREST_ACCRUAL', 'FEE_ACCRUAL', 'PROVISION')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE accrual_status AS ENUM ('ACCRUED', 'POSTED', 'REVERSED')".to_string(),
            ))
            .await?;

        let accruals_and_provisions = Table::create()
            .table(AccrualsAndProvisions::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(AccrualsAndProvisions::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(AccrualsAndProvisions::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AccrualsAndProvisions::AccrualType)
                    .custom("accrual_type")
                    .not_null(),
            )
            .col(
                ColumnDef::new(AccrualsAndProvisions::AccountId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AccrualsAndProvisions::AccrualDate)
                    .date()
                    .not_null(),
            )
            .col(ColumnDef::new(AccrualsAndProvisions::PostingDate).timestamp_with_time_zone())
            .col(
                ColumnDef::new(AccrualsAndProvisions::AccrualAmount)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(AccrualsAndProvisions::ReversalAmount)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(AccrualsAndProvisions::ReferenceType)
                    .custom("accrual_reference_type"),
            )
            .col(ColumnDef::new(AccrualsAndProvisions::ReferenceId).big_integer())
            .col(
                ColumnDef::new(AccrualsAndProvisions::Status)
                    .custom("accrual_status")
                    .default("ACCRUED"),
            )
            .col(ColumnDef::new(AccrualsAndProvisions::PostedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(AccrualsAndProvisions::CreatedBy).big_integer())
            .col(
                ColumnDef::new(AccrualsAndProvisions::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(AccrualsAndProvisions::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        AccrualsAndProvisions::Table,
                        AccrualsAndProvisions::InstitutionId,
                    )
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        AccrualsAndProvisions::Table,
                        AccrualsAndProvisions::AccountId,
                    )
                    .to(ChartOfAccounts::Table, ChartOfAccounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        AccrualsAndProvisions::Table,
                        AccrualsAndProvisions::CreatedBy,
                    )
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(accruals_and_provisions).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AccrualsAndProvisions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AccrualsAndProvisions {
    Table,
    Id,
    InstitutionId,
    AccrualType,
    AccountId,
    AccrualDate,
    PostingDate,
    AccrualAmount,
    ReversalAmount,
    ReferenceType,
    ReferenceId,
    Status,
    PostedAt,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
