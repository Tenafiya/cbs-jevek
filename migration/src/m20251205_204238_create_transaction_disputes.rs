use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::m20251204_150208_create_branches::Staff;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE transaction_dispute_type AS ENUM ('UNAUTHORIZED', 'DUPLICATE', 'INCORRECT_AMOUNT', 'NOT_RECEIVED')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE transaction_dispute_status AS ENUM ('OPEN', 'INVESTIGATION', 'RESOLVED', 'REJECTED')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE transaction_priority AS ENUM ('LOW', 'MEDIUM', 'HIGH', 'CRITICAL')"
                    .to_string(),
            ))
            .await?;

        let disputes = Table::create()
            .table(TransactionDisputes::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(TransactionDisputes::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(TransactionDisputes::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(TransactionDisputes::TransactionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(TransactionDisputes::CustomerId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(TransactionDisputes::DisputeType).custom("transaction_dispute_type"),
            )
            .col(ColumnDef::new(TransactionDisputes::Description).string())
            .col(ColumnDef::new(TransactionDisputes::AmountDisputed).decimal_len(20, 4))
            .col(ColumnDef::new(TransactionDisputes::SupportingDocuments).json_binary())
            .col(
                ColumnDef::new(TransactionDisputes::Status)
                    .custom("transaction_dispute_status")
                    .default("OPEN"),
            )
            .col(ColumnDef::new(TransactionDisputes::Priority).custom("transaction_priority"))
            .col(ColumnDef::new(TransactionDisputes::Resolution).string())
            .col(
                ColumnDef::new(TransactionDisputes::RefundAmount)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(ColumnDef::new(TransactionDisputes::ResolvedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(TransactionDisputes::SlaDeadline).timestamp_with_time_zone())
            .col(ColumnDef::new(TransactionDisputes::AssignedTo).big_integer())
            .col(ColumnDef::new(TransactionDisputes::ResolvedBy).big_integer())
            .col(
                ColumnDef::new(TransactionDisputes::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(TransactionDisputes::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(TransactionDisputes::Table, TransactionDisputes::AssignedTo)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(TransactionDisputes::Table, TransactionDisputes::ResolvedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(disputes).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TransactionDisputes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum TransactionDisputes {
    Table,
    Id,
    InstitutionId,
    TransactionId,
    CustomerId,
    DisputeType,
    Description,
    AmountDisputed,
    SupportingDocuments,
    Status,
    Priority,
    AssignedTo,
    Resolution,
    RefundAmount,
    ResolvedBy,
    ResolvedAt,
    SlaDeadline,
    CreatedAt,
    UpdatedAt,
}
