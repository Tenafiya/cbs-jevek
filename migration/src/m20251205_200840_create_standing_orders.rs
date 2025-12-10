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
                "CREATE TYPE standing_orders_freq AS ENUM ('DAILY', 'WEEKLY', 'MONTHLY', 'QUARTERLY')"
                    .to_string(),
            ))
            .await?;

        let stand = Table::create()
            .table(StandingOrders::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(StandingOrders::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(StandingOrders::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(StandingOrders::CustomerId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(StandingOrders::DebitAccountId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(StandingOrders::CreditAccountId).big_integer())
            .col(ColumnDef::new(StandingOrders::BeneficiaryName).string())
            .col(ColumnDef::new(StandingOrders::BeneficiaryAccount).string())
            .col(ColumnDef::new(StandingOrders::BeneficiaryBankCode).string())
            .col(ColumnDef::new(StandingOrders::Amount).decimal_len(20, 4))
            .col(ColumnDef::new(StandingOrders::Currency).json_binary())
            .col(ColumnDef::new(StandingOrders::Frequency).custom("standing_orders_freq"))
            .col(ColumnDef::new(StandingOrders::DayOfWeek).integer())
            .col(ColumnDef::new(StandingOrders::DayOfMonth).integer())
            .col(ColumnDef::new(StandingOrders::StartDate).date())
            .col(ColumnDef::new(StandingOrders::EndDate).date())
            .col(ColumnDef::new(StandingOrders::NextRunDate).date())
            .col(ColumnDef::new(StandingOrders::Narration).string())
            .col(
                ColumnDef::new(StandingOrders::Status)
                    .custom("acc_type_status")
                    .default("ACTIVE"),
            )
            .col(ColumnDef::new(StandingOrders::FailureCount).integer())
            .col(
                ColumnDef::new(StandingOrders::MaxFailureCount)
                    .integer()
                    .default(3),
            )
            .col(
                ColumnDef::new(StandingOrders::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(StandingOrders::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .to_owned();

        manager.create_table(stand).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StandingOrders::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum StandingOrders {
    Table,
    Id,
    InstitutionId,
    CustomerId,
    DebitAccountId,
    CreditAccountId,
    BeneficiaryName,
    BeneficiaryAccount,
    BeneficiaryBankCode,
    Amount,
    Currency,
    Frequency,
    DayOfWeek,
    DayOfMonth,
    StartDate,
    EndDate,
    NextRunDate,
    Narration,
    Status,
    FailureCount,
    MaxFailureCount,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
