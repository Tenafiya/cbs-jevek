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
                "CREATE TYPE regu_reports_report_type AS ENUM ('CAPITAL_ADEQUACY', 'LIQUIDITY', 'ASSET_QUALITY')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE regu_reports_status AS ENUM ('DRAFT', 'FINAL', 'SUBMITTED')"
                    .to_string(),
            ))
            .await?;

        let regulatory_reports = Table::create()
            .table(RegulatoryReports::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(RegulatoryReports::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(RegulatoryReports::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(RegulatoryReports::ReportName)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(RegulatoryReports::ReportType)
                    .custom("regu_reports_report_type")
                    .not_null(),
            )
            .col(
                ColumnDef::new(RegulatoryReports::ReportingPeriodStart)
                    .date()
                    .not_null(),
            )
            .col(
                ColumnDef::new(RegulatoryReports::ReportingPeriodEnd)
                    .date()
                    .not_null(),
            )
            .col(
                ColumnDef::new(RegulatoryReports::ReportData)
                    .json_binary()
                    .not_null(),
            )
            .col(
                ColumnDef::new(RegulatoryReports::ReportFormat)
                    .string()
                    .default("JSON"),
            )
            .col(
                ColumnDef::new(RegulatoryReports::Status)
                    .custom("regu_reports_status")
                    .default("DRAFT"),
            )
            .col(
                ColumnDef::new(RegulatoryReports::PreparedBy)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(RegulatoryReports::ReviewedBy).big_integer())
            .col(ColumnDef::new(RegulatoryReports::ApprovedBy).big_integer())
            .col(ColumnDef::new(RegulatoryReports::SubmittedAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(RegulatoryReports::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(RegulatoryReports::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(RegulatoryReports::Table, RegulatoryReports::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(RegulatoryReports::Table, RegulatoryReports::PreparedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(RegulatoryReports::Table, RegulatoryReports::ReviewedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(RegulatoryReports::Table, RegulatoryReports::ApprovedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .index(
                Index::create()
                    .name("idx_regulatory_reports_type")
                    .table(RegulatoryReports::Table)
                    .col(RegulatoryReports::ReportType),
            )
            .index(
                Index::create()
                    .name("idx_regulatory_reports_status")
                    .table(RegulatoryReports::Table)
                    .col(RegulatoryReports::Status),
            )
            .index(
                Index::create()
                    .name("idx_regulatory_reports_period")
                    .table(RegulatoryReports::Table)
                    .col(RegulatoryReports::ReportingPeriodStart)
                    .col(RegulatoryReports::ReportingPeriodEnd),
            )
            .to_owned();

        manager.create_table(regulatory_reports).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RegulatoryReports::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum RegulatoryReports {
    Table,
    Id,
    InstitutionId,
    ReportName,
    ReportType,
    ReportingPeriodStart,
    ReportingPeriodEnd,
    ReportData,
    ReportFormat,
    Status,
    PreparedBy,
    ReviewedBy,
    ApprovedBy,
    SubmittedAt,
    CreatedAt,
    UpdatedAt,
}
