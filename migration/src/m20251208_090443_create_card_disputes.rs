use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions,
    m20251204_152312_create_customers::Customers,
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
                "CREATE TYPE card_dispute_type AS ENUM ('FRAUD', 'DUPLICATE', 'NOT_RECEIVED', 'CANCELLED')".to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE card_dispute_status AS ENUM ('OPEN', 'INVESTIGATION', 'WON', 'LOST', 'CLOSED')".to_string(),
            ))
            .await?;

        let disputes = Table::create()
            .table(CardDisputes::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CardDisputes::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(CardDisputes::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CardDisputes::CustomerId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CardDisputes::CardTransactionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CardDisputes::DisputeType)
                    .custom("card_dispute_type")
                    .not_null(),
            )
            .col(ColumnDef::new(CardDisputes::DisputeReason).string())
            .col(
                ColumnDef::new(CardDisputes::AmountDisputed)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(CardDisputes::ProvisionalCreditAmount)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(CardDisputes::Status)
                    .custom("card_dispute_status")
                    .default("OPEN"),
            )
            .col(
                ColumnDef::new(CardDisputes::IsChargebackInitiated)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(CardDisputes::ChargebackDate).date())
            .col(ColumnDef::new(CardDisputes::Resolution).string())
            .col(ColumnDef::new(CardDisputes::FinalDecision).string())
            .col(ColumnDef::new(CardDisputes::ResolvedAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(CardDisputes::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(CardDisputes::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(CardDisputes::Table, CardDisputes::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(CardDisputes::Table, CardDisputes::CustomerId)
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(CardDisputes::Table, CardDisputes::CardTransactionId)
                    .to(CardTransactions::Table, CardTransactions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(disputes).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CardDisputes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum CardDisputes {
    Table,
    Id,
    InstitutionId,
    CustomerId,
    CardTransactionId,
    DisputeType,
    DisputeReason,
    AmountDisputed,
    ProvisionalCreditAmount,
    Status,
    AssignedTo,
    IsChargebackInitiated,
    ChargebackDate,
    Resolution,
    FinalDecision,
    ResolvedBy,
    ResolvedAt,
    CreatedAt,
    UpdatedAt,
}
