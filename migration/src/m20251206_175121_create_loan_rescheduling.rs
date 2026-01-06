use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{m20251204_150208_create_branches::Staff, m20251206_150936_create_loans::Loans};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE loan_rescheduling_status AS ENUM ('PENDING', 'ONGOING', 'COMPLETED')"
                    .to_string(),
            ))
            .await?;

        let reschedule = Table::create()
            .table(LoanRescheduling::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(LoanRescheduling::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(LoanRescheduling::LoanId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanRescheduling::OldMaturityDate)
                    .date()
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanRescheduling::NewdMaturityDate)
                    .date()
                    .not_null(),
            )
            .col(ColumnDef::new(LoanRescheduling::OldInstallmentAmount).decimal_len(20, 4))
            .col(ColumnDef::new(LoanRescheduling::NewInstallmentAmount).decimal_len(20, 4))
            .col(ColumnDef::new(LoanRescheduling::Reason).string())
            .col(ColumnDef::new(LoanRescheduling::SupportingDocuments).json_binary())
            .col(ColumnDef::new(LoanRescheduling::Status).custom("loan_rescheduling_status"))
            .col(
                ColumnDef::new(LoanRescheduling::RequestedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(ColumnDef::new(LoanRescheduling::ApprovedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(LoanRescheduling::RequestedBy).big_integer())
            .col(ColumnDef::new(LoanRescheduling::ApprovedBy).big_integer())
            .col(
                ColumnDef::new(LoanRescheduling::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(LoanRescheduling::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanRescheduling::Table, LoanRescheduling::Id)
                    .to(Loans::Table, Loans::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanRescheduling::Table, LoanRescheduling::ApprovedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(reschedule).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LoanRescheduling::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum LoanRescheduling {
    Table,
    Id,
    LoanId,
    OldMaturityDate,
    NewdMaturityDate,
    OldInstallmentAmount,
    NewInstallmentAmount,
    Reason,
    SupportingDocuments,
    Status,
    RequestedBy,
    RequestedAt,
    ApprovedBy,
    ApprovedAt,
    CreatedAt,
    UpdatedAt,
}
