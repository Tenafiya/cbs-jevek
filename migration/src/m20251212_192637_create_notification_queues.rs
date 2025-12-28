use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
    m20251204_152312_create_customers::Customers, m20251208_154224_create_agents::Agents,
    m20251212_192129_create_notification_templates::NotificationTemplates,
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
                "CREATE TYPE notification_queue_status AS ENUM ('PENDING', 'SENT', 'DELIVERED', 'FAILED')"
                    .to_string(),
            ))
            .await?;

        let notification_queue = Table::create()
            .table(NotificationQueue::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(NotificationQueue::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(NotificationQueue::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(NotificationQueue::CustomerId).big_integer())
            .col(ColumnDef::new(NotificationQueue::StaffId).big_integer())
            .col(ColumnDef::new(NotificationQueue::AgentId).big_integer())
            .col(ColumnDef::new(NotificationQueue::RecipientContact).string())
            .col(ColumnDef::new(NotificationQueue::RecipientDeviceId).string())
            .col(ColumnDef::new(NotificationQueue::TemplateId).big_integer())
            .col(
                ColumnDef::new(NotificationQueue::NotificationType)
                    .string()
                    .not_null(),
            )
            .col(ColumnDef::new(NotificationQueue::Subject).text())
            .col(ColumnDef::new(NotificationQueue::Body).text().not_null())
            .col(ColumnDef::new(NotificationQueue::Variables).json_binary())
            .col(
                ColumnDef::new(NotificationQueue::Status)
                    .custom("notification_queue_status")
                    .default("PENDING"),
            )
            .col(
                ColumnDef::new(NotificationQueue::Priority)
                    .integer()
                    .default(3),
            )
            .col(ColumnDef::new(NotificationQueue::ScheduledAt).timestamp_with_time_zone())
            .col(ColumnDef::new(NotificationQueue::SentAt).timestamp_with_time_zone())
            .col(ColumnDef::new(NotificationQueue::DeliveredAt).timestamp_with_time_zone())
            .col(ColumnDef::new(NotificationQueue::FailedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(NotificationQueue::FailureReason).text())
            .col(ColumnDef::new(NotificationQueue::ProviderResponse).json_binary())
            .col(ColumnDef::new(NotificationQueue::ProviderMessageId).string())
            .col(
                ColumnDef::new(NotificationQueue::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(NotificationQueue::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(NotificationQueue::Table, NotificationQueue::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(NotificationQueue::Table, NotificationQueue::CustomerId)
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(NotificationQueue::Table, NotificationQueue::StaffId)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(NotificationQueue::Table, NotificationQueue::AgentId)
                    .to(Agents::Table, Agents::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(NotificationQueue::Table, NotificationQueue::TemplateId)
                    .to(NotificationTemplates::Table, NotificationTemplates::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .index(
                Index::create()
                    .name("idx_notification_queue_status")
                    .table(NotificationQueue::Table)
                    .col(NotificationQueue::Status),
            )
            .index(
                Index::create()
                    .name("idx_notification_queue_priority")
                    .table(NotificationQueue::Table)
                    .col(NotificationQueue::Priority),
            )
            .index(
                Index::create()
                    .name("idx_notification_queue_scheduled")
                    .table(NotificationQueue::Table)
                    .col(NotificationQueue::ScheduledAt),
            )
            .index(
                Index::create()
                    .name("idx_notification_queue_type")
                    .table(NotificationQueue::Table)
                    .col(NotificationQueue::NotificationType),
            )
            .to_owned();

        manager.create_table(notification_queue).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NotificationQueue::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum NotificationQueue {
    Table,
    Id,
    InstitutionId,
    CustomerId,
    StaffId,
    AgentId,
    RecipientContact,
    RecipientDeviceId,
    TemplateId,
    NotificationType,
    Subject,
    Body,
    Variables,
    Status,
    Priority,
    ScheduledAt,
    SentAt,
    DeliveredAt,
    FailedAt,
    FailureReason,
    ProviderResponse,
    ProviderMessageId,
    CreatedAt,
    UpdatedAt,
}
