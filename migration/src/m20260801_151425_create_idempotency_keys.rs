use sea_orm_migration::prelude::*;

use crate::{
    m20251204_112805_create_institutions::Institutions,
    m20251204_152312_create_customers::Customers, m20251205_154503_create_accounts::Accounts,
    m20251205_193221_create_transactions::Transactions,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE TYPE idem_key_status AS ENUM ('PENDING', 'COMPLETED', 'PROCESSING', 'FAILED', 'EXPIRED', 'CANCELLED')
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE TYPE idempotency_channel AS ENUM ('WEB', 'MOBILE_APP', 'API', 'OPEN_BANKING', 'ATM', 'POS', 'BRANCH', 'TELLER', 'CALL_CENTER', 'BACK_OFFICE', 'ADMIN_PORTAL', 'BATCH_JOB', 'SCHEDULED_TASK', 'MESSAGE_QUEUE', 'WEBHOOK', 'USSD', 'SMS', 'AGENT_BANKING', 'THIRD_PARTY')
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE TYPE idempotency_operation AS ENUM ('ACCOUNT_CREATE', 'ACCOUNT_UPDATE', 'ACCOUNT_CLOSE', 'ACCOUNT_FREEZE', 'ACCOUNT_UNFREEZE', 'DEPOSIT', 'WITHDRAWAL', 'TRANSFER', 'INTERNAL_TRANSFER', 'EXTERNAL_TRANSFER', 'PAYMENT', 'BILL_PAYMENT', 'CARD_PAYMENT', 'CARD_ISSUE', 'CARD_REPLACEMENT', 'CARD_BLOCK', 'CARD_UNBLOCK',
                    'LOAN_APPLICATION', 'LOAN_DISBURSEMENT', 'LOAN_REPAYMENT', 'INTEREST_POSTING', 'FEE_POSTING', 'REVERSAL', 'REFUND', 'STANDING_ORDER', 'DIRECT_DEBIT', 'FOREIGN_EXCHANGE', 'CHEQUE_DEPOSIT', 'CHEQUE_CLEARING', 'CASH_DEPOSIT', 'CASH_WITHDRAWAL', 'MERCHANT_SETTLEMENT')
                "#,
            )
            .await?;

        let idem_key = Table::create()
            .table(IdempotencyKeys::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(IdempotencyKeys::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(IdempotencyKeys::Slug)
                    .string()
                    .not_null()
                    .unique_key(),
            )
            .col(
                ColumnDef::new(IdempotencyKeys::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(IdempotencyKeys::CustomerId).big_integer())
            .col(ColumnDef::new(IdempotencyKeys::AccountId).big_integer())
            .col(ColumnDef::new(IdempotencyKeys::TransactionId).big_integer())
            .col(ColumnDef::new(IdempotencyKeys::TransactionGroupId).uuid())
            .col(
                ColumnDef::new(IdempotencyKeys::IdempotencyKey)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(IdempotencyKeys::Operation)
                    .custom("idempotency_operation")
                    .not_null(),
            )
            .col(
                ColumnDef::new(IdempotencyKeys::RequestMethod)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(IdempotencyKeys::RequestPath)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(IdempotencyKeys::RequestHash)
                    .string_len(64)
                    .not_null(),
            )
            .col(ColumnDef::new(IdempotencyKeys::Channel).custom("idempotency_channel"))
            .col(
                ColumnDef::new(IdempotencyKeys::Status)
                    .custom("idem_key_status")
                    .default("PENDING"),
            )
            .col(ColumnDef::new(IdempotencyKeys::HttpStatus).small_integer())
            .col(ColumnDef::new(IdempotencyKeys::ResponseBody).json_binary())
            .col(ColumnDef::new(IdempotencyKeys::ErrorMessage).text())
            .col(ColumnDef::new(IdempotencyKeys::LockedAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(IdempotencyKeys::ExpiresAt)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(ColumnDef::new(IdempotencyKeys::CompletedAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(IdempotencyKeys::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(IdempotencyKeys::UpdatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(IdempotencyKeys::Table, IdempotencyKeys::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(IdempotencyKeys::Table, IdempotencyKeys::CustomerId)
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(IdempotencyKeys::Table, IdempotencyKeys::AccountId)
                    .to(Accounts::Table, Accounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(IdempotencyKeys::Table, IdempotencyKeys::TransactionId)
                    .to(Transactions::Table, Transactions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(idem_key).await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE UNIQUE INDEX uq_idempotency_keys
                    ON idempotency_keys(
                        institution_id,
                        operation,
                        idempotency_key
                    )
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE INDEX idx_idempotency_transaction
                    ON idempotency_keys(transaction_id);
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE INDEX idx_idempotency_transaction_group
                    ON idempotency_keys(transaction_group_id);
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE INDEX idx_idempotency_trans_id_group
                    ON idempotency_keys(transaction_id, transaction_group_id);
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE INDEX idx_idempotency_customer
                    ON idempotency_keys(customer_id);
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE INDEX idx_idempotency_account
                    ON idempotency_keys(account_id);
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE INDEX idx_idempotency_status
                    ON idempotency_keys(status);
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE INDEX idx_idempotency_expires
                    ON idempotency_keys(expires_at);
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE INDEX idx_idempotency_processing
                    ON idempotency_keys(status)
                    WHERE status = 'PROCESSING';
                "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IdempotencyKeys::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum IdempotencyKeys {
    Table,
    Id,
    Slug,
    InstitutionId,
    CustomerId,
    AccountId,
    TransactionId,
    TransactionGroupId,
    IdempotencyKey,
    Operation,
    RequestMethod,
    RequestPath,
    RequestHash,
    Channel,
    Status,
    HttpStatus,
    ResponseBody,
    ErrorMessage,
    LockedAt,
    ExpiresAt,
    CompletedAt,
    CreatedAt,
    UpdatedAt,
}
