use sea_orm_migration::prelude::*;

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
    m20251205_193221_create_transactions::Transactions,
    m20251208_163720_create_agent_commissions::AgentCommissionRules,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let staff_commissions = Table::create()
            .table(StaffCommissions::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(StaffCommissions::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(StaffCommissions::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(StaffCommissions::StaffId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(StaffCommissions::CommissionRuleId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(StaffCommissions::TransactionId).big_integer())
            .col(ColumnDef::new(StaffCommissions::TransactionReference).string())
            .col(
                ColumnDef::new(StaffCommissions::TransactionAmount)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(StaffCommissions::CommissionRate)
                    .decimal_len(10, 6)
                    .not_null(),
            )
            .col(
                ColumnDef::new(StaffCommissions::CommissionAmount)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(StaffCommissions::TransactionType)
                    .custom("agent_commissions_trans_type"),
            )
            .col(ColumnDef::new(StaffCommissions::Status).custom("agent_commissions_status"))
            .col(ColumnDef::new(StaffCommissions::PaidAt).timestamp_with_time_zone())
            .col(ColumnDef::new(StaffCommissions::PayrollCycle).string())
            .col(
                ColumnDef::new(StaffCommissions::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(StaffCommissions::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(StaffCommissions::Table, StaffCommissions::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(StaffCommissions::Table, StaffCommissions::StaffId)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(StaffCommissions::Table, StaffCommissions::TransactionId)
                    .to(Transactions::Table, Transactions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(StaffCommissions::Table, StaffCommissions::CommissionRuleId)
                    .to(AgentCommissionRules::Table, AgentCommissionRules::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .index(
                Index::create()
                    .name("idx_staff_commission_staff")
                    .table(StaffCommissions::Table)
                    .col(StaffCommissions::StaffId),
            )
            .index(
                Index::create()
                    .name("idx_staff_commission_status")
                    .table(StaffCommissions::Table)
                    .col(StaffCommissions::Status),
            )
            .to_owned();

        manager.create_table(staff_commissions).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StaffCommissions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum StaffCommissions {
    Table,
    Id,
    InstitutionId,
    StaffId,
    CommissionRuleId,
    TransactionId,
    TransactionReference,
    TransactionAmount,
    CommissionRate,
    CommissionAmount,
    TransactionType,
    Status,
    PaidAt,
    PayrollCycle,
    CreatedAt,
    UpdatedAt,
}
