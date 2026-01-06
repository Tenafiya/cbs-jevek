use sea_orm_migration::prelude::*;

use crate::{
    m20251204_150208_create_branches::Staff,
    m20251206_143556_create_loan_applications::LoanApplications,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let app_history = Table::create()
            .table(LoanApplicationStatusHistory::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(LoanApplicationStatusHistory::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(LoanApplicationStatusHistory::LoanApplicationId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(LoanApplicationStatusHistory::FromStatus).string())
            .col(ColumnDef::new(LoanApplicationStatusHistory::ToStatus).string())
            .col(ColumnDef::new(LoanApplicationStatusHistory::TransitionReason).json_binary())
            .col(ColumnDef::new(LoanApplicationStatusHistory::ChangedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(LoanApplicationStatusHistory::ChangedBy).big_integer())
            .col(
                ColumnDef::new(LoanApplicationStatusHistory::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(LoanApplicationStatusHistory::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        LoanApplicationStatusHistory::Table,
                        LoanApplicationStatusHistory::LoanApplicationId,
                    )
                    .to(LoanApplications::Table, LoanApplications::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        LoanApplicationStatusHistory::Table,
                        LoanApplicationStatusHistory::ChangedBy,
                    )
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(app_history).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(LoanApplicationStatusHistory::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum LoanApplicationStatusHistory {
    Table,
    Id,
    LoanApplicationId,
    FromStatus,
    ToStatus,
    TransitionReason,
    ChangedBy,
    ChangedAt,
    CreatedAt,
    UpdatedAt,
}
