use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::m20251212_200256_create_integration_webhooks::IntegrationWebhooks;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE integration_deliveries_status AS ENUM ('PENDING', 'FAILED', 'DELIVERED')".to_string(),
            ))
            .await?;

        let webhook_deliveries = Table::create()
            .table(WebhookDeliveries::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(WebhookDeliveries::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(WebhookDeliveries::WebhookId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(WebhookDeliveries::EventType)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(WebhookDeliveries::EventData)
                    .json_binary()
                    .not_null(),
            )
            .col(
                ColumnDef::new(WebhookDeliveries::DeliveryAttempt)
                    .integer()
                    .default(1),
            )
            .col(ColumnDef::new(WebhookDeliveries::RequestPayload).json_binary())
            .col(ColumnDef::new(WebhookDeliveries::HttpStatusCode).integer())
            .col(ColumnDef::new(WebhookDeliveries::ResponseBody).text())
            .col(ColumnDef::new(WebhookDeliveries::ResponseHeaders).json_binary())
            .col(
                ColumnDef::new(WebhookDeliveries::DeliveryStatus)
                    .custom("integration_deliveries_status")
                    .default("PENDING"),
            )
            .col(ColumnDef::new(WebhookDeliveries::DeliveredAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(WebhookDeliveries::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(WebhookDeliveries::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(WebhookDeliveries::Table, WebhookDeliveries::WebhookId)
                    .to(IntegrationWebhooks::Table, IntegrationWebhooks::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .index(
                Index::create()
                    .name("idx_webhook_deliveries_webhook")
                    .table(WebhookDeliveries::Table)
                    .col(WebhookDeliveries::WebhookId),
            )
            .index(
                Index::create()
                    .name("idx_webhook_deliveries_event_type")
                    .table(WebhookDeliveries::Table)
                    .col(WebhookDeliveries::EventType),
            )
            .index(
                Index::create()
                    .name("idx_webhook_deliveries_status")
                    .table(WebhookDeliveries::Table)
                    .col(WebhookDeliveries::DeliveryStatus),
            )
            .index(
                Index::create()
                    .name("idx_webhook_deliveries_attempt")
                    .table(WebhookDeliveries::Table)
                    .col(WebhookDeliveries::DeliveryAttempt),
            )
            .index(
                Index::create()
                    .name("idx_webhook_deliveries_created_at")
                    .table(WebhookDeliveries::Table)
                    .col(WebhookDeliveries::CreatedAt),
            )
            .to_owned();

        manager.create_table(webhook_deliveries).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WebhookDeliveries::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum WebhookDeliveries {
    Table,
    Id,
    WebhookId,
    EventType,
    EventData,
    DeliveryAttempt,
    RequestPayload,
    HttpStatusCode,
    ResponseBody,
    ResponseHeaders,
    DeliveryStatus,
    DeliveredAt,
    CreatedAt,
    UpdatedAt,
}
