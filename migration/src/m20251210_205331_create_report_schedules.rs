use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
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
                "CREATE TYPE report_category AS ENUM ('PORTFOLIO', 'PERFORMANCE', 'COMPLIANCE', 'RISK')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE report_frequency AS ENUM ('DAILY', 'WEEKLY', 'MONTHLY', 'ADHOC')"
                    .to_string(),
            ))
            .await?;

        let report_schedules = Table::create()
            .table(ReportSchedules::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(ReportSchedules::Id)
                    .big_integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(
                ColumnDef::new(ReportSchedules::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(ReportSchedules::ReportName)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(ReportSchedules::ReportType)
                    .string()
                    .not_null(),
            )
            .col(ColumnDef::new(ReportSchedules::ReportCategory).custom("report_category"))
            .col(
                ColumnDef::new(ReportSchedules::Frequency)
                    .custom("report_frequency")
                    .not_null(),
            )
            .col(ColumnDef::new(ReportSchedules::ScheduleConfig).json_binary())
            .col(ColumnDef::new(ReportSchedules::NextRunAt).timestamp_with_time_zone())
            .col(ColumnDef::new(ReportSchedules::LastRunAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(ReportSchedules::Recipients)
                    .json_binary()
                    .not_null(),
            )
            .col(ColumnDef::new(ReportSchedules::ReportParameters).json_binary())
            .col(
                ColumnDef::new(ReportSchedules::IsActive)
                    .boolean()
                    .default(true),
            )
            .col(ColumnDef::new(ReportSchedules::CreatedBy).big_integer())
            .col(
                ColumnDef::new(ReportSchedules::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(ReportSchedules::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(ReportSchedules::Table, ReportSchedules::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(ReportSchedules::Table, ReportSchedules::CreatedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .to_owned();

        manager.create_table(report_schedules).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ReportSchedules::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ReportSchedules {
    Table,
    Id,
    InstitutionId,
    ReportName,
    ReportType,
    ReportCategory,
    Frequency,
    ScheduleConfig,
    NextRunAt,
    LastRunAt,
    Recipients,
    ReportParameters,
    IsActive,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
