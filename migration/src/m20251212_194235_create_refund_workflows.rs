use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
    m20251205_154503_create_accounts::Accounts, m20251205_193221_create_transactions::Transactions,
    m20251207_184227_create_wallets::Wallets,
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
                "CREATE TYPE refund_workflow_refund_type AS ENUM ('FAILED_TRANSACTION', 'FRAUD', 'CUSTOMER_SERVICE')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE refund_workflow_approval_status AS ENUM ('PENDING', 'APPROVED', 'REJECTED', 'PROCESSED')"
                    .to_string(),
            ))
            .await?;

        let refund_workflows = Table::create()
            .table(RefundWorkflows::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(RefundWorkflows::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(RefundWorkflows::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(RefundWorkflows::TransactionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(RefundWorkflows::DisputeId).big_integer())
            .col(
                ColumnDef::new(RefundWorkflows::RefundAmount)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(RefundWorkflows::FeeRefundAmount)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(RefundWorkflows::RefundReason)
                    .text()
                    .not_null(),
            )
            .col(
                ColumnDef::new(RefundWorkflows::RefundType)
                    .custom("refund_workflow_refund_type")
                    .not_null(),
            )
            .col(
                ColumnDef::new(RefundWorkflows::RequesterId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(RefundWorkflows::RequesterNotes).text())
            .col(
                ColumnDef::new(RefundWorkflows::RequestedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(ColumnDef::new(RefundWorkflows::ApproverId).big_integer())
            .col(
                ColumnDef::new(RefundWorkflows::ApprovalStatus)
                    .custom("refund_workflow_approval_status")
                    .default("PENDING"),
            )
            .col(ColumnDef::new(RefundWorkflows::ApprovalNotes).text())
            .col(ColumnDef::new(RefundWorkflows::ApprovedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(RefundWorkflows::ProcessedBy).big_integer())
            .col(ColumnDef::new(RefundWorkflows::ProcessedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(RefundWorkflows::RefundReference).string())
            .col(ColumnDef::new(RefundWorkflows::RefundToAccountId).big_integer())
            .col(ColumnDef::new(RefundWorkflows::RefundToWalletId).big_integer())
            .col(
                ColumnDef::new(RefundWorkflows::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(RefundWorkflows::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(RefundWorkflows::Table, RefundWorkflows::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(RefundWorkflows::Table, RefundWorkflows::TransactionId)
                    .to(Transactions::Table, Transactions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(RefundWorkflows::Table, RefundWorkflows::RequesterId)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(RefundWorkflows::Table, RefundWorkflows::ApproverId)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(RefundWorkflows::Table, RefundWorkflows::ProcessedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(RefundWorkflows::Table, RefundWorkflows::RefundToAccountId)
                    .to(Accounts::Table, Accounts::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(RefundWorkflows::Table, RefundWorkflows::RefundToWalletId)
                    .to(Wallets::Table, Wallets::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .index(
                Index::create()
                    .name("idx_refund_workflows_status")
                    .table(RefundWorkflows::Table)
                    .col(RefundWorkflows::ApprovalStatus),
            )
            .index(
                Index::create()
                    .name("idx_refund_workflows_transaction")
                    .table(RefundWorkflows::Table)
                    .col(RefundWorkflows::TransactionId),
            )
            .index(
                Index::create()
                    .name("idx_refund_workflows_type")
                    .table(RefundWorkflows::Table)
                    .col(RefundWorkflows::RefundType),
            )
            .to_owned();

        manager.create_table(refund_workflows).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RefundWorkflows::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum RefundWorkflows {
    Table,
    Id,
    InstitutionId,
    TransactionId,
    DisputeId,
    RefundAmount,
    FeeRefundAmount,
    RefundReason,
    RefundType,
    RequesterId,
    RequesterNotes,
    RequestedAt,
    ApproverId,
    ApprovalStatus,
    ApprovalNotes,
    ApprovedAt,
    ProcessedBy,
    ProcessedAt,
    RefundReference,
    RefundToAccountId,
    RefundToWalletId,
    CreatedAt,
    UpdatedAt,
}
