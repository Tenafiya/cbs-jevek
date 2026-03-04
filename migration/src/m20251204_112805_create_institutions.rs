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
            .col(ColumnDef::new(Institutions::City).string())
            .col(ColumnDef::new(Institutions::ZipCode).string())
            .col(ColumnDef::new(Institutions::State).string())
            .col(ColumnDef::new(Institutions::DateFormat).string())
            .col(ColumnDef::new(Institutions::DateTimeFormat).string())
            .col(ColumnDef::new(Institutions::Address).json_binary())
            .col(ColumnDef::new(Institutions::PostalAddress).json_binary())
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

        let contact_details  = Table::create()
            .table(ContactDetails::Table)
            .if_not_exists()
            .col(ColumnDef::new(ContactDetails::Id).big_integer().not_null().primary_key())
            .col(ColumnDef::new(ContactDetails::InstitutionId).big_integer().not_null())
            .col(ColumnDef::new(ContactDetails::Department).string())
            .col(ColumnDef::new(ContactDetails::ContactType).string().not_null())
            .col(ColumnDef::new(ContactDetails::ContactValue).string().not_null())
            .col(ColumnDef::new(ContactDetails::IsActive).boolean().default(true))
            .col(ColumnDef::new(ContactDetails::IsPrimary).boolean().default(false))
            .col(
                ColumnDef::new(ContactDetails::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(ContactDetails::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .to_owned();

        manager.create_table(contact_details).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Institutions::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum ContactDetails {
    Table,
    Id,
    InstitutionId,
    Department,
    ContactType,
    ContactValue,
    IsActive,
    IsPrimary,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Institutions {
    Table,
    Id,
    Name,
    Code,
    CountryId,
    City,
    Timezone,
    LicenseNumber,
    RegulatoryNumber,
    ZipCode,
    State,
    PostalAddress,
    Address,
    DateFormat,
    DateTimeFormat,
    LogoUrl,
    IsDeleted,
    IsActive,
    DeletedAt,
    CreatedAt,
    UpdatedAt,
}
