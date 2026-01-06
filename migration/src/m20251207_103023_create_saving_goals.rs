use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions,
    m20251204_152312_create_customers::Customers, m20251205_154503_create_accounts::Accounts,
    m20251206_193123_create_savings_products::SavingsProducts,
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
                "CREATE TYPE saving_goals_status AS ENUM ('ACTIVE', 'PAUSED', 'COMPLETED', 'ABANDONED')"
                    .to_string(),
            ))
            .await?;

        let save_goals = Table::create()
            .table(SavingGoals::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SavingGoals::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(SavingGoals::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(SavingGoals::CustomerId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(SavingGoals::SavingsProductId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(SavingGoals::AccountId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(SavingGoals::GoalName).string())
            .col(ColumnDef::new(SavingGoals::TargetAmount).decimal_len(20, 4))
            .col(
                ColumnDef::new(SavingGoals::CurrentAmount)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(ColumnDef::new(SavingGoals::StartDate).date().not_null())
            .col(ColumnDef::new(SavingGoals::TargetCompletionDate).date())
            .col(ColumnDef::new(SavingGoals::ContributionAmount).decimal_len(20, 4))
            .col(ColumnDef::new(SavingGoals::ContributionFreq).custom("savings_product_freq"))
            .col(ColumnDef::new(SavingGoals::Status).custom("saving_goals_status"))
            .col(ColumnDef::new(SavingGoals::CompletionDate).date())
            .col(ColumnDef::new(SavingGoals::ProgressPercentage).integer())
            .col(
                ColumnDef::new(SavingGoals::IsGroupSavings)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(SavingGoals::GroupOwnerId).big_integer())
            .col(ColumnDef::new(SavingGoals::CustomFields).json_binary())
            .col(
                ColumnDef::new(SavingGoals::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(SavingGoals::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(SavingGoals::Table, SavingGoals::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(SavingGoals::Table, SavingGoals::CustomerId)
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(SavingGoals::Table, SavingGoals::SavingsProductId)
                    .to(SavingsProducts::Table, SavingsProducts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(SavingGoals::Table, SavingGoals::AccountId)
                    .to(Accounts::Table, Accounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(SavingGoals::Table, SavingGoals::GroupOwnerId)
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(save_goals).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SavingGoals::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SavingGoals {
    Table,
    Id,
    InstitutionId,
    CustomerId,
    SavingsProductId,
    AccountId,
    GoalName,
    TargetAmount,
    CurrentAmount,
    ProgressPercentage,
    StartDate,
    TargetCompletionDate,
    ContributionAmount,
    ContributionFreq,
    Status,
    CompletionDate,
    IsGroupSavings,
    GroupOwnerId,
    CustomFields,
    CreatedAt,
    UpdatedAt,
}
