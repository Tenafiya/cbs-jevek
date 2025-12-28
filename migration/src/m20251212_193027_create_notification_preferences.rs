use sea_orm_migration::prelude::*;

use crate::{
    m20251204_112805_create_institutions::Institutions,
    m20251204_152312_create_customers::Customers,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let notification_preferences = Table::create()
            .table(NotificationPreferences::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(NotificationPreferences::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(NotificationPreferences::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(NotificationPreferences::CustomerId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(NotificationPreferences::SmsEnabled)
                    .boolean()
                    .default(true),
            )
            .col(
                ColumnDef::new(NotificationPreferences::EmailEnabled)
                    .boolean()
                    .default(true),
            )
            .col(
                ColumnDef::new(NotificationPreferences::PushEnabled)
                    .boolean()
                    .default(false),
            )
            .col(
                ColumnDef::new(NotificationPreferences::WhatsappEnabled)
                    .boolean()
                    .default(false),
            )
            .col(
                ColumnDef::new(NotificationPreferences::EventTypes)
                    .json_binary()
                    .not_null(),
            )
            .col(ColumnDef::new(NotificationPreferences::QuietHoursStart).time())
            .col(ColumnDef::new(NotificationPreferences::QuietHoursEnd).time())
            .col(
                ColumnDef::new(NotificationPreferences::DailyMessageLimit)
                    .integer()
                    .default(10),
            )
            .col(
                ColumnDef::new(NotificationPreferences::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(NotificationPreferences::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        NotificationPreferences::Table,
                        NotificationPreferences::InstitutionId,
                    )
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        NotificationPreferences::Table,
                        NotificationPreferences::CustomerId,
                    )
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .index(
                Index::create()
                    .name("idx_notification_preferences")
                    .table(NotificationPreferences::Table)
                    .col(NotificationPreferences::InstitutionId)
                    .col(NotificationPreferences::CustomerId)
                    .unique(),
            )
            .index(
                Index::create()
                    .name("idx_notification_preferences_customer")
                    .table(NotificationPreferences::Table)
                    .col(NotificationPreferences::CustomerId),
            )
            .to_owned();

        manager.create_table(notification_preferences).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(NotificationPreferences::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum NotificationPreferences {
    Table,
    Id,
    InstitutionId,
    CustomerId,
    SmsEnabled,
    EmailEnabled,
    PushEnabled,
    WhatsappEnabled,
    EventTypes,
    QuietHoursStart,
    QuietHoursEnd,
    DailyMessageLimit,
    CreatedAt,
    UpdatedAt,
}
