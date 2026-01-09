use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::m20251204_112805_create_institutions::Institutions;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE audit_trails_user_type AS ENUM ('STAFF', 'CUSTOMER', 'SYSTEM', 'API')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE audit_trails_resource_type AS ENUM ('ACCOUNT', 'TRANSACTION', 'CUSTOMER', 'LOAN')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE audit_trails_action AS ENUM ('CREATE', 'UPDATE', 'DELETE', 'VIEW', 'APPROVE')"
                    .to_string(),
            ))
            .await?;

        let audit_trails = Table::create()
            .table(AuditTrails::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(AuditTrails::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(AuditTrails::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AuditTrails::EventType)
                    .string_len(100)
                    .not_null(),
            )
            .col(ColumnDef::new(AuditTrails::EventDescription).string())
            .col(
                ColumnDef::new(AuditTrails::EventTimestamp)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(ColumnDef::new(AuditTrails::UserId).big_integer())
            .col(ColumnDef::new(AuditTrails::UserType).custom("audit_trails_user_type"))
            .col(ColumnDef::new(AuditTrails::SessionId).string())
            .col(ColumnDef::new(AuditTrails::IpAddress).custom("INET"))
            .col(ColumnDef::new(AuditTrails::UserAgent).string())
            .col(ColumnDef::new(AuditTrails::ResourceType).custom("audit_trails_resource_type"))
            .col(ColumnDef::new(AuditTrails::ResourceId).big_integer())
            .col(
                ColumnDef::new(AuditTrails::Action)
                    .custom("audit_trails_action")
                    .not_null(),
            )
            .col(ColumnDef::new(AuditTrails::OldValues).json_binary())
            .col(ColumnDef::new(AuditTrails::NewValues).json_binary())
            .col(ColumnDef::new(AuditTrails::Changes).json_binary())
            .col(ColumnDef::new(AuditTrails::MakerCheckerWorkflowId).big_integer())
            .col(
                ColumnDef::new(AuditTrails::IsSuccess)
                    .boolean()
                    .default(true),
            )
            .col(ColumnDef::new(AuditTrails::ErrorMessage).text())
            .col(ColumnDef::new(AuditTrails::Tags).custom("TEXT[]"))
            .col(
                ColumnDef::new(AuditTrails::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(AuditTrails::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AuditTrails::Table, AuditTrails::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(audit_trails).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AuditTrails::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AuditTrails {
    Table,
    Id,
    InstitutionId,
    EventType,
    EventDescription,
    EventTimestamp,
    UserId,
    UserType,
    SessionId,
    IpAddress,
    UserAgent,
    ResourceType,
    ResourceId,
    Action,
    OldValues,
    NewValues,
    Changes,
    MakerCheckerWorkflowId,
    IsSuccess,
    ErrorMessage,
    Tags,
    CreatedAt,
    UpdatedAt,
}
