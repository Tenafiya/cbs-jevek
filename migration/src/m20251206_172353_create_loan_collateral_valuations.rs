use sea_orm_migration::prelude::*;

use crate::m20251206_171201_create_loan_collateral::LoanCollaterals;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let valuations = Table::create()
            .table(LoanCollateralValuations::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(LoanCollateralValuations::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(LoanCollateralValuations::CollateralId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanCollateralValuations::ValuationAmount)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanCollateralValuations::ValuationDate)
                    .date()
                    .not_null(),
            )
            .col(ColumnDef::new(LoanCollateralValuations::ValuatorName).string())
            .col(ColumnDef::new(LoanCollateralValuations::ValuationReportUrl).string())
            .col(
                ColumnDef::new(LoanCollateralValuations::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(LoanCollateralValuations::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        LoanCollateralValuations::Table,
                        LoanCollateralValuations::CollateralId,
                    )
                    .to(LoanCollaterals::Table, LoanCollaterals::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(valuations).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(LoanCollateralValuations::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum LoanCollateralValuations {
    Table,
    Id,
    CollateralId,
    ValuationAmount,
    ValuationDate,
    ValuatorName,
    ValuationReportUrl,
    CreatedAt,
    UpdatedAt,
}
