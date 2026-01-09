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
                "CREATE TYPE comm_payout_status AS ENUM ('PROCESSING', 'COMPLETED', 'FAILED')"
                    .to_string(),
            ))
            .await?;

        let commission_payouts = Table::create()
            .table(CommissionPayouts::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CommissionPayouts::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(CommissionPayouts::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CommissionPayouts::PayoutBatchName)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CommissionPayouts::PayoutCycle)
                    .string_len(50)
                    .not_null(),
            )
            .col(
                ColumnDef::new(CommissionPayouts::TotalAgents)
                    .integer()
                    .default(0),
            )
            .col(
                ColumnDef::new(CommissionPayouts::TotalStaff)
                    .integer()
                    .default(0),
            )
            .col(
                ColumnDef::new(CommissionPayouts::TotalPayoutAmount)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(CommissionPayouts::Status)
                    .custom("comm_payout_status")
                    .default("PROCESSING"),
            )
            .col(ColumnDef::new(CommissionPayouts::ProcessedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(CommissionPayouts::SettlementAccountId).big_integer())
            .col(ColumnDef::new(CommissionPayouts::SettlementReference).string())
            .col(
                ColumnDef::new(CommissionPayouts::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(CommissionPayouts::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(ColumnDef::new(CommissionPayouts::ProcessedBy).big_integer())
            .foreign_key(
                ForeignKey::create()
                    .from(CommissionPayouts::Table, CommissionPayouts::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        CommissionPayouts::Table,
                        CommissionPayouts::SettlementAccountId,
                    )
                    .to(ChartOfAccounts::Table, ChartOfAccounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(CommissionPayouts::Table, CommissionPayouts::ProcessedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(commission_payouts).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CommissionPayouts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum CommissionPayouts {
    Table,
    Id,
    InstitutionId,
    PayoutBatchName,
    PayoutCycle,
    TotalAgents,
    TotalStaff,
    TotalPayoutAmount,
    Status,
    ProcessedAt,
    SettlementAccountId,
    SettlementReference,
    ProcessedBy,
    CreatedAt,
    UpdatedAt,
}
