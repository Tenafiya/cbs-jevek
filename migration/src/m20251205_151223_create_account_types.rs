use sea_orm_migration::{prelude::*, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE acc_type_int_calc AS ENUM ('SIMPLE', 'COMPOUND', 'DAILY')".to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE acc_type_int_payout_freq AS ENUM ('MONTHLY', 'QUATERLY', 'YEARLY')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE acc_type_status AS ENUM ('ACTIVE', 'DORMANT', 'INACTIVE', 'FROZEN', 'CLOSED', 'SUSPENDED')"
                    .to_string(),
            ))
            .await?;

        let acc_types = Table::create()
            .table(AccountTypes::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(AccountTypes::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(AccountTypes::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(AccountTypes::Name).string())
            .col(ColumnDef::new(AccountTypes::Code).string().unique_key())
            .col(
                ColumnDef::new(AccountTypes::CategoryId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(AccountTypes::Description).string())
            .col(ColumnDef::new(AccountTypes::Currency).json_binary())
            .col(
                ColumnDef::new(AccountTypes::MinimumBalance)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(ColumnDef::new(AccountTypes::MaximumBalance).decimal_len(20, 4))
            .col(ColumnDef::new(AccountTypes::InterestRate).decimal_len(10, 6))
            .col(ColumnDef::new(AccountTypes::InterestRateCalcMethod).custom("acc_type_int_calc"))
            .col(ColumnDef::new(AccountTypes::KycTier).json_binary())
            .col(
                ColumnDef::new(AccountTypes::InterestPayoutFrequency)
                    .custom("acc_type_int_payout_freq"),
            )
            .col(
                ColumnDef::new(AccountTypes::IsOverdraftAllowable)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(AccountTypes::OverdraftLimit).decimal_len(20, 4))
            .col(ColumnDef::new(AccountTypes::OverdraftInterestRate).decimal_len(10, 6))
            .col(
                ColumnDef::new(AccountTypes::DormancyPeriodDays)
                    .integer()
                    .default(180),
            )
            .col(
                ColumnDef::new(AccountTypes::MaintenanceFee)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(AccountTypes::WithdrawalFee)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(AccountTypes::Status)
                    .custom("acc_type_status")
                    .default("ACTIVE"),
            )
            .col(ColumnDef::new(AccountTypes::CustomFields).json_binary())
            .col(
                ColumnDef::new(AccountTypes::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(AccountTypes::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .to_owned();

        manager.create_table(acc_types).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AccountTypes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AccountTypes {
    Table,
    Id,
    InstitutionId,
    Name,
    Code,
    CategoryId,
    Description,
    Currency,
    MinimumBalance,
    MaximumBalance,
    InterestRate,
    InterestRateCalcMethod,
    InterestPayoutFrequency,
    IsOverdraftAllowable,
    OverdraftLimit,
    OverdraftInterestRate,
    DormancyPeriodDays,
    MaintenanceFee,
    WithdrawalFee,
    Status,
    KycTier,
    CustomFields,
    CreatedAt,
    UpdatedAt,
}
