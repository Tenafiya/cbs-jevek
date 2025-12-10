use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251208_154224_create_agents::Agents,
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
                "CREATE TYPE agent_wallet_type AS ENUM ('FLOAT', 'COMMISSION')".to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE agent_wallet_status AS ENUM ('ACTIVE', 'INACTIVE')".to_string(),
            ))
            .await?;

        let agent_wals = Table::create()
            .table(AgentWallets::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(AgentWallets::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(AgentWallets::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AgentWallets::AgentId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(AgentWallets::WalletType).custom("agent_wallet_type"))
            .col(ColumnDef::new(AgentWallets::Currency).json_binary())
            .col(
                ColumnDef::new(AgentWallets::CurrentBalance)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(AgentWallets::AvailableBalance)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(AgentWallets::MinBalance)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(ColumnDef::new(AgentWallets::MaxBalance).decimal_len(20, 4))
            .col(ColumnDef::new(AgentWallets::Status).custom("agent_wallet_status"))
            .col(ColumnDef::new(AgentWallets::LastFundedAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(AgentWallets::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(AgentWallets::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AgentWallets::Table, AgentWallets::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AgentWallets::Table, AgentWallets::AgentId)
                    .to(Agents::Table, Agents::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(agent_wals).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AgentWallets::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AgentWallets {
    Table,
    Id,
    InstitutionId,
    AgentId,
    WalletType,
    Currency,
    CurrentBalance,
    AvailableBalance,
    MinBalance,
    MaxBalance,
    Status,
    LastFundedAt,
    CreatedAt,
    UpdatedAt,
}
