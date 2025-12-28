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
                "CREATE TYPE gen_report_status AS ENUM ('GENERATING', 'COMPLETED', 'FAILED')"
                    .to_string(),
            ))
            .await?;

        let regulatory_reporting_exports = Table::create()
            .table(RegulatoryReportingExports::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(RegulatoryReportingExports::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(RegulatoryReportingExports::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(RegulatoryReportingExports::ExportName)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(RegulatoryReportingExports::ExportType)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(RegulatoryReportingExports::ExportFormat)
                    .string()
                    .not_null(),
            )
            .col(ColumnDef::new(RegulatoryReportingExports::ExportData).json_binary())
            .col(ColumnDef::new(RegulatoryReportingExports::FilePath).text())
            .col(ColumnDef::new(RegulatoryReportingExports::FileUrl).text())
            .col(ColumnDef::new(RegulatoryReportingExports::FileSizeBytes).integer())
            .col(
                ColumnDef::new(RegulatoryReportingExports::ExportStatus)
                    .custom("gen_report_status")
                    .default("GENERATING"),
            )
            .col(ColumnDef::new(RegulatoryReportingExports::GeneratedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(RegulatoryReportingExports::ApprovedBy).big_integer())
            .col(ColumnDef::new(RegulatoryReportingExports::ApprovedAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(RegulatoryReportingExports::SubmittedToRegulator)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(RegulatoryReportingExports::RegulatorResponse).json_binary())
            .col(ColumnDef::new(RegulatoryReportingExports::SubmittedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(RegulatoryReportingExports::CreatedBy).big_integer())
            .col(
                ColumnDef::new(RegulatoryReportingExports::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(RegulatoryReportingExports::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        RegulatoryReportingExports::Table,
                        RegulatoryReportingExports::InstitutionId,
                    )
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        RegulatoryReportingExports::Table,
                        RegulatoryReportingExports::ApprovedBy,
                    )
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        RegulatoryReportingExports::Table,
                        RegulatoryReportingExports::CreatedBy,
                    )
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .index(
                Index::create()
                    .name("idx_regulatory_exports_institution")
                    .table(RegulatoryReportingExports::Table)
                    .col(RegulatoryReportingExports::InstitutionId),
            )
            .index(
                Index::create()
                    .name("idx_regulatory_exports_type")
                    .table(RegulatoryReportingExports::Table)
                    .col(RegulatoryReportingExports::ExportType),
            )
            .index(
                Index::create()
                    .name("idx_regulatory_exports_status")
                    .table(RegulatoryReportingExports::Table)
                    .col(RegulatoryReportingExports::ExportStatus),
            )
            .index(
                Index::create()
                    .name("idx_regulatory_exports_submitted")
                    .table(RegulatoryReportingExports::Table)
                    .col(RegulatoryReportingExports::SubmittedToRegulator),
            )
            .to_owned();

        manager.create_table(regulatory_reporting_exports).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(RegulatoryReportingExports::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum RegulatoryReportingExports {
    Table,
    Id,
    InstitutionId,
    ExportName,
    ExportType,
    ExportFormat,
    ExportData,
    FilePath,
    FileUrl,
    FileSizeBytes,
    ExportStatus,
    GeneratedAt,
    ApprovedBy,
    ApprovedAt,
    SubmittedToRegulator,
    RegulatorResponse,
    SubmittedAt,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
