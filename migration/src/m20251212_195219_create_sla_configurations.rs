use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::m20251204_112805_create_institutions::Institutions;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE sla_config_category AS ENUM ('TICKET_RESOLUTION', 'LOAN_APPROVAL', 'DISPUTE_RESOLUTION')"
                    .to_string(),
            ))
            .await?;

        let sla_configurations = Table::create()
            .table(SlaConfigurations::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SlaConfigurations::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(SlaConfigurations::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(SlaConfigurations::SlaName)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(SlaConfigurations::SlaCategory)
                    .custom("sla_config_category")
                    .not_null(),
            )
            .col(
                ColumnDef::new(SlaConfigurations::TargetResolutionTimeMinutes)
                    .integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(SlaConfigurations::PriorityLevels)
                    .json_binary()
                    .not_null(),
            )
            .col(ColumnDef::new(SlaConfigurations::AppliesToGroups).json_binary())
            .col(
                ColumnDef::new(SlaConfigurations::EscalationEnabled)
                    .boolean()
                    .default(true),
            )
            .col(ColumnDef::new(SlaConfigurations::EscalationRules).json_binary())
            .col(
                ColumnDef::new(SlaConfigurations::IsActive)
                    .boolean()
                    .default(true),
            )
            .col(
                ColumnDef::new(SlaConfigurations::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(SlaConfigurations::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(SlaConfigurations::Table, SlaConfigurations::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(sla_configurations).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SlaConfigurations::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SlaConfigurations {
    Table,
    Id,
    InstitutionId,
    SlaName,
    SlaCategory,
    TargetResolutionTimeMinutes,
    PriorityLevels,
    AppliesToGroups,
    EscalationEnabled,
    EscalationRules,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
