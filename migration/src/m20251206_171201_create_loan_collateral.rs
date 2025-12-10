use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251206_143556_create_loan_applications::LoanApplications,
    m20251206_150936_create_loans::Loans,
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
                "CREATE TYPE loan_collateral_types AS ENUM ('LAND', 'BUILDING', 'VEHICLE', 'EQUIPMENT', 'INVENTORY', 'CASH')"
                    .to_string(),
            ))
            .await?;

        let collateral = Table::create()
            .table(LoanCollaterals::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(LoanCollaterals::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(LoanCollaterals::LoanApplicationId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(LoanCollaterals::LoanId).big_integer())
            .col(ColumnDef::new(LoanCollaterals::CollateralType).custom("loan_collateral_types"))
            .col(ColumnDef::new(LoanCollaterals::Description).string())
            .col(
                ColumnDef::new(LoanCollaterals::EstimatedValue)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanCollaterals::ValuationDate)
                    .date()
                    .not_null(),
            )
            .col(ColumnDef::new(LoanCollaterals::DocumentUrls).json_binary())
            .col(ColumnDef::new(LoanCollaterals::OwnershipDocuments).json_binary())
            .col(
                ColumnDef::new(LoanCollaterals::IsVerified)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(LoanCollaterals::VerifiedAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(LoanCollaterals::IsReleased)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(LoanCollaterals::ReleasedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(LoanCollaterals::ReleaseReason).string())
            .col(
                ColumnDef::new(LoanCollaterals::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(LoanCollaterals::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanCollaterals::Table, LoanCollaterals::LoanApplicationId)
                    .to(LoanApplications::Table, LoanApplications::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanCollaterals::Table, LoanCollaterals::LoanId)
                    .to(Loans::Table, Loans::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(collateral).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LoanCollaterals::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum LoanCollaterals {
    Table,
    Id,
    LoanApplicationId,
    LoanId,
    CollateralType,
    Description,
    EstimatedValue,
    ValuationDate,
    DocumentUrls,
    OwnershipDocuments,
    IsVerified,
    VerifiedBy,
    VerifiedAt,
    IsReleased,
    ReleasedAt,
    ReleaseReason,
    CreatedAt,
    UpdatedAt,
}
