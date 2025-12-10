use sea_orm_migration::prelude::*;

use crate::m20251204_103935_create_countries::Countries;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let insti = Table::create()
            .table(Institutions::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Institutions::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(Institutions::Name).string().not_null())
            .col(ColumnDef::new(Institutions::Code).string())
            .col(
                ColumnDef::new(Institutions::CountryId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(Institutions::Timezone).string())
            .col(
                ColumnDef::new(Institutions::LicenseNumber)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(Institutions::RegulatoryNumber)
                    .string()
                    .not_null(),
            )
            .col(ColumnDef::new(Institutions::LogoUrl).string())
            .col(
                ColumnDef::new(Institutions::IsDeleted)
                    .boolean()
                    .default(false),
            )
            .col(
                ColumnDef::new(Institutions::IsActive)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(Institutions::DeletedAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(Institutions::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Institutions::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Institutions::Table, Institutions::CountryId)
                    .to(Countries::Table, Countries::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(insti).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Institutions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Institutions {
    Table,
    Id,
    Name,
    Code,
    CountryId,
    Timezone,
    LicenseNumber,
    RegulatoryNumber,
    LogoUrl,
    IsDeleted,
    IsActive,
    DeletedAt,
    CreatedAt,
    UpdatedAt,
}
