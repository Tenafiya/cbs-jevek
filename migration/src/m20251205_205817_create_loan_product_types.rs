use sea_orm_migration::prelude::*;

use crate::m20251204_112805_create_institutions::Institutions;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let pr = Table::create()
            .table(LoanProductTypes::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(LoanProductTypes::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(LoanProductTypes::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(LoanProductTypes::Name).string())
            .col(ColumnDef::new(LoanProductTypes::Code).string().unique_key())
            .col(ColumnDef::new(LoanProductTypes::Description).string())
            .col(
                ColumnDef::new(LoanProductTypes::IsActive)
                    .boolean()
                    .default(true),
            )
            .col(
                ColumnDef::new(LoanProductTypes::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(LoanProductTypes::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LoanProductTypes::Table, LoanProductTypes::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(pr).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LoanProductTypes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum LoanProductTypes {
    Table,
    Id,
    InstitutionId,
    Name,
    Code,
    Description,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
