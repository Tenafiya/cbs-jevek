use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251208_154224_create_agents::Agents,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the custom enum type
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE risk_level_enum AS ENUM ('LOW', 'MEDIUM', 'HIGH')".to_string(),
            ))
            .await?;

        let agent_audits = Table::create()
            .table(AgentAudits::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(AgentAudits::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(AgentAudits::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AgentAudits::AgentId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(AgentAudits::AuditDate).date().not_null())
            .col(
                ColumnDef::new(AgentAudits::AuditorId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(AgentAudits::ExpectedFloatBalance).decimal_len(20, 4))
            .col(ColumnDef::new(AgentAudits::ActualFloatBalance).decimal_len(20, 4))
            .col(ColumnDef::new(AgentAudits::Variance).decimal_len(20, 4))
            .col(ColumnDef::new(AgentAudits::Findings).json_binary())
            .col(ColumnDef::new(AgentAudits::ComplianceScore).decimal_len(5, 2))
            .col(ColumnDef::new(AgentAudits::RiskLevel).custom("risk_level_enum"))
            .col(ColumnDef::new(AgentAudits::ActionsTaken).string())
            .col(
                ColumnDef::new(AgentAudits::IsFollowUpRequired)
                    .boolean()
                    .default(false),
            )
            .col(
                ColumnDef::new(AgentAudits::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(AgentAudits::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AgentAudits::Table, AgentAudits::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AgentAudits::Table, AgentAudits::AgentId)
                    .to(Agents::Table, Agents::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .index(
                Index::create()
                    .name("idx_agent_audit_date")
                    .table(AgentAudits::Table)
                    .col(AgentAudits::AgentId)
                    .col(AgentAudits::AuditDate)
                    .unique(),
            )
            .to_owned();

        manager.create_table(agent_audits).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AgentAudits::Table).to_owned())
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "DROP TYPE IF EXISTS risk_level_enum".to_string(),
            ))
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum AgentAudits {
    Table,
    Id,
    InstitutionId,
    AgentId,
    AuditDate,
    AuditorId,
    ExpectedFloatBalance,
    ActualFloatBalance,
    Variance,
    Findings,
    ComplianceScore,
    RiskLevel,
    ActionsTaken,
    IsFollowUpRequired,
    CreatedAt,
    UpdatedAt,
}
