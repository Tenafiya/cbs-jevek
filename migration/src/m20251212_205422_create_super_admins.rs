use sea_orm_migration::{prelude::*, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE super_admin_status AS ENUM ('ACTIVE', 'INACTIVE')".to_string(),
            ))
            .await?;

        let super_admins = Table::create()
            .table(SuperAdmins::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SuperAdmins::Id)
                    .big_integer()
                    .not_null()
                    .primary_key()
            )
            .col(
                ColumnDef::new(SuperAdmins::Username)
                    .string()
                    .not_null()
                    .unique_key(),
            )
            .col(
                ColumnDef::new(SuperAdmins::EmailAddress)
                    .string()
                    .not_null()
                    .unique_key(),
            )
            .col(
                ColumnDef::new(SuperAdmins::PasswordHash)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(SuperAdmins::MfaEnabled)
                    .boolean()
                    .default(true),
            )
            .col(ColumnDef::new(SuperAdmins::MfaSecretEncrypted).text())
            .col(ColumnDef::new(SuperAdmins::FullName).string())
            .col(ColumnDef::new(SuperAdmins::PhoneNumber).string())
            .col(
                ColumnDef::new(SuperAdmins::Permissions)
                    .json_binary()
                    .not_null(),
            )
            .col(
                ColumnDef::new(SuperAdmins::AdminStatus)
                    .custom("super_admin_status")
                    .default("ACTIVE"),
            )
            .col(ColumnDef::new(SuperAdmins::LastLoginAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(SuperAdmins::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(SuperAdmins::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .index(
                Index::create()
                    .name("idx_super_admins_username")
                    .table(SuperAdmins::Table)
                    .col(SuperAdmins::Username)
                    .unique(),
            )
            .index(
                Index::create()
                    .name("uk_super_admins_email")
                    .table(SuperAdmins::Table)
                    .col(SuperAdmins::EmailAddress)
                    .unique(),
            )
            .index(
                Index::create()
                    .name("idx_super_admins_status")
                    .table(SuperAdmins::Table)
                    .col(SuperAdmins::AdminStatus),
            )
            .index(
                Index::create()
                    .name("idx_super_admins_last_login")
                    .table(SuperAdmins::Table)
                    .col(SuperAdmins::LastLoginAt),
            )
            .to_owned();

        manager.create_table(super_admins).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SuperAdmins::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SuperAdmins {
    Table,
    Id,
    Username,
    EmailAddress,
    PasswordHash,
    MfaEnabled,
    MfaSecretEncrypted,
    FullName,
    PhoneNumber,
    Permissions,
    AdminStatus,
    LastLoginAt,
    CreatedAt,
    UpdatedAt,
}
