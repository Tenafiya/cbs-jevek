use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
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
                "CREATE TYPE integration_providers_service_type AS ENUM ('KYC', 'CREDIT_BUREAU', 'SMS_GATEWAY', 'PAYMENT_PROCESSOR')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE integration_providers_prov_status AS ENUM ('ACTIVE', 'INACTIVE', 'MAINTENANCE')"
                    .to_string(),
            ))
            .await?;

        let integration_providers = Table::create()
            .table(IntegrationProviders::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(IntegrationProviders::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(IntegrationProviders::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(IntegrationProviders::ProviderName)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(IntegrationProviders::ProviderCode)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(IntegrationProviders::ServiceType)
                    .custom("integration_providers_service_type")
                    .not_null(),
            )
            .col(ColumnDef::new(IntegrationProviders::ApiBaseUrl).text())
            .col(ColumnDef::new(IntegrationProviders::ApiVersion).string())
            .col(ColumnDef::new(IntegrationProviders::ApiKeyEncrypted).text())
            .col(ColumnDef::new(IntegrationProviders::SecretKeyEncrypted).text())
            .col(ColumnDef::new(IntegrationProviders::WebhookSecret).text())
            .col(ColumnDef::new(IntegrationProviders::AuthMethod).string())
            .col(ColumnDef::new(IntegrationProviders::OauthConfig).json_binary())
            .col(ColumnDef::new(IntegrationProviders::JwtConfig).json_binary())
            .col(
                ColumnDef::new(IntegrationProviders::ProviderStatus)
                    .custom("integration_providers_prov_status")
                    .default("ACTIVE"),
            )
            .col(
                ColumnDef::new(IntegrationProviders::IsPrimary)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(IntegrationProviders::RateLimitPerMinute).integer())
            .col(ColumnDef::new(IntegrationProviders::RateLimitPerHour).integer())
            .col(ColumnDef::new(IntegrationProviders::RateLimitPerDay).integer())
            .col(
                ColumnDef::new(IntegrationProviders::RetryEnabled)
                    .boolean()
                    .default(true),
            )
            .col(
                ColumnDef::new(IntegrationProviders::MaxRetryAttempts)
                    .integer()
                    .default(3),
            )
            .col(ColumnDef::new(IntegrationProviders::CreatedBy).big_integer())
            .col(
                ColumnDef::new(IntegrationProviders::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(IntegrationProviders::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        IntegrationProviders::Table,
                        IntegrationProviders::InstitutionId,
                    )
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(IntegrationProviders::Table, IntegrationProviders::CreatedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .to_owned();

        manager.create_table(integration_providers).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IntegrationProviders::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum IntegrationProviders {
    Table,
    Id,
    InstitutionId,
    ProviderName,
    ProviderCode,
    ServiceType,
    ApiBaseUrl,
    ApiVersion,
    ApiKeyEncrypted,
    SecretKeyEncrypted,
    WebhookSecret,
    AuthMethod,
    OauthConfig,
    JwtConfig,
    ProviderStatus,
    IsPrimary,
    RateLimitPerMinute,
    RateLimitPerHour,
    RateLimitPerDay,
    RetryEnabled,
    MaxRetryAttempts,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
