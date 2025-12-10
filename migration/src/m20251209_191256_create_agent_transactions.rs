use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions,
    m20251205_193221_create_transactions::Transactions, m20251208_154224_create_agents::Agents,
    m20251208_162111_create_agent_wallets::AgentWallets,
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
                "CREATE TYPE agent_trans_type AS ENUM ('CASH_DEPOSIT', 'CASH_WITHDRAWAL', 'WALLET_TOPUP', 'WALLET_WITHDRAWAL', 'LOAN_REPAYMENT', 'BILL_PAYMENT')".to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE agent_trans_status AS ENUM ('PENDING', 'PROCESSED', 'FAILED', 'CANCELLED')".to_string(),
            ))
            .await?;

        let agent_trans = Table::create()
            .table(AgentTransactions::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(AgentTransactions::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(AgentTransactions::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AgentTransactions::AgentId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AgentTransactions::AgentWalletId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(AgentTransactions::TransactionId).big_integer())
            .col(
                ColumnDef::new(AgentTransactions::TransactionType)
                    .custom("agent_trans_type")
                    .not_null(),
            )
            .col(ColumnDef::new(AgentTransactions::Amount).decimal_len(20, 4))
            .col(
                ColumnDef::new(AgentTransactions::CommissionEarned)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(ColumnDef::new(AgentTransactions::CustomerPhone).string())
            .col(ColumnDef::new(AgentTransactions::CustomerAccount).string())
            .col(ColumnDef::new(AgentTransactions::Status).custom("agent_trans_status"))
            .col(ColumnDef::new(AgentTransactions::PostedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(AgentTransactions::ConfirmedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(AgentTransactions::TransactionAddress).json_binary())
            .col(ColumnDef::new(AgentTransactions::GeoAccuracy).decimal_len(10, 2))
            .col(ColumnDef::new(AgentTransactions::ReconciledAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(AgentTransactions::IsReconciled)
                    .boolean()
                    .default(false),
            )
            .col(
                ColumnDef::new(AgentTransactions::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(AgentTransactions::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AgentTransactions::Table, AgentTransactions::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AgentTransactions::Table, AgentTransactions::AgentId)
                    .to(Agents::Table, Agents::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AgentTransactions::Table, AgentTransactions::AgentWalletId)
                    .to(AgentWallets::Table, AgentWallets::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AgentTransactions::Table, AgentTransactions::TransactionId)
                    .to(Transactions::Table, Transactions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(agent_trans).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AgentTransactions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AgentTransactions {
    Table,
    Id,
    InstitutionId,
    AgentId,
    AgentWalletId,
    TransactionId,
    TransactionType,
    Amount,
    CommissionEarned,
    CustomerPhone,
    CustomerAccount,
    Status,
    PostedAt,
    ConfirmedAt,
    TransactionAddress,
    GeoAccuracy,
    ReconciledAt,
    IsReconciled,
    CreatedAt,
    UpdatedAt,
}
