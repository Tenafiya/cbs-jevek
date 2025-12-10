use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions,
    m20251204_152312_create_customers::Customers, m20251205_154503_create_accounts::Accounts,
    m20251205_193221_create_transactions::Transactions,
    m20251207_103023_create_saving_goals::SavingGoals,
    m20251207_131906_create_contribution_cycles::ContributionCycles,
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
                "CREATE TYPE contribution_type AS ENUM ('REGULAR', 'PENALTY', 'BONUS')".to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE contribution_status AS ENUM ('PENDING', 'COMPLETED', 'CANCELLED')"
                    .to_string(),
            ))
            .await?;

        let contributions = Table::create()
            .table(Contributions::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Contributions::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(Contributions::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(Contributions::SavingGoalId).big_integer())
            .col(
                ColumnDef::new(Contributions::CustomerId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(Contributions::ContributionCycleId).big_integer())
            .col(ColumnDef::new(Contributions::AccountId).big_integer())
            .col(ColumnDef::new(Contributions::TransactionId).big_integer())
            .col(ColumnDef::new(Contributions::ContributionDate).date())
            .col(
                ColumnDef::new(Contributions::Amount)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(ColumnDef::new(Contributions::ContributionType).custom("contribution_type"))
            .col(ColumnDef::new(Contributions::ContributionReference).string())
            .col(
                ColumnDef::new(Contributions::IsMissedContribution)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(Contributions::MissedContributionDate).date())
            .col(
                ColumnDef::new(Contributions::PenaltyApplied)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(ColumnDef::new(Contributions::Status).custom("contribution_status"))
            .col(
                ColumnDef::new(Contributions::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Contributions::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Contributions::Table, Contributions::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Contributions::Table, Contributions::SavingGoalId)
                    .to(SavingGoals::Table, SavingGoals::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Contributions::Table, Contributions::CustomerId)
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Contributions::Table, Contributions::ContributionCycleId)
                    .to(ContributionCycles::Table, ContributionCycles::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Contributions::Table, Contributions::AccountId)
                    .to(Accounts::Table, Accounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Contributions::Table, Contributions::TransactionId)
                    .to(Transactions::Table, Transactions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(contributions).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Contributions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Contributions {
    Table,
    Id,
    InstitutionId,
    SavingGoalId,
    CustomerId,
    ContributionCycleId,
    AccountId,
    TransactionId,
    ContributionDate,
    Amount,
    ContributionType,
    ContributionReference,
    IsMissedContribution,
    MissedContributionDate,
    PenaltyApplied,
    Status,
    CreatedAt,
    UpdatedAt,
}
