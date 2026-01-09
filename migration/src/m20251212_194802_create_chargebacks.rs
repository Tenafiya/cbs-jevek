use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions,
    m20251205_193221_create_transactions::Transactions,
    m20251207_214445_create_card_transactions::CardTransactions,
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
                "CREATE TYPE chargeback_status AS ENUM ('INITIATED', 'INVESTIGATION', 'WON', 'LOST', 'ARBITRATION')"
                    .to_string(),
            ))
            .await?;

        let chargebacks = Table::create()
            .table(Chargebacks::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Chargebacks::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(Chargebacks::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(Chargebacks::TransactionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(Chargebacks::CardTransactionId).big_integer())
            .col(ColumnDef::new(Chargebacks::DisputeId).big_integer())
            .col(
                ColumnDef::new(Chargebacks::ChargebackAmount)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(ColumnDef::new(Chargebacks::ChargebackReasonCode).string_len(50))
            .col(ColumnDef::new(Chargebacks::ChargebackReason).text())
            .col(
                ColumnDef::new(Chargebacks::Status)
                    .custom("chargeback_status")
                    .default("INITIATED"),
            )
            .col(
                ColumnDef::new(Chargebacks::InitiatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(ColumnDef::new(Chargebacks::RepresentmentDueDate).date())
            .col(ColumnDef::new(Chargebacks::ResolvedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(Chargebacks::ProvisionalCreditAmount).decimal_len(20, 4))
            .col(ColumnDef::new(Chargebacks::FinalLiabilityAmount).decimal_len(20, 4))
            .col(ColumnDef::new(Chargebacks::NetworkReference).string())
            .col(ColumnDef::new(Chargebacks::NetworkResponse).json_binary())
            .col(
                ColumnDef::new(Chargebacks::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Chargebacks::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Chargebacks::Table, Chargebacks::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Chargebacks::Table, Chargebacks::TransactionId)
                    .to(Transactions::Table, Transactions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Chargebacks::Table, Chargebacks::CardTransactionId)
                    .to(CardTransactions::Table, CardTransactions::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .to_owned();

        manager.create_table(chargebacks).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Chargebacks::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Chargebacks {
    Table,
    Id,
    InstitutionId,
    TransactionId,
    CardTransactionId,
    DisputeId,
    ChargebackAmount,
    ChargebackReasonCode,
    ChargebackReason,
    Status,
    InitiatedAt,
    RepresentmentDueDate,
    ResolvedAt,
    ProvisionalCreditAmount,
    FinalLiabilityAmount,
    NetworkReference,
    NetworkResponse,
    CreatedAt,
    UpdatedAt,
}
