use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
    m20251204_152312_create_customers::Customers,
    m20251205_210647_create_loan_products::LoanProducts,
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
                "CREATE TYPE loan_application_status AS ENUM ('DRAFT', 'PENDING', 'APPROVED', 'REJECTED', 'DISBURSED', 'REPAID', 'WRITTEN_OFF', 'RESCHEDULED', 'REFINANCED')".to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE loan_risk_rating AS ENUM ('LOW', 'MEDIUM', 'HIGH', 'CRITICAL')"
                    .to_string(),
            ))
            .await?;

        let loan_apps = Table::create()
            .table(LoanApplications::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(LoanApplications::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(LoanApplications::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanApplications::LoanProductId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanApplications::CustomerId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanApplications::ApplicationNumber)
                    .string()
                    .unique_key(),
            )
            .col(
                ColumnDef::new(LoanApplications::Status)
                    .custom("loan_application_status")
                    .default("DRAFT"),
            )
            .col(
                ColumnDef::new(LoanApplications::RequestedPrincipal)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanApplications::RequestedTenureDays)
                    .integer()
                    .not_null(),
            )
            .col(ColumnDef::new(LoanApplications::Purpose).string())
            .col(ColumnDef::new(LoanApplications::ApprovedPrincipal).decimal_len(20, 4))
            .col(ColumnDef::new(LoanApplications::ApprovedTenureDays).integer())
            .col(ColumnDef::new(LoanApplications::ApprovedInterestRate).decimal_len(10, 6))
            .col(ColumnDef::new(LoanApplications::InstallmentAmount).decimal_len(20, 4))
            .col(ColumnDef::new(LoanApplications::CreditScore).integer())
            .col(ColumnDef::new(LoanApplications::CreditBureauResponse).json_binary())
            .col(ColumnDef::new(LoanApplications::RiskRating).custom("loan_risk_rating"))
            .col(ColumnDef::new(LoanApplications::DocumentsProvided).json_binary())
            .col(ColumnDef::new(LoanApplications::DocumentsMissing).json_binary())
            .col(ColumnDef::new(LoanApplications::ApplicationData).json_binary())
            .col(ColumnDef::new(LoanApplications::EmploymentDetails).json_binary())
            .col(ColumnDef::new(LoanApplications::FinancialDetails).json_binary())
            .col(ColumnDef::new(LoanApplications::SubmittedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(LoanApplications::CurrentStage).string())
            .col(ColumnDef::new(LoanApplications::ApprovedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(LoanApplications::ApprovalConditions).json_binary())
            .col(ColumnDef::new(LoanApplications::RejectedReason).string())
            .col(ColumnDef::new(LoanApplications::RejectedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(LoanApplications::SubmittedBy).big_integer())
            .col(ColumnDef::new(LoanApplications::AssignedOfficer).big_integer())
            .col(ColumnDef::new(LoanApplications::ApprovedBy).big_integer())
            .col(ColumnDef::new(LoanApplications::RejectedBy).big_integer())
            .col(
                ColumnDef::new(LoanApplications::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(LoanApplications::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanApplications::Table, LoanApplications::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanApplications::Table, LoanApplications::LoanProductId)
                    .to(LoanProducts::Table, LoanProducts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanApplications::Table, LoanApplications::CustomerId)
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanApplications::Table, LoanApplications::SubmittedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanApplications::Table, LoanApplications::AssignedOfficer)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanApplications::Table, LoanApplications::ApprovedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanApplications::Table, LoanApplications::RejectedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(loan_apps).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LoanApplications::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum LoanApplications {
    Table,
    Id,
    InstitutionId,
    LoanProductId,
    CustomerId,
    ApplicationNumber,
    Status,
    RequestedPrincipal,
    RequestedTenureDays,
    Purpose,
    ApprovedPrincipal,
    ApprovedTenureDays,
    ApprovedInterestRate,
    InstallmentAmount,
    CreditScore,
    CreditBureauResponse,
    RiskRating,
    DocumentsProvided,
    DocumentsMissing,
    ApplicationData,
    EmploymentDetails,
    FinancialDetails,
    SubmittedAt,
    SubmittedBy,
    CurrentStage,
    AssignedOfficer,
    ApprovedBy,
    ApprovedAt,
    ApprovalConditions,
    RejectedReason,
    RejectedAt,
    RejectedBy,
    CreatedAt,
    UpdatedAt,
}
