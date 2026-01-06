use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_103935_create_countries::Countries,
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
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
                "CREATE TYPE customer_type AS ENUM ('INDIVIDUAL', 'SME', 'GROUP', 'COOPERATIVE', 'CORPORATE')"
                    .to_string(),
            ))
            .await?;

        let customer = Table::create()
            .table(Customers::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Customers::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(Customers::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(Customers::CustomerType)
                    .custom("customer_type")
                    .default("INDIVIDUAL"),
            )
            .col(
                ColumnDef::new(Customers::CustomerNumber)
                    .string()
                    .unique_key(),
            )
            .col(ColumnDef::new(Customers::RiskLevel).string())
            .col(ColumnDef::new(Customers::Status).string())
            .col(
                ColumnDef::new(Customers::IsBlackListed)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(Customers::BlackListReason).string())
            .col(ColumnDef::new(Customers::FirstName).string())
            .col(ColumnDef::new(Customers::MiddleName).string())
            .col(ColumnDef::new(Customers::LastName).string())
            .col(ColumnDef::new(Customers::DateOfBirth).date())
            .col(ColumnDef::new(Customers::Gender).string())
            .col(ColumnDef::new(Customers::Nationality).string())
            .col(ColumnDef::new(Customers::PhoneCountryCode).string())
            .col(ColumnDef::new(Customers::PhoneNumber).string())
            .col(ColumnDef::new(Customers::Email).string())
            .col(
                ColumnDef::new(Customers::IsPhoneVerified)
                    .boolean()
                    .default(false),
            )
            .col(
                ColumnDef::new(Customers::IsEmailVerified)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(Customers::PhoneVerifiedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(Customers::EmailVerifiedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(Customers::ResidentialAddress).json_binary())
            .col(ColumnDef::new(Customers::PostalAddress).json_binary())
            .col(ColumnDef::new(Customers::City).string())
            .col(ColumnDef::new(Customers::StateProvince).string())
            .col(ColumnDef::new(Customers::CountryId).big_integer())
            .col(ColumnDef::new(Customers::Occupation).string())
            .col(ColumnDef::new(Customers::EmployerName).string())
            .col(ColumnDef::new(Customers::IncomeSource).string())
            .col(
                ColumnDef::new(Customers::MonthlyIncome)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(ColumnDef::new(Customers::EmployeeNumber).string())
            .col(ColumnDef::new(Customers::NextOfKin).json_binary())
            .col(ColumnDef::new(Customers::IsPep).boolean().default(false))
            .col(ColumnDef::new(Customers::PepDetails).json_binary())
            .col(
                ColumnDef::new(Customers::IsSanctionsCheckPassed)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(Customers::SanctionsCheckDate).timestamp_with_time_zone())
            .col(ColumnDef::new(Customers::SanctionsProvider).string())
            .col(ColumnDef::new(Customers::CustomFields).json_binary())
            .col(ColumnDef::new(Customers::Tags).array(ColumnType::Text))
            .col(ColumnDef::new(Customers::VerifiedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(Customers::CreatedBy).big_integer())
            .col(ColumnDef::new(Customers::VerifiedBy).big_integer())
            .col(ColumnDef::new(Customers::KycTier).json_binary())
            .col(
                ColumnDef::new(Customers::IsDeleted)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(Customers::DeletedAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(Customers::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Customers::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Customers::Table, Customers::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Customers::Table, Customers::CountryId)
                    .to(Countries::Table, Countries::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Customers::Table, Customers::CreatedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Customers::Table, Customers::VerifiedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(customer).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Customers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Customers {
    Table,
    Id,
    InstitutionId,
    CustomerType,
    CustomerNumber,
    KycTier,
    RiskLevel,
    Status,
    IsBlackListed,
    BlackListReason,
    FirstName,
    MiddleName,
    LastName,
    DateOfBirth,
    Gender,
    Nationality,
    PhoneCountryCode,
    PhoneNumber,
    Email,
    IsPhoneVerified,
    IsEmailVerified,
    PhoneVerifiedAt,
    EmailVerifiedAt,
    ResidentialAddress,
    PostalAddress,
    City,
    StateProvince,
    CountryId,
    Occupation,
    EmployerName,
    IncomeSource,
    MonthlyIncome,
    EmployeeNumber,
    NextOfKin,
    IsPep,
    PepDetails,
    IsSanctionsCheckPassed,
    SanctionsCheckDate,
    SanctionsProvider,
    CustomFields,
    Tags,
    CreatedBy,
    VerifiedBy,
    VerifiedAt,
    IsDeleted,
    DeletedAt,
    CreatedAt,
    UpdatedAt,
}
