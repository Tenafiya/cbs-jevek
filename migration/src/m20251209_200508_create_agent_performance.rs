use sea_orm_migration::prelude::*;

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251208_154224_create_agents::Agents,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let agent_performance = Table::create()
            .table(AgentPerformance::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(AgentPerformance::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(AgentPerformance::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AgentPerformance::AgentId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AgentPerformance::ReportDate)
                    .date()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AgentPerformance::DepositCount)
                    .integer()
                    .default(0),
            )
            .col(
                ColumnDef::new(AgentPerformance::DepositAmount)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(AgentPerformance::WithdrawalCount)
                    .integer()
                    .default(0),
            )
            .col(
                ColumnDef::new(AgentPerformance::WithdrawalAmount)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(AgentPerformance::LoanRepaymentCount)
                    .integer()
                    .default(0),
            )
            .col(
                ColumnDef::new(AgentPerformance::LoanRepaymentAmount)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(AgentPerformance::CommissionEarned)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(AgentPerformance::ActiveCustomerCount)
                    .integer()
                    .default(0),
            )
            .col(ColumnDef::new(AgentPerformance::PerformanceScore).decimal_len(5, 2))
            .col(
                ColumnDef::new(AgentPerformance::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(AgentPerformance::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AgentPerformance::Table, AgentPerformance::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AgentPerformance::Table, AgentPerformance::AgentId)
                    .to(Agents::Table, Agents::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .index(
                Index::create()
                    .name("idx_agent_performance_date")
                    .table(AgentPerformance::Table)
                    .col(AgentPerformance::AgentId)
                    .col(AgentPerformance::ReportDate)
                    .unique(),
            )
            .to_owned();

        manager.create_table(agent_performance).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AgentPerformance::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AgentPerformance {
    Table,
    Id,
    InstitutionId,
    AgentId,
    ReportDate,
    DepositCount,
    DepositAmount,
    WithdrawalCount,
    WithdrawalAmount,
    LoanRepaymentCount,
    LoanRepaymentAmount,
    CommissionEarned,
    ActiveCustomerCount,
    PerformanceScore,
    CreatedAt,
    UpdatedAt,
}
