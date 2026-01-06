use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
    m20251204_152312_create_customers::Customers, m20251205_154503_create_accounts::Accounts,
    m20251205_210647_create_loan_products::LoanProducts,
    m20251206_143556_create_loan_applications::LoanApplications,
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
                "CREATE TYPE loan_repayment_freq AS ENUM ('DAILY', 'WEEKLY', 'MONTHLY', 'BULLET')"
                    .to_string(),
            ))
            .await?;

        let loans = Table::create()
            .table(Loans::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Loans::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(Loans::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(Loans::LoanApplicationId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(Loans::LoanProductId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(Loans::CustomerId).big_integer().not_null())
            .col(ColumnDef::new(Loans::AccountId).big_integer().not_null())
            .col(
                ColumnDef::new(Loans::LoanAccountNumber)
                    .string()
                    .not_null()
                    .unique_key(),
            )
            .col(
                ColumnDef::new(Loans::PrincipalAmount)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(Loans::DisbursedAmount)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(Loans::OutstandingPrincipal)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(Loans::OutstandingInterest)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(Loans::OutstandingPenalty)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(ColumnDef::new(Loans::TenureDays).integer().not_null())
            .col(
                ColumnDef::new(Loans::InterestRate)
                    .decimal_len(10, 6)
                    .not_null(),
            )
            .col(ColumnDef::new(Loans::RepaymentFreq).custom("loan_repayment_freq"))
            .col(ColumnDef::new(Loans::ApplicationDate).date().not_null())
            .col(ColumnDef::new(Loans::ApprovalDate).date())
            .col(ColumnDef::new(Loans::DisbursementDate).date())
            .col(ColumnDef::new(Loans::FirstRepaymentDate).date())
            .col(ColumnDef::new(Loans::LastRepaymentDate).date())
            .col(ColumnDef::new(Loans::MaturityDate).date().not_null())
            .col(
                ColumnDef::new(Loans::Status)
                    .custom("loan_application_status")
                    .default("DRAFT"),
            )
            .col(ColumnDef::new(Loans::IsNpa).boolean().default(false))
            .col(ColumnDef::new(Loans::NpaClassificationDate).date())
            .col(ColumnDef::new(Loans::DaysInArrears).integer().default(0))
            .col(
                ColumnDef::new(Loans::ArrearsAmount)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(ColumnDef::new(Loans::DaysToWriteOff).integer())
            .col(
                ColumnDef::new(Loans::ProvisionRate)
                    .decimal_len(10, 6)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(Loans::ProvisionAmount)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(ColumnDef::new(Loans::CustomFields).json_binary())
            .col(ColumnDef::new(Loans::CreatedBy).big_integer())
            .col(
                ColumnDef::new(Loans::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Loans::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Loans::Table, Loans::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Loans::Table, Loans::LoanApplicationId)
                    .to(LoanApplications::Table, LoanApplications::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Loans::Table, Loans::LoanProductId)
                    .to(LoanProducts::Table, LoanProducts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Loans::Table, Loans::CustomerId)
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Loans::Table, Loans::AccountId)
                    .to(Accounts::Table, Accounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Loans::Table, Loans::CreatedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(loans).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Loans::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Loans {
    Table,
    Id,
    InstitutionId,
    LoanApplicationId,
    LoanProductId,
    CustomerId,
    AccountId,
    LoanAccountNumber,
    PrincipalAmount,
    DisbursedAmount,
    OutstandingPrincipal,
    OutstandingInterest,
    OutstandingPenalty,
    TenureDays,
    InterestRate,
    RepaymentFreq,
    ApplicationDate,
    ApprovalDate,
    DisbursementDate,
    FirstRepaymentDate,
    LastRepaymentDate,
    MaturityDate,
    Status,
    IsNpa,
    NpaClassificationDate,
    DaysInArrears,
    ArrearsAmount,
    DaysToWriteOff,
    ProvisionRate,
    ProvisionAmount,
    CustomFields,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
