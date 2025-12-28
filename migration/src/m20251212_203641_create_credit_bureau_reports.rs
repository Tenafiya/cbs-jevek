use sea_orm_migration::prelude::*;

use crate::{
    m20251204_112805_create_institutions::Institutions,
    m20251204_152312_create_customers::Customers,
    m20251206_143556_create_loan_applications::LoanApplications,
    m20251212_195551_create_integration_providers::IntegrationProviders,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let credit_bureau_reports = Table::create()
            .table(CreditBureauReports::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CreditBureauReports::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(CreditBureauReports::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CreditBureauReports::ProviderId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CreditBureauReports::CustomerId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CreditBureauReports::InquiryType)
                    .string()
                    .not_null(),
            )
            .col(ColumnDef::new(CreditBureauReports::LoanApplicationId).big_integer())
            .col(ColumnDef::new(CreditBureauReports::BureauCustomerId).string())
            .col(ColumnDef::new(CreditBureauReports::ReportData).json_binary())
            .col(ColumnDef::new(CreditBureauReports::CreditScore).integer())
            .col(ColumnDef::new(CreditBureauReports::RiskGrade).string())
            .col(
                ColumnDef::new(CreditBureauReports::NumberOfInquiries)
                    .integer()
                    .default(0),
            )
            .col(
                ColumnDef::new(CreditBureauReports::InquiriesLast30Days)
                    .integer()
                    .default(0),
            )
            .col(
                ColumnDef::new(CreditBureauReports::TotalAccounts)
                    .integer()
                    .default(0),
            )
            .col(
                ColumnDef::new(CreditBureauReports::ActiveAccounts)
                    .integer()
                    .default(0),
            )
            .col(
                ColumnDef::new(CreditBureauReports::DefaultsCount)
                    .integer()
                    .default(0),
            )
            .col(
                ColumnDef::new(CreditBureauReports::TotalOutstandingDefaultAmount)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(CreditBureauReports::Status)
                    .string()
                    .default("SUCCESS"),
            )
            .col(ColumnDef::new(CreditBureauReports::ErrorCode).string())
            .col(ColumnDef::new(CreditBureauReports::ErrorMessage).text())
            .col(
                ColumnDef::new(CreditBureauReports::ConsentProvided)
                    .boolean()
                    .default(true),
            )
            .col(ColumnDef::new(CreditBureauReports::ConsentDate).timestamp_with_time_zone())
            .col(
                ColumnDef::new(CreditBureauReports::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(CreditBureauReports::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        CreditBureauReports::Table,
                        CreditBureauReports::InstitutionId,
                    )
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(CreditBureauReports::Table, CreditBureauReports::ProviderId)
                    .to(IntegrationProviders::Table, IntegrationProviders::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(CreditBureauReports::Table, CreditBureauReports::CustomerId)
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        CreditBureauReports::Table,
                        CreditBureauReports::LoanApplicationId,
                    )
                    .to(LoanApplications::Table, LoanApplications::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .index(
                Index::create()
                    .name("idx_credit_bureau_reports_customer")
                    .table(CreditBureauReports::Table)
                    .col(CreditBureauReports::CustomerId),
            )
            .index(
                Index::create()
                    .name("idx_credit_bureau_reports_provider")
                    .table(CreditBureauReports::Table)
                    .col(CreditBureauReports::ProviderId),
            )
            .index(
                Index::create()
                    .name("idx_credit_bureau_reports_status")
                    .table(CreditBureauReports::Table)
                    .col(CreditBureauReports::Status),
            )
            .index(
                Index::create()
                    .name("idx_credit_bureau_reports_loan_app")
                    .table(CreditBureauReports::Table)
                    .col(CreditBureauReports::LoanApplicationId),
            )
            .index(
                Index::create()
                    .name("idx_credit_bureau_reports_inquiry_type")
                    .table(CreditBureauReports::Table)
                    .col(CreditBureauReports::InquiryType),
            )
            .to_owned();

        manager.create_table(credit_bureau_reports).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CreditBureauReports::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum CreditBureauReports {
    Table,
    Id,
    InstitutionId,
    ProviderId,
    CustomerId,
    InquiryType,
    LoanApplicationId,
    BureauCustomerId,
    ReportData,
    CreditScore,
    RiskGrade,
    NumberOfInquiries,
    InquiriesLast30Days,
    TotalAccounts,
    ActiveAccounts,
    DefaultsCount,
    TotalOutstandingDefaultAmount,
    Status,
    ErrorCode,
    ErrorMessage,
    ConsentProvided,
    ConsentDate,
    CreatedAt,
    UpdatedAt,
}
