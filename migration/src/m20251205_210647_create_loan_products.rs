use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
    m20251204_151411_create_chart_of_accounts::ChartOfAccounts,
    m20251205_205817_create_loan_product_types::LoanProductTypes,
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
                "CREATE TYPE loan_product_interest_type AS ENUM ('FIXED', 'VARIABLE')".to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE loan_product_calc_method AS ENUM ('FLAT', 'REDUCING_BALANCE', 'DECLINING_BALANCE')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE loan_product_freq AS ENUM ('DAILY', 'WEEKLY', 'MONTHLY', 'BULLET')"
                    .to_string(),
            ))
            .await?;

        let loan_products = Table::create()
            .table(LoanProducts::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(LoanProducts::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(LoanProducts::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanProducts::LoanProductTypeId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(LoanProducts::Name).string())
            .col(ColumnDef::new(LoanProducts::Code).string().unique_key())
            .col(ColumnDef::new(LoanProducts::Description).string())
            .col(ColumnDef::new(LoanProducts::Currency).json_binary())
            .col(
                ColumnDef::new(LoanProducts::MinimumPrincipal)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanProducts::MaximumPrincipal)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(ColumnDef::new(LoanProducts::DefaultPrincipal).decimal_len(20, 4))
            .col(
                ColumnDef::new(LoanProducts::MinimumTenureDays)
                    .integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanProducts::MaximumTenureDays)
                    .integer()
                    .not_null(),
            )
            .col(ColumnDef::new(LoanProducts::DefaultTenureDays).integer())
            .col(
                ColumnDef::new(LoanProducts::InterestRate)
                    .decimal_len(10, 6)
                    .not_null(),
            )
            .col(
                ColumnDef::new(LoanProducts::InterestRateType).custom("loan_product_interest_type"),
            )
            .col(
                ColumnDef::new(LoanProducts::InterestCalcMethod).custom("loan_product_calc_method"),
            )
            .col(ColumnDef::new(LoanProducts::InterestAccuralFreq).custom("loan_product_freq"))
            .col(
                ColumnDef::new(LoanProducts::ProcessingFeeRate)
                    .decimal_len(10, 6)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(LoanProducts::ProcessingFeeFlat)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(LoanProducts::InsuranceFeeRate)
                    .decimal_len(10, 6)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(LoanProducts::InsuranceFeeFlat)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(LoanProducts::LatePaymentPenaltyRate)
                    .decimal_len(10, 6)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(LoanProducts::LatePaymentPenaltyFlat)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(LoanProducts::PenaltyGracePeriodDays)
                    .integer()
                    .default(3),
            )
            .col(
                ColumnDef::new(LoanProducts::RepaymentFreq)
                    .string()
                    .not_null(),
            )
            .col(ColumnDef::new(LoanProducts::AllowedRepaymentMethods).json_binary())
            .col(ColumnDef::new(LoanProducts::MinimumCreditScore).integer())
            .col(ColumnDef::new(LoanProducts::MaxAge).integer())
            .col(ColumnDef::new(LoanProducts::MinAge).integer())
            .col(ColumnDef::new(LoanProducts::AllowedCustomerTypes).json_binary())
            .col(
                ColumnDef::new(LoanProducts::IsCollateralRequired)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(LoanProducts::MinimumCollateralRatio).decimal_len(10, 6))
            .col(ColumnDef::new(LoanProducts::AllowedCollateralTypes).json_binary())
            .col(
                ColumnDef::new(LoanProducts::IsGuarantorRequired)
                    .boolean()
                    .default(false),
            )
            .col(
                ColumnDef::new(LoanProducts::MinimumGuarantors)
                    .integer()
                    .default(0),
            )
            .col(
                ColumnDef::new(LoanProducts::IsActive)
                    .boolean()
                    .default(true),
            )
            .col(ColumnDef::new(LoanProducts::Visibility).json_binary())
            .col(ColumnDef::new(LoanProducts::LoanGlAccountId).big_integer())
            .col(ColumnDef::new(LoanProducts::InterestGlAccountId).big_integer())
            .col(ColumnDef::new(LoanProducts::PenaltyGlAccountId).big_integer())
            .col(ColumnDef::new(LoanProducts::RequiredKyc).json_binary())
            .col(ColumnDef::new(LoanProducts::CreatedBy).big_integer())
            .col(
                ColumnDef::new(LoanProducts::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(LoanProducts::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanProducts::Table, LoanProducts::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanProducts::Table, LoanProducts::LoanProductTypeId)
                    .to(LoanProductTypes::Table, LoanProductTypes::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanProducts::Table, LoanProducts::LoanGlAccountId)
                    .to(ChartOfAccounts::Table, ChartOfAccounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanProducts::Table, LoanProducts::InterestGlAccountId)
                    .to(ChartOfAccounts::Table, ChartOfAccounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanProducts::Table, LoanProducts::PenaltyGlAccountId)
                    .to(ChartOfAccounts::Table, ChartOfAccounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanProducts::Table, LoanProducts::CreatedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(loan_products).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LoanProducts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum LoanProducts {
    Table,
    Id,
    InstitutionId,
    LoanProductTypeId,
    Name,
    Code,
    Description,
    Currency,
    MinimumPrincipal,
    MaximumPrincipal,
    DefaultPrincipal,
    MinimumTenureDays,
    MaximumTenureDays,
    DefaultTenureDays,
    InterestRate,
    InterestRateType,
    InterestCalcMethod,
    InterestAccuralFreq,
    ProcessingFeeRate,
    ProcessingFeeFlat,
    InsuranceFeeRate,
    InsuranceFeeFlat,
    LatePaymentPenaltyRate,
    LatePaymentPenaltyFlat,
    PenaltyGracePeriodDays,
    RepaymentFreq,
    AllowedRepaymentMethods,
    MinimumCreditScore,
    MinAge,
    MaxAge,
    AllowedCustomerTypes,
    RequiredKyc,
    IsCollateralRequired,
    MinimumCollateralRatio,
    AllowedCollateralTypes,
    IsGuarantorRequired,
    MinimumGuarantors,
    IsActive,
    Visibility,
    LoanGlAccountId,
    InterestGlAccountId,
    PenaltyGlAccountId,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
