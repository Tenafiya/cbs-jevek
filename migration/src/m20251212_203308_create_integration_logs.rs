use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions,
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
                "CREATE TYPE integration_log_status AS ENUM ('SUCCESS', 'FAILURE')".to_string(),
            ))
            .await?;

        let integration_logs = Table::create()
            .table(IntegrationLogs::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(IntegrationLogs::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(IntegrationLogs::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(IntegrationLogs::ProviderId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(IntegrationLogs::RequestMethod)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(IntegrationLogs::RequestUrl)
                    .text()
                    .not_null(),
            )
            .col(ColumnDef::new(IntegrationLogs::RequestHeaders).json_binary())
            .col(ColumnDef::new(IntegrationLogs::RequestBody).json_binary())
            .col(ColumnDef::new(IntegrationLogs::ResponseStatusCode).integer())
            .col(ColumnDef::new(IntegrationLogs::ResponseBody).json_binary())
            .col(ColumnDef::new(IntegrationLogs::ResponseTimeMs).integer())
            .col(
                ColumnDef::new(IntegrationLogs::Status)
                    .custom("integration_log_status")
                    .default("SUCCESS"),
            )
            .col(ColumnDef::new(IntegrationLogs::ErrorMessage).text())
            .col(ColumnDef::new(IntegrationLogs::ErrorCode).string())
            .col(ColumnDef::new(IntegrationLogs::ReferenceType).string())
            .col(ColumnDef::new(IntegrationLogs::ReferenceId).big_integer())
            .col(
                ColumnDef::new(IntegrationLogs::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(IntegrationLogs::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(IntegrationLogs::Table, IntegrationLogs::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(IntegrationLogs::Table, IntegrationLogs::ProviderId)
                    .to(IntegrationProviders::Table, IntegrationProviders::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .index(
                Index::create()
                    .name("idx_integration_logs_provider")
                    .table(IntegrationLogs::Table)
                    .col(IntegrationLogs::ProviderId),
            )
            .index(
                Index::create()
                    .name("idx_integration_logs_status")
                    .table(IntegrationLogs::Table)
                    .col(IntegrationLogs::Status),
            )
            .index(
                Index::create()
                    .name("idx_integration_logs_created_at")
                    .table(IntegrationLogs::Table)
                    .col(IntegrationLogs::CreatedAt),
            )
            .index(
                Index::create()
                    .name("idx_integration_logs_reference")
                    .table(IntegrationLogs::Table)
                    .col(IntegrationLogs::ReferenceType)
                    .col(IntegrationLogs::ReferenceId),
            )
            .index(
                Index::create()
                    .name("idx_integration_logs_response_time")
                    .table(IntegrationLogs::Table)
                    .col(IntegrationLogs::ResponseTimeMs),
            )
            .to_owned();

        manager.create_table(integration_logs).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IntegrationLogs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum IntegrationLogs {
    Table,
    Id,
    InstitutionId,
    ProviderId,
    RequestMethod,
    RequestUrl,
    RequestHeaders,
    RequestBody,
    ResponseStatusCode,
    ResponseBody,
    ResponseTimeMs,
    Status,
    ErrorMessage,
    ErrorCode,
    ReferenceType,
    ReferenceId,
    CreatedAt,
    UpdatedAt,
}
