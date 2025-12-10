use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251207_212120_create_cards::Cards,
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
                "CREATE TYPE card_transactions_trans_type AS ENUM ('ATM', 'POS', 'ONLINE', 'CONTACTLESS')".to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE card_transactions_trans_channel AS ENUM ('DOMESTIC', 'INTERNATIONAL')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE card_transactions_status AS ENUM ('PENDING', 'APPROVED', 'DECLINED', 'REVERSED')".to_string(),
            ))
            .await?;

        let card_trans = Table::create()
            .table(CardTransactions::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CardTransactions::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(CardTransactions::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CardTransactions::CardId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CardTransactions::TransactionReference)
                    .string()
                    .unique_key(),
            )
            .col(ColumnDef::new(CardTransactions::ExternalReference).string())
            .col(ColumnDef::new(CardTransactions::MerchantName).string())
            .col(ColumnDef::new(CardTransactions::MerchantCategoryCode).string())
            .col(ColumnDef::new(CardTransactions::MerchantId).string())
            .col(
                ColumnDef::new(CardTransactions::TransactionType)
                    .custom("card_transactions_trans_type")
                    .not_null(),
            )
            .col(
                ColumnDef::new(CardTransactions::TransactionChannel)
                    .custom("card_transactions_trans_channel"),
            )
            .col(ColumnDef::new(CardTransactions::Amount).decimal_len(20, 4))
            .col(ColumnDef::new(CardTransactions::Currency).json_binary())
            .col(ColumnDef::new(CardTransactions::BillingAmount).decimal_len(20, 4))
            .col(ColumnDef::new(CardTransactions::BillingCurrency).json_binary())
            .col(ColumnDef::new(CardTransactions::TransactionDate).timestamp_with_time_zone())
            .col(ColumnDef::new(CardTransactions::SettlementDate).date())
            .col(ColumnDef::new(CardTransactions::Status).custom("card_transactions_status"))
            .col(ColumnDef::new(CardTransactions::ResponseCode).string())
            .col(ColumnDef::new(CardTransactions::ResponseMessage).json_binary())
            .col(ColumnDef::new(CardTransactions::AtmLocation).json_binary())
            .col(ColumnDef::new(CardTransactions::PosTerminalId).string())
            .col(
                ColumnDef::new(CardTransactions::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(CardTransactions::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(CardTransactions::Table, CardTransactions::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(CardTransactions::Table, CardTransactions::CardId)
                    .to(Cards::Table, Cards::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(card_trans).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CardTransactions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum CardTransactions {
    Table,
    Id,
    InstitutionId,
    CardId,
    TransactionReference,
    ExternalReference,
    MerchantName,
    MerchantCategoryCode,
    MerchantId,
    TransactionType,
    TransactionChannel,
    Amount,
    Currency,
    BillingAmount,
    BillingCurrency,
    TransactionDate,
    SettlementDate,
    Status,
    ResponseCode,
    ResponseMessage,
    AtmLocation,
    PosTerminalId,
    CreatedAt,
    UpdatedAt,
}
