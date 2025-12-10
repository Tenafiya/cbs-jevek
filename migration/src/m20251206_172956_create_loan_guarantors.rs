use sea_orm_migration::prelude::*;

use crate::{
    m20251204_152312_create_customers::Customers,
    m20251206_143556_create_loan_applications::LoanApplications,
    m20251206_150936_create_loans::Loans,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let guarantors = Table::create()
            .table(LoanGuarantors::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(LoanGuarantors::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(LoanGuarantors::LoanApplicationId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(LoanGuarantors::LoanId).big_integer())
            .col(ColumnDef::new(LoanGuarantors::CustomerId).big_integer())
            .col(ColumnDef::new(LoanGuarantors::GuarantorName).string())
            .col(ColumnDef::new(LoanGuarantors::GuarantorIdNumber).string())
            .col(ColumnDef::new(LoanGuarantors::GuarantorPhone).string())
            .col(ColumnDef::new(LoanGuarantors::GuarantorEmail).string())
            .col(ColumnDef::new(LoanGuarantors::GuarantorAddress).string())
            .col(ColumnDef::new(LoanGuarantors::GuarantorRelationship).string())
            .col(ColumnDef::new(LoanGuarantors::GuarantorOccupation).string())
            .col(ColumnDef::new(LoanGuarantors::GuarantorIncome).decimal_len(20, 4))
            .col(ColumnDef::new(LoanGuarantors::AgreementDocumentUrl).string())
            .col(
                ColumnDef::new(LoanGuarantors::IsConsentProvided)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(LoanGuarantors::ConsentDate).timestamp_with_time_zone())
            .col(
                ColumnDef::new(LoanGuarantors::IsActive)
                    .boolean()
                    .default(true),
            )
            .col(ColumnDef::new(LoanGuarantors::AcceptedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(LoanGuarantors::ReleasedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(LoanGuarantors::ReleaseReason).string())
            .col(
                ColumnDef::new(LoanGuarantors::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(LoanGuarantors::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanGuarantors::Table, LoanGuarantors::LoanApplicationId)
                    .to(LoanApplications::Table, LoanApplications::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanGuarantors::Table, LoanGuarantors::LoanId)
                    .to(Loans::Table, Loans::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanGuarantors::Table, LoanGuarantors::CustomerId)
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(guarantors).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LoanGuarantors::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum LoanGuarantors {
    Table,
    Id,
    LoanApplicationId,
    LoanId,
    CustomerId,
    GuarantorName,
    GuarantorIdNumber,
    GuarantorPhone,
    GuarantorEmail,
    GuarantorAddress,
    GuarantorRelationship,
    GuarantorIncome,
    GuarantorOccupation,
    AgreementDocumentUrl,
    IsConsentProvided,
    ConsentDate,
    IsActive,
    AcceptedAt,
    ReleasedAt,
    ReleaseReason,
    CreatedAt,
    UpdatedAt,
}
