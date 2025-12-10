use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251205_154503_create_accounts::Accounts,
    m20251205_193221_create_transactions::Transactions, m20251206_150936_create_loans::Loans,
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
                "CREATE TYPE loan_repayment_methods AS ENUM ('MOBILE_MONEY', 'BANK_TRANSFER', 'CASH', 'SALARY_DEDUCTION')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE loan_repayment_status AS ENUM ('PENDING', 'PROCESSING', 'COMPLETED', 'FAILED', 'CANCELLED')"
                    .to_string(),
            ))
            .await?;

        let repay = Table::create()
            .table(LoanRepayments::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(LoanRepayments::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(LoanRepayments::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanRepayments::LoanId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanRepayments::AccountId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanRepayments::TransactionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanRepayments::InstallmentNumber)
                    .integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanRepayments::PrincipalAmount)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanRepayments::InterestAmount)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanRepayments::PenaltyAmount)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(LoanRepayments::TotalAmount)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(ColumnDef::new(LoanRepayments::PaymentMethod).custom("loan_repayment_methods"))
            .col(ColumnDef::new(LoanRepayments::PaymentReference).string())
            .col(
                ColumnDef::new(LoanRepayments::RepaymentDate)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(ColumnDef::new(LoanRepayments::ValueDate).date())
            .col(
                ColumnDef::new(LoanRepayments::Status)
                    .custom("loan_repayment_status")
                    .default("COMPLETED"),
            )
            .col(
                ColumnDef::new(LoanRepayments::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(LoanRepayments::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanRepayments::Table, LoanRepayments::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanRepayments::Table, LoanRepayments::LoanId)
                    .to(Loans::Table, Loans::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanRepayments::Table, LoanRepayments::AccountId)
                    .to(Accounts::Table, Accounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanRepayments::Table, LoanRepayments::TransactionId)
                    .to(Transactions::Table, Transactions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(repay).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LoanRepayments::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum LoanRepayments {
    Table,
    Id,
    InstitutionId,
    LoanId,
    AccountId,
    TransactionId,
    InstallmentNumber,
    PrincipalAmount,
    InterestAmount,
    PenaltyAmount,
    TotalAmount,
    PaymentMethod,
    PaymentReference,
    RepaymentDate,
    ValueDate,
    Status,
    CreatedAt,
    UpdatedAt,
}
