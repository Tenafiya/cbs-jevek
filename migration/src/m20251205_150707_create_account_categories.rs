use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::m20251204_112805_create_institutions::Institutions;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE acc_category_type AS ENUM ('SAVINGS', 'CURRENT', 'FIXED_DEPOSIT', 'LOAN', 'WALLET', 'AGENT_FLOAT', 'SUSU')"
                    .to_string(),
            ))
            .await?;

        let cats = Table::create()
            .table(AccountCategories::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(AccountCategories::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(AccountCategories::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(AccountCategories::Name).string())
            .col(ColumnDef::new(AccountCategories::CategoryType).string())
            .col(ColumnDef::new(AccountCategories::Description).string())
            .col(
                ColumnDef::new(AccountCategories::IsActive)
                    .boolean()
                    .default(true),
            )
            .col(
                ColumnDef::new(AccountCategories::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(AccountCategories::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AccountCategories::Table, AccountCategories::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(cats).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AccountCategories::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AccountCategories {
    Table,
    Id,
    InstitutionId,
    Name,
    CategoryType,
    Description,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
