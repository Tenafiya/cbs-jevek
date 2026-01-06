use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
    m20251205_193221_create_transactions::Transactions, m20251208_154224_create_agents::Agents,
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
                "CREATE TYPE agent_commissions_trans_type AS ENUM ('DEPOSIT', 'WITHDRAWAL', 'LOAN_REPAYMENT', 'BILL_PAYMENT')".to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE agent_commissions_status AS ENUM ('PENDING', 'PAID', 'CANCELLED')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE agent_commission_rule_comm_type AS ENUM ('PERCENTAGE', 'FLAT')"
                    .to_string(),
            ))
            .await?;

        let comm_rules = Table::create()
            .table(AgentCommissionRules::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(AgentCommissionRules::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(AgentCommissionRules::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AgentCommissionRules::RuleName)
                    .string()
                    .not_null(),
            )
            .col(ColumnDef::new(AgentCommissionRules::RuleDescription).string())
            .col(ColumnDef::new(AgentCommissionRules::AppliesToTransactionTypes).json_binary())
            .col(ColumnDef::new(AgentCommissionRules::AppliesToAgents).json_binary())
            .col(
                ColumnDef::new(AgentCommissionRules::CommissionType)
                    .custom("agent_commission_rule_comm_type"),
            )
            .col(ColumnDef::new(AgentCommissionRules::CommissionValue).decimal_len(20, 4))
            .col(
                ColumnDef::new(AgentCommissionRules::HasSlabs)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(AgentCommissionRules::SlabConfig).json_binary())
            .col(
                ColumnDef::new(AgentCommissionRules::MinCommission)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(ColumnDef::new(AgentCommissionRules::MaxCommission).decimal_len(20, 4))
            .col(
                ColumnDef::new(AgentCommissionRules::IsActive)
                    .boolean()
                    .default(true),
            )
            .col(ColumnDef::new(AgentCommissionRules::EffectiveFrom).timestamp_with_time_zone())
            .col(ColumnDef::new(AgentCommissionRules::EffectiveTo).timestamp_with_time_zone())
            .col(ColumnDef::new(AgentCommissionRules::CreatedBy).big_integer())
            .col(
                ColumnDef::new(AgentCommissionRules::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(AgentCommissionRules::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        AgentCommissionRules::Table,
                        AgentCommissionRules::InstitutionId,
                    )
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AgentCommissionRules::Table, AgentCommissionRules::CreatedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(comm_rules).await?;

        let comms = Table::create()
            .table(AgentCommissions::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(AgentCommissions::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(AgentCommissions::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AgentCommissions::AgentId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AgentCommissions::CommissionRuleId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(AgentCommissions::TransactionId).big_integer())
            .col(ColumnDef::new(AgentCommissions::TransactionReference).string())
            .col(
                ColumnDef::new(AgentCommissions::TransactionAmount)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(AgentCommissions::CommissionRate)
                    .decimal_len(10, 6)
                    .not_null(),
            )
            .col(
                ColumnDef::new(AgentCommissions::CommissionAmount)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(AgentCommissions::TransactionType)
                    .custom("agent_commissions_trans_type"),
            )
            .col(ColumnDef::new(AgentCommissions::Status).custom("agent_commissions_status"))
            .col(ColumnDef::new(AgentCommissions::PaidAt).timestamp_with_time_zone())
            .col(ColumnDef::new(AgentCommissions::SettledCycle).string())
            .col(
                ColumnDef::new(AgentCommissions::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(AgentCommissions::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AgentCommissions::Table, AgentCommissions::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AgentCommissions::Table, AgentCommissions::AgentId)
                    .to(Agents::Table, Agents::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AgentCommissions::Table, AgentCommissions::TransactionId)
                    .to(Transactions::Table, Transactions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AgentCommissions::Table, AgentCommissions::CommissionRuleId)
                    .to(AgentCommissionRules::Table, AgentCommissionRules::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(comms).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AgentCommissions::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(AgentCommissionRules::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AgentCommissions {
    Table,
    Id,
    InstitutionId,
    AgentId,
    CommissionRuleId,
    TransactionId,
    TransactionReference,
    TransactionAmount,
    CommissionRate,
    CommissionAmount,
    TransactionType,
    Status,
    PaidAt,
    SettledCycle,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum AgentCommissionRules {
    Table,
    Id,
    InstitutionId,
    RuleName,
    RuleDescription,
    AppliesToTransactionTypes,
    AppliesToAgents,
    CommissionType,
    CommissionValue,
    HasSlabs,
    SlabConfig,
    MinCommission,
    MaxCommission,
    IsActive,
    EffectiveFrom,
    EffectiveTo,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
