use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions,
    m20251212_205422_create_super_admins::SuperAdmins,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the INET extension if not exists
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"".to_string(),
            ))
            .await?;

        let admin_audit_logs = Table::create()
            .table(AdminAuditLogs::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(AdminAuditLogs::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(AdminAuditLogs::AdminId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(AdminAuditLogs::InstitutionId).big_integer())
            .col(ColumnDef::new(AdminAuditLogs::Action).string().not_null())
            .col(ColumnDef::new(AdminAuditLogs::ResourceType).string())
            .col(ColumnDef::new(AdminAuditLogs::ResourceId).big_integer())
            .col(ColumnDef::new(AdminAuditLogs::OldValues).json_binary())
            .col(ColumnDef::new(AdminAuditLogs::NewValues).json_binary())
            .col(ColumnDef::new(AdminAuditLogs::IpAddress).custom("INET"))
            .col(ColumnDef::new(AdminAuditLogs::UserAgent).text())
            .col(
                ColumnDef::new(AdminAuditLogs::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(AdminAuditLogs::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AdminAuditLogs::Table, AdminAuditLogs::AdminId)
                    .to(SuperAdmins::Table, SuperAdmins::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AdminAuditLogs::Table, AdminAuditLogs::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .to_owned();

        manager.create_table(admin_audit_logs).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AdminAuditLogs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AdminAuditLogs {
    Table,
    Id,
    AdminId,
    InstitutionId,
    Action,
    ResourceType,
    ResourceId,
    OldValues,
    NewValues,
    IpAddress,
    UserAgent,
    CreatedAt,
    UpdatedAt,
}
