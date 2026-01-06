use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions,
    m20251204_150208_create_branches::{Branches, Staff},
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
                "CREATE TYPE teller_status AS ENUM ('ACTIVE', 'INACTIVE')".to_string(),
            ))
            .await?;

        let teller = Table::create()
            .table(Tellers::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Tellers::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(Tellers::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(Tellers::BranchId).big_integer().not_null())
            .col(ColumnDef::new(Tellers::TellerName).string().not_null())
            .col(ColumnDef::new(Tellers::TellerNumber).string().not_null())
            .col(
                ColumnDef::new(Tellers::DrawerLimit)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(Tellers::CurrentDrawerBalance)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(Tellers::Status)
                    .custom("teller_status")
                    .default("ACTIVE"),
            )
            .col(ColumnDef::new(Tellers::IsLoggedIn).boolean().default(false))
            .col(ColumnDef::new(Tellers::LastLoginAt).timestamp_with_time_zone())
            .col(ColumnDef::new(Tellers::CurrentSessionId).string())
            .col(ColumnDef::new(Tellers::CurrentTerminalId).string())
            .col(ColumnDef::new(Tellers::StaffId).big_integer())
            .col(
                ColumnDef::new(Tellers::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Tellers::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Tellers::Table, Tellers::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Tellers::Table, Tellers::BranchId)
                    .to(Branches::Table, Branches::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Tellers::Table, Tellers::StaffId)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(teller).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tellers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Tellers {
    Table,
    Id,
    InstitutionId,
    BranchId,
    StaffId,
    TellerNumber,
    TellerName,
    DrawerLimit,
    CurrentDrawerBalance,
    Status,
    IsLoggedIn,
    LastLoginAt,
    CurrentSessionId,
    CurrentTerminalId,
    CreatedAt,
    UpdatedAt,
}
