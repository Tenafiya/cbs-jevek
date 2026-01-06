use sea_orm_migration::prelude::*;

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
    m20251206_150936_create_loans::Loans,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let provisioning = Table::create()
            .table(LoanProvisioning::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(LoanProvisioning::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(LoanProvisioning::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanProvisioning::LoanId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(LoanProvisioning::ProvisionRate).decimal_len(10, 6))
            .col(ColumnDef::new(LoanProvisioning::ProvisionDate).date())
            .col(ColumnDef::new(LoanProvisioning::ProvisionAmount).decimal_len(20, 4))
            .col(ColumnDef::new(LoanProvisioning::OutstandingBalance).decimal_len(20, 4))
            .col(ColumnDef::new(LoanProvisioning::DaysInArrears).integer())
            .col(ColumnDef::new(LoanProvisioning::CalculationMethod).string())
            .col(ColumnDef::new(LoanProvisioning::CreatedBy).big_integer())
            .col(
                ColumnDef::new(LoanProvisioning::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(LoanProvisioning::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanProvisioning::Table, LoanProvisioning::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanProvisioning::Table, LoanProvisioning::LoanId)
                    .to(Loans::Table, Loans::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanProvisioning::Table, LoanProvisioning::CreatedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(provisioning).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LoanProvisioning::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum LoanProvisioning {
    Table,
    Id,
    InstitutionId,
    LoanId,
    ProvisionRate,
    ProvisionDate,
    ProvisionAmount,
    OutstandingBalance,
    DaysInArrears,
    CalculationMethod,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
