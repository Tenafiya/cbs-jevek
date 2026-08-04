use sea_orm_migration::prelude::*;

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
    m20251210_205331_create_report_schedules::ReportSchedules,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE TYPE gen_reports_format AS ENUM ('PDF', 'EXCEL', 'CSV', 'JSON')
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE TYPE gen_reports_dist_Status AS ENUM ('PENDING', 'PROCESSED', 'COMPLETED', 'FAILED')
                "#,
            )
            .await?;

        let generated_reports = Table::create()
            .table(GeneratedReports::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(GeneratedReports::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(GeneratedReports::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(GeneratedReports::ScheduleId).big_integer())
            .col(
                ColumnDef::new(GeneratedReports::ReportName)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(GeneratedReports::ReportType)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(GeneratedReports::GeneratedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(ColumnDef::new(GeneratedReports::GeneratedBy).big_integer())
            .col(
                ColumnDef::new(GeneratedReports::ReportPeriodStart)
                    .date()
                    .not_null(),
            )
            .col(
                ColumnDef::new(GeneratedReports::ReportPeriodEnd)
                    .date()
                    .not_null(),
            )
            .col(ColumnDef::new(GeneratedReports::ReportData).json_binary())
            .col(ColumnDef::new(GeneratedReports::ReportDataSizeBytes).integer())
            .col(ColumnDef::new(GeneratedReports::FileFormat).custom("gen_reports_format"))
            .col(ColumnDef::new(GeneratedReports::FilePath).string())
            .col(ColumnDef::new(GeneratedReports::FileUrl).string())
            .col(ColumnDef::new(GeneratedReports::FileChecksum).string())
            .col(ColumnDef::new(GeneratedReports::DistributedTo).json_binary())
            .col(
                ColumnDef::new(GeneratedReports::DistributionStatus)
                    .custom("gen_reports_dist_Status")
                    .default("PENDING"),
            )
            .col(
                ColumnDef::new(GeneratedReports::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(GeneratedReports::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(GeneratedReports::Table, GeneratedReports::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(GeneratedReports::Table, GeneratedReports::ScheduleId)
                    .to(ReportSchedules::Table, ReportSchedules::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(GeneratedReports::Table, GeneratedReports::GeneratedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .to_owned();

        manager.create_table(generated_reports).await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE INDEX idx_generated_reports_period ON generated_reports(report_period_start, report_period_end);
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE INDEX idx_generated_reports_type ON generated_reports(report_type);
                "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GeneratedReports::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum GeneratedReports {
    Table,
    Id,
    InstitutionId,
    ScheduleId,
    ReportName,
    ReportType,
    GeneratedAt,
    GeneratedBy,
    ReportPeriodStart,
    ReportPeriodEnd,
    ReportData,
    ReportDataSizeBytes,
    FileFormat,
    FilePath,
    FileUrl,
    FileChecksum,
    DistributedTo,
    DistributionStatus,
    CreatedAt,
    UpdatedAt,
}
