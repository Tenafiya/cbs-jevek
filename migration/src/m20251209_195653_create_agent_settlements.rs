use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
    m20251208_154224_create_agents::Agents,
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
                "CREATE TYPE agent_settlement_channels AS ENUM ('BANK_TRANSFER', 'MOBILE_MONEY')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE agent_settlement_status AS ENUM ('PENDING', 'PROCESSED', 'FAILED')"
                    .to_string(),
            ))
            .await?;

        let agent_settlements = Table::create()
            .table(AgentSettlements::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(AgentSettlements::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(AgentSettlements::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AgentSettlements::AgentId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AgentSettlements::SettlementCycle)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AgentSettlements::SettlementDate)
                    .date()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AgentSettlements::TotalTransactions)
                    .integer()
                    .default(0),
            )
            .col(
                ColumnDef::new(AgentSettlements::TotalTransactionVolume)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(AgentSettlements::TotalCommissionEarned)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(AgentSettlements::PreviousBalance)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(AgentSettlements::Adjustments)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(AgentSettlements::FinalSettlementAmount)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(ColumnDef::new(AgentSettlements::SettlementAccount).string())
            .col(ColumnDef::new(AgentSettlements::SettlementReference).string())
            .col(
                ColumnDef::new(AgentSettlements::SettlementChannel)
                    .custom("agent_settlement_channels"),
            )
            .col(
                ColumnDef::new(AgentSettlements::Status)
                    .custom("agent_settlement_status")
                    .default("PENDING"),
            )
            .col(
                ColumnDef::new(AgentSettlements::InitiatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(ColumnDef::new(AgentSettlements::ProcessedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(AgentSettlements::InitiatedBy).big_integer())
            .col(
                ColumnDef::new(AgentSettlements::ReconciliationStatus)
                    .custom("agent_settlement_status")
                    .default("PENDING"),
            )
            .col(ColumnDef::new(AgentSettlements::ReconciledAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(AgentSettlements::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(AgentSettlements::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AgentSettlements::Table, AgentSettlements::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AgentSettlements::Table, AgentSettlements::AgentId)
                    .to(Agents::Table, Agents::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AgentSettlements::Table, AgentSettlements::InitiatedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .index(
                Index::create()
                    .name("idx_agent_settlement_cycle")
                    .table(AgentSettlements::Table)
                    .col(AgentSettlements::AgentId)
                    .col(AgentSettlements::SettlementCycle)
                    .unique(),
            )
            .to_owned();

        manager.create_table(agent_settlements).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AgentSettlements::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AgentSettlements {
    Table,
    Id,
    InstitutionId,
    AgentId,
    SettlementCycle,
    SettlementDate,
    TotalTransactions,
    TotalTransactionVolume,
    TotalCommissionEarned,
    PreviousBalance,
    Adjustments,
    FinalSettlementAmount,
    SettlementAccount,
    SettlementReference,
    SettlementChannel,
    Status,
    InitiatedBy,
    InitiatedAt,
    ProcessedAt,
    ReconciliationStatus,
    ReconciledAt,
    CreatedAt,
    UpdatedAt,
}
