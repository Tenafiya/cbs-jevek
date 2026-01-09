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
                "CREATE TYPE notification_type AS ENUM ('SMS', 'EMAIL', 'PUSH', 'WHATSAPP', 'IN_APP')"
                    .to_string(),
            ))
            .await?;

        let notification_templates = Table::create()
            .table(NotificationTemplates::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(NotificationTemplates::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(NotificationTemplates::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(NotificationTemplates::TemplateName)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(NotificationTemplates::TemplateCode)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(NotificationTemplates::NotificationType)
                    .custom("notification_type")
                    .not_null(),
            )
            .col(ColumnDef::new(NotificationTemplates::SubjectTemplate).string())
            .col(
                ColumnDef::new(NotificationTemplates::BodyTemplate)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(NotificationTemplates::LanguageCode)
                    .string()
                    .default("en"),
            )
            .col(ColumnDef::new(NotificationTemplates::RequiredVariables).json_binary())
            .col(ColumnDef::new(NotificationTemplates::SmsSender).string())
            .col(ColumnDef::new(NotificationTemplates::EmailTemplateHtml).string())
            .col(ColumnDef::new(NotificationTemplates::PushNotificationData).json_binary())
            .col(
                ColumnDef::new(NotificationTemplates::IsActive)
                    .boolean()
                    .default(true),
            )
            .col(
                ColumnDef::new(NotificationTemplates::IsDefault)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(NotificationTemplates::CreatedBy).big_integer())
            .col(
                ColumnDef::new(NotificationTemplates::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(NotificationTemplates::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        NotificationTemplates::Table,
                        NotificationTemplates::InstitutionId,
                    )
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        NotificationTemplates::Table,
                        NotificationTemplates::CreatedBy,
                    )
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(notification_templates).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NotificationTemplates::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum NotificationTemplates {
    Table,
    Id,
    InstitutionId,
    TemplateName,
    TemplateCode,
    NotificationType,
    SubjectTemplate,
    BodyTemplate,
    LanguageCode,
    RequiredVariables,
    SmsSender,
    EmailTemplateHtml,
    PushNotificationData,
    IsActive,
    IsDefault,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
