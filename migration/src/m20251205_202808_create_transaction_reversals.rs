use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::m20251205_193221_create_transactions::Transactions;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE transaction_reversal_typs AS ENUM ('CUSTOMER_REQUEST', 'FRAUD', 'ERROR', 'COMPLIANCE')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE transaction_reversal_status AS ENUM ('PENDING', 'PROCESSED', 'CANCELLED')"
                    .to_string(),
            ))
            .await?;

        let reversal = Table::create()
            .table(TransactionReversals::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(TransactionReversals::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(TransactionReversals::OrginalTransactionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(TransactionReversals::ReversalTransactionId).big_integer())
            .col(
                ColumnDef::new(TransactionReversals::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(TransactionReversals::ReversalType)
                    .custom("transaction_reversal_typs"),
            )
            .col(ColumnDef::new(TransactionReversals::Reason).string())
            .col(ColumnDef::new(TransactionReversals::Amount).decimal_len(20, 4))
            .col(ColumnDef::new(TransactionReversals::Status).custom("transaction_reversal_status"))
            .col(ColumnDef::new(TransactionReversals::RequestedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(TransactionReversals::ApprovedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(TransactionReversals::MakerCheckerWorkflowId).big_integer())
            .col(
                ColumnDef::new(TransactionReversals::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(TransactionReversals::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        TransactionReversals::Table,
                        TransactionReversals::OrginalTransactionId,
                    )
                    .to(Transactions::Table, Transactions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        TransactionReversals::Table,
                        TransactionReversals::ReversalTransactionId,
                    )
                    .to(Transactions::Table, Transactions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(reversal).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TransactionReversals::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum TransactionReversals {
    Table,
    Id,
    OrginalTransactionId,
    ReversalTransactionId,
    InstitutionId,
    ReversalType,
    Reason,
    Amount,
    Status,
    RequestedBy,
    RequestedAt,
    ApprovedBy,
    ApprovedAt,
    MakerCheckerWorkflowId,
    CreatedAt,
    UpdatedAt,
}
