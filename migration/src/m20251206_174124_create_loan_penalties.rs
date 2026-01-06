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
                "CREATE TYPE loan_penalty_type AS ENUM ('LATE_PAYMENT', 'DEFAULT')".to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE loan_penalty_status AS ENUM ('UNPAID', 'PAID', 'PROCESSING')"
                    .to_string(),
            ))
            .await?;

        let penalties = Table::create()
            .table(LoanPenalties::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(LoanPenalties::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(LoanPenalties::LoanId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanPenalties::InstallmentNumber)
                    .integer()
                    .not_null(),
            )
            .col(ColumnDef::new(LoanPenalties::PenaltyType).custom("loan_penalty_type"))
            .col(ColumnDef::new(LoanPenalties::PenaltyAmount).decimal_len(20, 4))
            .col(ColumnDef::new(LoanPenalties::CalculatedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(LoanPenalties::WaivedAmount).decimal_len(20, 4))
            .col(ColumnDef::new(LoanPenalties::WaivedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(LoanPenalties::WaiveReason).string())
            .col(ColumnDef::new(LoanPenalties::Status).custom("loan_penalty_status"))
            .col(ColumnDef::new(LoanPenalties::WaivedBy).big_integer())
            .col(
                ColumnDef::new(LoanPenalties::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(LoanPenalties::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanPenalties::Table, LoanPenalties::LoanId)
                    .to(Loans::Table, Loans::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanPenalties::Table, LoanPenalties::WaivedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(penalties).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LoanPenalties::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum LoanPenalties {
    Table,
    Id,
    LoanId,
    InstallmentNumber,
    PenaltyType,
    PenaltyAmount,
    CalculatedAt,
    WaivedAmount,
    WaivedAt,
    WaivedBy,
    WaiveReason,
    Status,
    CreatedAt,
    UpdatedAt,
}
