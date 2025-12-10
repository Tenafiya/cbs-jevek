use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions,
    m20251204_151411_create_chart_of_accounts::ChartOfAccounts,
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
                "CREATE TYPE savings_product_type AS ENUM ('TARGET', 'DAILY', 'WEEKLY', 'MONTHLY', 'ROTATIONAL', 'GROUP')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE savings_product_freq AS ENUM ('DAILY', 'WEEKLY', 'MONTHLY')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE savings_product_payout_freq AS ENUM ('MONTHLY', 'QUARTERLY', 'YEARLY')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE savings_product_interest_calc AS ENUM (
                    'SIMPLE',
                    'COMPOUND_ANNUAL',
                    'COMPOUND_SEMI_ANNUAL',
                    'COMPOUND_QUARTERLY',
                    'COMPOUND_MONTHLY',
                    'COMPOUND_DAILY',
                    'CONTINUOUS',
                    'DAILY_BALANCE',
                    'AVERAGE_DAILY_BALANCE',
                    'MINIMUM_BALANCE',
                    'TIERED',
                    'BONUS',
                    'CONDITIONAL',
                    'FIXED_SIMPLE',
                    'FIXED_COMPOUND',
                    'FIXED_PAYOUT',
                    'PROFIT_SHARING',
                    'HYBRID'
                )"
                .to_string(),
            ))
            .await?;

        let save_prods = Table::create()
            .table(SavingsProducts::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SavingsProducts::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(SavingsProducts::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(SavingsProducts::Name).string())
            .col(ColumnDef::new(SavingsProducts::Code).string().unique_key())
            .col(ColumnDef::new(SavingsProducts::Description).string())
            .col(ColumnDef::new(SavingsProducts::SavingsType).custom("savings_product_type"))
            .col(ColumnDef::new(SavingsProducts::Currency).json_binary())
            .col(
                ColumnDef::new(SavingsProducts::MinContribution)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(ColumnDef::new(SavingsProducts::MaxContribution).decimal_len(20, 4))
            .col(ColumnDef::new(SavingsProducts::DefaultContribution).decimal_len(20, 4))
            .col(ColumnDef::new(SavingsProducts::ContributionFreq).custom("savings_product_freq"))
            .col(ColumnDef::new(SavingsProducts::InterestRate).decimal_len(10, 6))
            .col(
                ColumnDef::new(SavingsProducts::InterestCalcMethod)
                    .custom("savings_product_interest_calc"),
            )
            .col(
                ColumnDef::new(SavingsProducts::InterestPayoutFreq)
                    .custom("savings_product_payout_freq"),
            )
            .col(
                ColumnDef::new(SavingsProducts::LockPeriodDays)
                    .integer()
                    .default(0),
            )
            .col(
                ColumnDef::new(SavingsProducts::WithdrawalRestrictionDays)
                    .integer()
                    .default(0),
            )
            .col(
                ColumnDef::new(SavingsProducts::IsEarlyWithdrawalAllowed)
                    .boolean()
                    .default(true),
            )
            .col(
                ColumnDef::new(SavingsProducts::EarlyWithdrawalPenaltyRate)
                    .decimal_len(10, 6)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(SavingsProducts::IsTargetAmountEnabled)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(SavingsProducts::MinTargetAmount).decimal_len(20, 4))
            .col(ColumnDef::new(SavingsProducts::MaxTargetAmount).decimal_len(20, 4))
            .col(
                ColumnDef::new(SavingsProducts::IsGroupSavingsAllowed)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(SavingsProducts::MinGroupMembers).integer())
            .col(ColumnDef::new(SavingsProducts::MaxGroupMembers).integer())
            .col(
                ColumnDef::new(SavingsProducts::MissedContributionPenalty)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(SavingsProducts::MaintenanceFee)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(SavingsProducts::IsActive)
                    .boolean()
                    .default(true),
            )
            .col(
                ColumnDef::new(SavingsProducts::RequiresApproval)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(SavingsProducts::GlAccountId).big_integer())
            .col(
                ColumnDef::new(SavingsProducts::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(SavingsProducts::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(SavingsProducts::Table, SavingsProducts::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(SavingsProducts::Table, SavingsProducts::GlAccountId)
                    .to(ChartOfAccounts::Table, ChartOfAccounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(save_prods).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SavingsProducts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SavingsProducts {
    Table,
    Id,
    InstitutionId,
    Name,
    Code,
    Description,
    SavingsType,
    Currency,
    MinContribution,
    MaxContribution,
    DefaultContribution,
    ContributionFreq,
    InterestRate,
    InterestCalcMethod,
    InterestPayoutFreq,
    LockPeriodDays,
    WithdrawalRestrictionDays,
    IsEarlyWithdrawalAllowed,
    EarlyWithdrawalPenaltyRate,
    IsTargetAmountEnabled,
    MinTargetAmount,
    MaxTargetAmount,
    IsGroupSavingsAllowed,
    MinGroupMembers,
    MaxGroupMembers,
    MissedContributionPenalty,
    MaintenanceFee,
    IsActive,
    RequiresApproval,
    GlAccountId,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
