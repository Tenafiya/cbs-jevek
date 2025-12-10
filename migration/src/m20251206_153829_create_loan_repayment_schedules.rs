use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::m20251206_150936_create_loans::Loans;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE loan_repayment_schedule_status AS ENUM ('PENDING', 'PARTIAL', 'PAID', 'OVERDUE')"
                    .to_string(),
            ))
            .await?;

        let schedule = Table::create()
            .table(LoanRepaymentSchedules::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(LoanRepaymentSchedules::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(LoanRepaymentSchedules::LoanId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanRepaymentSchedules::InstallmentNumber)
                    .integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanRepaymentSchedules::DueDate)
                    .date()
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanRepaymentSchedules::PrincipalDue)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanRepaymentSchedules::InterestDue)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanRepaymentSchedules::TotalDue)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanRepaymentSchedules::PrincipalPaid)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(LoanRepaymentSchedules::InterestPaid)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(LoanRepaymentSchedules::PenaltyPaid)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(LoanRepaymentSchedules::Status)
                    .custom("loan_repayment_schedule_status"),
            )
            .col(
                ColumnDef::new(LoanRepaymentSchedules::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(LoanRepaymentSchedules::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        LoanRepaymentSchedules::Table,
                        LoanRepaymentSchedules::LoanId,
                    )
                    .to(Loans::Table, Loans::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(schedule).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(LoanRepaymentSchedules::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum LoanRepaymentSchedules {
    Table,
    Id,
    LoanId,
    InstallmentNumber,
    DueDate,
    PrincipalDue,
    InterestDue,
    TotalDue,
    PrincipalPaid,
    InterestPaid,
    PenaltyPaid,
    Status,
    CreatedAt,
    UpdatedAt,
}
