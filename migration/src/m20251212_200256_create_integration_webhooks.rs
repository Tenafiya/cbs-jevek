use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
    m20251212_195551_create_integration_providers::IntegrationProviders,
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
                "CREATE TYPE integration_webhook_status AS ENUM ('ACTIVE', 'INACTIVE')".to_string(),
            ))
            .await?;

        let integration_webhooks = Table::create()
            .table(IntegrationWebhooks::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(IntegrationWebhooks::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(IntegrationWebhooks::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(IntegrationWebhooks::ProviderId).big_integer())
            .col(
                ColumnDef::new(IntegrationWebhooks::WebhookName)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(IntegrationWebhooks::EndpointUrl)
                    .text()
                    .not_null(),
            )
            .col(ColumnDef::new(IntegrationWebhooks::SecretTokenEncrypted).text())
            .col(
                ColumnDef::new(IntegrationWebhooks::SubscribedEvents)
                    .json_binary()
                    .not_null(),
            )
            .col(ColumnDef::new(IntegrationWebhooks::AuthMethod).string())
            .col(
                ColumnDef::new(IntegrationWebhooks::VerifySsl)
                    .boolean()
                    .default(true),
            )
            .col(
                ColumnDef::new(IntegrationWebhooks::WebhookStatus)
                    .custom("integration_webhook_status")
                    .default("ACTIVE"),
            )
            .col(ColumnDef::new(IntegrationWebhooks::LastDeliveryAt).timestamp_with_time_zone())
            .col(ColumnDef::new(IntegrationWebhooks::LastFailureAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(IntegrationWebhooks::ConsecutiveFailures)
                    .integer()
                    .default(0),
            )
            .col(
                ColumnDef::new(IntegrationWebhooks::MaxRetryAttempts)
                    .integer()
                    .default(3),
            )
            .col(ColumnDef::new(IntegrationWebhooks::CreatedBy).big_integer())
            .col(
                ColumnDef::new(IntegrationWebhooks::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(IntegrationWebhooks::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        IntegrationWebhooks::Table,
                        IntegrationWebhooks::InstitutionId,
                    )
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(IntegrationWebhooks::Table, IntegrationWebhooks::ProviderId)
                    .to(IntegrationProviders::Table, IntegrationProviders::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(IntegrationWebhooks::Table, IntegrationWebhooks::CreatedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .to_owned();

        manager.create_table(integration_webhooks).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IntegrationWebhooks::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum IntegrationWebhooks {
    Table,
    Id,
    InstitutionId,
    ProviderId,
    WebhookName,
    EndpointUrl,
    SecretTokenEncrypted,
    SubscribedEvents,
    AuthMethod,
    VerifySsl,
    WebhookStatus,
    LastDeliveryAt,
    LastFailureAt,
    ConsecutiveFailures,
    MaxRetryAttempts,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
