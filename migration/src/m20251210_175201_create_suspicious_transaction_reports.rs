use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
    m20251204_152312_create_customers::Customers, m20251205_154503_create_accounts::Accounts,
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
                "CREATE TYPE transaction_suspicion_type AS ENUM ('MONEY_LAUNDERING', 'TERRORIST_FINANCING', 'FRAUD')".to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE transaction_suspicions_status AS ENUM ('DRAFT', 'SUBMITTED', 'ACKNOWLEDGED', 'UNDER_REVIEW')".to_string(),
            ))
            .await?;

        let suspicious_transaction_reports = Table::create()
            .table(SuspiciousTransactionReports::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SuspiciousTransactionReports::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(SuspiciousTransactionReports::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(SuspiciousTransactionReports::ReportReference)
                    .string()
                    .not_null()
                    .unique_key(),
            )
            .col(
                ColumnDef::new(SuspiciousTransactionReports::FilingDate)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(SuspiciousTransactionReports::FiledBy)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(SuspiciousTransactionReports::CustomerId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(SuspiciousTransactionReports::AccountId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(SuspiciousTransactionReports::TransactionIds).custom("TEXT"), // Store as JSON array of UUID strings
            )
            .col(
                ColumnDef::new(SuspiciousTransactionReports::TotalAmount)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(SuspiciousTransactionReports::SuspicionType)
                    .custom("transaction_suspicion_type")
                    .not_null(),
            )
            .col(
                ColumnDef::new(SuspiciousTransactionReports::SuspicionDescription)
                    .string()
                    .not_null(),
            )
            .col(ColumnDef::new(SuspiciousTransactionReports::RedFlags).json_binary())
            .col(
                ColumnDef::new(SuspiciousTransactionReports::IsFiled)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(SuspiciousTransactionReports::FilledReference).string())
            .col(ColumnDef::new(SuspiciousTransactionReports::FiledAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(SuspiciousTransactionReports::Status)
                    .custom("transaction_suspicions_status")
                    .default("DRAFT"),
            )
            .col(
                ColumnDef::new(SuspiciousTransactionReports::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(SuspiciousTransactionReports::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        SuspiciousTransactionReports::Table,
                        SuspiciousTransactionReports::InstitutionId,
                    )
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        SuspiciousTransactionReports::Table,
                        SuspiciousTransactionReports::FiledBy,
                    )
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        SuspiciousTransactionReports::Table,
                        SuspiciousTransactionReports::CustomerId,
                    )
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        SuspiciousTransactionReports::Table,
                        SuspiciousTransactionReports::AccountId,
                    )
                    .to(Accounts::Table, Accounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .index(
                Index::create()
                    .name("uk_str_report_reference")
                    .table(SuspiciousTransactionReports::Table)
                    .col(SuspiciousTransactionReports::ReportReference)
                    .unique(),
            )
            .index(
                Index::create()
                    .name("idx_str_status")
                    .table(SuspiciousTransactionReports::Table)
                    .col(SuspiciousTransactionReports::Status),
            )
            .index(
                Index::create()
                    .name("idx_str_customer")
                    .table(SuspiciousTransactionReports::Table)
                    .col(SuspiciousTransactionReports::CustomerId),
            )
            .index(
                Index::create()
                    .name("idx_str_filed_to_fiu")
                    .table(SuspiciousTransactionReports::Table)
                    .col(SuspiciousTransactionReports::IsFiled),
            )
            .to_owned();

        manager.create_table(suspicious_transaction_reports).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(SuspiciousTransactionReports::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum SuspiciousTransactionReports {
    Table,
    Id,
    InstitutionId,
    ReportReference,
    FilingDate,
    FiledBy,
    CustomerId,
    AccountId,
    TransactionIds,
    TotalAmount,
    SuspicionType,
    SuspicionDescription,
    RedFlags,
    IsFiled,
    FilledReference,
    FiledAt,
    Status,
    CreatedAt,
    UpdatedAt,
}
