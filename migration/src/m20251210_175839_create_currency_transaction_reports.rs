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
                "CREATE TYPE currency_report_type AS ENUM ('DAILY', 'WEEKLY', 'MONTHLY')"
                    .to_string(),
            ))
            .await?;

        let currency_transaction_reports = Table::create()
            .table(CurrencyTransactionReports::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CurrencyTransactionReports::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(CurrencyTransactionReports::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CurrencyTransactionReports::ReportDate)
                    .date()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CurrencyTransactionReports::ReportType)
                    .custom("currency_report_type")
                    .not_null(),
            )
            .col(
                ColumnDef::new(CurrencyTransactionReports::TotalCashTransactions)
                    .integer()
                    .default(0),
            )
            .col(
                ColumnDef::new(CurrencyTransactionReports::TotalCashAmount)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(ColumnDef::new(CurrencyTransactionReports::ThresholdBreaches).json_binary())
            .col(
                ColumnDef::new(CurrencyTransactionReports::IsFiledToRegulator)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(CurrencyTransactionReports::RegulatorReference).string())
            .col(ColumnDef::new(CurrencyTransactionReports::FiledAt).timestamp_with_time_zone())
            .col(ColumnDef::new(CurrencyTransactionReports::CreatedBy).big_integer())
            .col(
                ColumnDef::new(CurrencyTransactionReports::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(CurrencyTransactionReports::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        CurrencyTransactionReports::Table,
                        CurrencyTransactionReports::InstitutionId,
                    )
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        CurrencyTransactionReports::Table,
                        CurrencyTransactionReports::CreatedBy,
                    )
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .index(
                Index::create()
                    .name("idx_currency_reports_date_type")
                    .table(CurrencyTransactionReports::Table)
                    .col(CurrencyTransactionReports::InstitutionId)
                    .col(CurrencyTransactionReports::ReportDate)
                    .col(CurrencyTransactionReports::ReportType)
                    .unique(),
            )
            .index(
                Index::create()
                    .name("idx_currency_reports_type")
                    .table(CurrencyTransactionReports::Table)
                    .col(CurrencyTransactionReports::ReportType),
            )
            .index(
                Index::create()
                    .name("idx_currency_reports_filed")
                    .table(CurrencyTransactionReports::Table)
                    .col(CurrencyTransactionReports::IsFiledToRegulator),
            )
            .to_owned();

        manager.create_table(currency_transaction_reports).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(CurrencyTransactionReports::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum CurrencyTransactionReports {
    Table,
    Id,
    InstitutionId,
    ReportDate,
    ReportType,
    TotalCashTransactions,
    TotalCashAmount,
    ThresholdBreaches,
    IsFiledToRegulator,
    RegulatorReference,
    FiledAt,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
