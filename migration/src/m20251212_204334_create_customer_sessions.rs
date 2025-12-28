use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions,
    m20251204_152312_create_customers::Customers,
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

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE customer_sessions_status AS ENUM ('ACTIVE', 'INACTIVE')".to_string(),
            ))
            .await?;

        let customer_sessions = Table::create()
            .table(CustomerSessions::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CustomerSessions::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(CustomerSessions::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CustomerSessions::CustomerId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(CustomerSessions::DeviceId).string())
            .col(ColumnDef::new(CustomerSessions::DeviceType).string())
            .col(ColumnDef::new(CustomerSessions::DeviceOs).string())
            .col(ColumnDef::new(CustomerSessions::DeviceFingerprint).string())
            .col(
                ColumnDef::new(CustomerSessions::SessionTokenHash)
                    .string()
                    .not_null(),
            )
            .col(ColumnDef::new(CustomerSessions::RefreshTokenHash).string())
            .col(
                ColumnDef::new(CustomerSessions::MfaVerified)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(CustomerSessions::MfaMethod).string())
            .col(ColumnDef::new(CustomerSessions::IpAddress).custom("INET"))
            .col(ColumnDef::new(CustomerSessions::Location).json_binary())
            .col(
                ColumnDef::new(CustomerSessions::SessionStatus)
                    .custom("customer_sessions_status")
                    .default("ACTIVE"),
            )
            .col(
                ColumnDef::new(CustomerSessions::IsCurrent)
                    .boolean()
                    .default(true),
            )
            .col(
                ColumnDef::new(CustomerSessions::LoginAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(CustomerSessions::LastActivityAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(CustomerSessions::ExpiresAt)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(ColumnDef::new(CustomerSessions::LoggedOutAt).timestamp_with_time_zone())
            .col(ColumnDef::new(CustomerSessions::LoginMethod).string())
            .col(
                ColumnDef::new(CustomerSessions::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(CustomerSessions::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(CustomerSessions::Table, CustomerSessions::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(CustomerSessions::Table, CustomerSessions::CustomerId)
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .index(
                Index::create()
                    .name("uk_customer_session_token")
                    .table(CustomerSessions::Table)
                    .col(CustomerSessions::SessionTokenHash)
                    .unique(),
            )
            .index(
                Index::create()
                    .name("idx_customer_sessions_customer")
                    .table(CustomerSessions::Table)
                    .col(CustomerSessions::CustomerId),
            )
            .index(
                Index::create()
                    .name("idx_customer_sessions_status")
                    .table(CustomerSessions::Table)
                    .col(CustomerSessions::SessionStatus),
            )
            .index(
                Index::create()
                    .name("idx_customer_sessions_current")
                    .table(CustomerSessions::Table)
                    .col(CustomerSessions::IsCurrent),
            )
            .index(
                Index::create()
                    .name("idx_customer_sessions_expires")
                    .table(CustomerSessions::Table)
                    .col(CustomerSessions::ExpiresAt),
            )
            .index(
                Index::create()
                    .name("idx_customer_sessions_activity")
                    .table(CustomerSessions::Table)
                    .col(CustomerSessions::LastActivityAt),
            )
            .to_owned();

        manager.create_table(customer_sessions).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CustomerSessions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum CustomerSessions {
    Table,
    Id,
    InstitutionId,
    CustomerId,
    DeviceId,
    DeviceType,
    DeviceOs,
    DeviceFingerprint,
    SessionTokenHash,
    RefreshTokenHash,
    MfaVerified,
    MfaMethod,
    IpAddress,
    Location,
    SessionStatus,
    IsCurrent,
    LoginAt,
    LastActivityAt,
    ExpiresAt,
    LoggedOutAt,
    LoginMethod,
    CreatedAt,
    UpdatedAt,
}
