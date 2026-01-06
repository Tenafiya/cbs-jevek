use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
    m20251204_152312_create_customers::Customers, m20251205_154503_create_accounts::Accounts,
    m20251205_165925_create_transaction_channels::TransactionChannels,
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
                "CREATE TYPE transaction_category_type AS ENUM ('CASH_DEPOSIT', 'CASH_WITHDRAWAL', 'TRANSFER', 'LOAN_DISBURSEMENT', 'LOAN_REPAYMENT')".to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE transaction_status AS ENUM ('PENDING', 'COMPLETED', 'FAILED', 'REVERSED', 'DISPUTED', 'CANCELLED')".to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE transaction_type AS ENUM ('DEBIT', 'CREDIT')".to_string(),
            ))
            .await?;

        let trans = Table::create()
            .table(Transactions::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Transactions::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(Transactions::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(Transactions::TransactionChannelId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(Transactions::TransactionReference).string())
            .col(
                ColumnDef::new(Transactions::ParentTransactionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(Transactions::ReversalReason).string())
            .col(ColumnDef::new(Transactions::DebitAccountId).big_integer())
            .col(ColumnDef::new(Transactions::CreditAccountId).big_integer())
            .col(ColumnDef::new(Transactions::DebitCustomerId).big_integer())
            .col(ColumnDef::new(Transactions::CreditCustomerId).big_integer())
            .col(
                ColumnDef::new(Transactions::Amount)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(ColumnDef::new(Transactions::Currency).json_binary())
            .col(
                ColumnDef::new(Transactions::FeeAmount)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(Transactions::VatAmount)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(Transactions::TotalAmount)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(ColumnDef::new(Transactions::TransactionType).custom("transaction_type"))
            .col(
                ColumnDef::new(Transactions::TransactionCategory)
                    .custom("transaction_category_type"),
            )
            .col(ColumnDef::new(Transactions::Description).string())
            .col(ColumnDef::new(Transactions::Narrative).string())
            .col(ColumnDef::new(Transactions::ExternalReference).string())
            .col(ColumnDef::new(Transactions::Status).custom("transaction_status"))
            .col(ColumnDef::new(Transactions::PostedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(Transactions::CompletedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(Transactions::FailedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(Transactions::FailureReason).string())
            .col(ColumnDef::new(Transactions::ValueDate).date())
            .col(ColumnDef::new(Transactions::IpAddress).string())
            .col(ColumnDef::new(Transactions::DeviceId).json_binary())
            .col(ColumnDef::new(Transactions::Location).json_binary())
            .col(
                ColumnDef::new(Transactions::IsSuspicious)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(Transactions::AmlAlertId).string())
            .col(ColumnDef::new(Transactions::CustomFields).json_binary())
            .col(ColumnDef::new(Transactions::ApprovedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(Transactions::CreatedBy).big_integer())
            .col(ColumnDef::new(Transactions::ApprovedBy).big_integer())
            .col(
                ColumnDef::new(Transactions::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Transactions::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Transactions::Table, Transactions::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Transactions::Table, Transactions::TransactionChannelId)
                    .to(TransactionChannels::Table, TransactionChannels::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Transactions::Table, Transactions::ParentTransactionId)
                    .to(Transactions::Table, Transactions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Transactions::Table, Transactions::DebitAccountId)
                    .to(Accounts::Table, Accounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Transactions::Table, Transactions::CreditAccountId)
                    .to(Accounts::Table, Accounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Transactions::Table, Transactions::DebitCustomerId)
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Transactions::Table, Transactions::CreditCustomerId)
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Transactions::Table, Transactions::CreatedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Transactions::Table, Transactions::ApprovedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(trans).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Transactions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Transactions {
    Table,
    Id,
    InstitutionId,
    TransactionChannelId,
    TransactionReference,
    ParentTransactionId,
    ReversalReason,
    DebitAccountId,
    CreditAccountId,
    DebitCustomerId,
    CreditCustomerId,
    Amount,
    Currency,
    FeeAmount,
    VatAmount,
    TotalAmount,
    TransactionType,
    TransactionCategory,
    Description,
    Narrative,
    ExternalReference,
    Status,
    PostedAt,
    CompletedAt,
    FailedAt,
    FailureReason,
    ValueDate,
    IpAddress,
    DeviceId,
    Location,
    IsSuspicious,
    AmlAlertId,
    CustomFields,
    CreatedBy,
    ApprovedBy,
    ApprovedAt,
    CreatedAt,
    UpdatedAt,
}
