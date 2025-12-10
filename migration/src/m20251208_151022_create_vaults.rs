use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Branches,
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
                "CREATE TYPE vault_status AS ENUM ('ACTIVE', 'INACTIVE')".to_string(),
            ))
            .await?;

        let vault = Table::create()
            .table(Vaults::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Vaults::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(Vaults::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(Vaults::BranchId).big_integer().not_null())
            .col(ColumnDef::new(Vaults::VaultName).string())
            .col(ColumnDef::new(Vaults::VaultCode).string())
            .col(
                ColumnDef::new(Vaults::TotalBalance)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(ColumnDef::new(Vaults::CashBreakdown).json_binary())
            .col(ColumnDef::new(Vaults::MaxBalance).decimal_len(20, 4))
            .col(
                ColumnDef::new(Vaults::MinBalance)
                    .decimal_len(20, 4)
                    .default(Expr::current_timestamp()),
            )
            .col(ColumnDef::new(Vaults::Status).custom("vault_status"))
            .col(ColumnDef::new(Vaults::LastAuditedAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(Vaults::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Vaults::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Vaults::Table, Vaults::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Vaults::Table, Vaults::BranchId)
                    .to(Branches::Table, Branches::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(vault).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Vaults::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Vaults {
    Table,
    Id,
    InstitutionId,
    BranchId,
    VaultName,
    VaultCode,
    TotalBalance,
    CashBreakdown,
    MaxBalance,
    MinBalance,
    Status,
    LastAuditedAt,
    CreatedAt,
    UpdatedAt,
}
