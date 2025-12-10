use sea_orm_migration::prelude::*;

use crate::m20251206_150936_create_loans::Loans;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let write_off = Table::create()
            .table(LoanWriteOffs::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(LoanWriteOffs::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(LoanWriteOffs::LoanId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanWriteOffs::WriteOffAmount)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanWriteOffs::OutstandingPrincipal)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanWriteOffs::OutstandingInterest)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanWriteOffs::OutstandingPenalty)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(ColumnDef::new(LoanWriteOffs::WriteOffReason).string())
            .col(ColumnDef::new(LoanWriteOffs::ProvisionAmount).decimal_len(20, 4))
            .col(
                ColumnDef::new(LoanWriteOffs::WrittenOffAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(LoanWriteOffs::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(LoanWriteOffs::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanWriteOffs::Table, LoanWriteOffs::LoanId)
                    .to(Loans::Table, Loans::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(write_off).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LoanWriteOffs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum LoanWriteOffs {
    Table,
    Id,
    LoanId,
    WriteOffAmount,
    OutstandingPrincipal,
    OutstandingInterest,
    OutstandingPenalty,
    WriteOffReason,
    ProvisionAmount,
    WrittenOffBy,
    WrittenOffAt,
    CreatedAt,
    UpdatedAt,
}
