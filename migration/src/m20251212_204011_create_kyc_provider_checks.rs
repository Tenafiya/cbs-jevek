use sea_orm_migration::prelude::*;

use crate::{
    m20251204_112805_create_institutions::Institutions,
    m20251204_152312_create_customers::Customers,
    m20251204_155243_create_customer_identifications::CustomerIdentifications,
    m20251212_195551_create_integration_providers::IntegrationProviders,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let kyc_provider_checks = Table::create()
            .table(KycProviderChecks::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(KycProviderChecks::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(KycProviderChecks::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(KycProviderChecks::ProviderId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(KycProviderChecks::CustomerId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(KycProviderChecks::CheckType)
                    .string()
                    .not_null(),
            )
            .col(ColumnDef::new(KycProviderChecks::IdentificationId).big_integer())
            .col(
                ColumnDef::new(KycProviderChecks::CheckStatus)
                    .string()
                    .not_null(),
            )
            .col(ColumnDef::new(KycProviderChecks::VerificationScore).decimal_len(10, 6))
            .col(ColumnDef::new(KycProviderChecks::VerificationData).json_binary())
            .col(ColumnDef::new(KycProviderChecks::SubmittedImageUrl).text())
            .col(ColumnDef::new(KycProviderChecks::VerifiedImageUrl).text())
            .col(ColumnDef::new(KycProviderChecks::ErrorCode).string())
            .col(ColumnDef::new(KycProviderChecks::ErrorMessage).text())
            .col(
                ColumnDef::new(KycProviderChecks::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(KycProviderChecks::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(KycProviderChecks::Table, KycProviderChecks::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(KycProviderChecks::Table, KycProviderChecks::ProviderId)
                    .to(IntegrationProviders::Table, IntegrationProviders::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(KycProviderChecks::Table, KycProviderChecks::CustomerId)
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        KycProviderChecks::Table,
                        KycProviderChecks::IdentificationId,
                    )
                    .to(CustomerIdentifications::Table, CustomerIdentifications::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .to_owned();

        manager.create_table(kyc_provider_checks).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(KycProviderChecks::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum KycProviderChecks {
    Table,
    Id,
    InstitutionId,
    ProviderId,
    CustomerId,
    CheckType,
    IdentificationId,
    CheckStatus,
    VerificationScore,
    VerificationData,
    SubmittedImageUrl,
    VerifiedImageUrl,
    ErrorCode,
    ErrorMessage,
    CreatedAt,
    UpdatedAt,
}
