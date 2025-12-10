use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions,
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
                "CREATE TYPE contribution_cycles_status AS ENUM ('ACTIVE', 'COMPLETED', 'CANCELLED')"
                    .to_string(),
            ))
            .await?;

        let cycles = Table::create()
            .table(ContributionCycles::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(ContributionCycles::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(ContributionCycles::SavingsProductId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(ContributionCycles::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(ContributionCycles::CycleName).string())
            .col(ColumnDef::new(ContributionCycles::StartDate).date())
            .col(ColumnDef::new(ContributionCycles::EndDate).date())
            .col(ColumnDef::new(ContributionCycles::ExpectedContributions).decimal_len(20, 4))
            .col(
                ColumnDef::new(ContributionCycles::ActualContributions)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(ColumnDef::new(ContributionCycles::Status).custom("contribution_cycles_status"))
            .col(
                ColumnDef::new(ContributionCycles::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(ContributionCycles::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        ContributionCycles::Table,
                        ContributionCycles::SavingsProductId,
                    )
                    .to(SavingsProducts::Table, SavingsProducts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(ContributionCycles::Table, ContributionCycles::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(cycles).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ContributionCycles::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ContributionCycles {
    Table,
    Id,
    SavingsProductId,
    InstitutionId,
    CycleName,
    StartDate,
    EndDate,
    ExpectedContributions,
    ActualContributions,
    Status,
    CreatedAt,
    UpdatedAt,
}
