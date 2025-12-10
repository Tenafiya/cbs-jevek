use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions,
    m20251204_152312_create_customers::Customers, m20251205_154503_create_accounts::Accounts,
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
                "CREATE TYPE card_type AS ENUM ('DEBIT', 'CREDIT', 'PREPAID')".to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE card_form AS ENUM ('PHYSICAL', 'VIRTUAL')".to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE card_status AS ENUM ('INACTIVE', 'ACTIVE', 'BLOCKED', 'EXPIRED', 'CANCELLED', 'FROZEN')".to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE card_brand AS ENUM ('VISA', 'MASTERCARD')".to_string(),
            ))
            .await?;

        let cards = Table::create()
            .table(Cards::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Cards::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(Cards::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(Cards::CustomerId).big_integer().not_null())
            .col(ColumnDef::new(Cards::AccountId).big_integer().not_null())
            .col(ColumnDef::new(Cards::CardNumberHashed).string().not_null())
            .col(ColumnDef::new(Cards::CardNumberMasked).string().not_null())
            .col(
                ColumnDef::new(Cards::CardFormFactor)
                    .custom("card_form")
                    .default("PHYSICAL"),
            )
            .col(ColumnDef::new(Cards::CardType).custom("card_type"))
            .col(ColumnDef::new(Cards::CardBrand).custom("card_brand"))
            .col(ColumnDef::new(Cards::CardBin).string())
            .col(ColumnDef::new(Cards::LastFour).string_len(4))
            .col(ColumnDef::new(Cards::ExpiryMonth).integer())
            .col(ColumnDef::new(Cards::ExpiryYear).integer())
            .col(ColumnDef::new(Cards::CvvHash).string())
            .col(ColumnDef::new(Cards::PinHash).string())
            .col(ColumnDef::new(Cards::PinRetries).integer().default(0))
            .col(ColumnDef::new(Cards::PinBlockedAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(Cards::CardStatus)
                    .custom("card_status")
                    .default("INACTIVE"),
            )
            .col(ColumnDef::new(Cards::ActivationDate).timestamp_with_time_zone())
            .col(ColumnDef::new(Cards::BlockingReason).string())
            .col(ColumnDef::new(Cards::DailyAtmLimit).decimal_len(20, 4))
            .col(ColumnDef::new(Cards::DailyPosLimit).decimal_len(20, 4))
            .col(ColumnDef::new(Cards::DailyOnlineLimit).decimal_len(20, 4))
            .col(ColumnDef::new(Cards::CardholderName).string())
            .col(ColumnDef::new(Cards::DeliveryAddress).json_binary())
            .col(ColumnDef::new(Cards::DeliveryStatus).string())
            .col(ColumnDef::new(Cards::DeliveredAt).timestamp_with_time_zone())
            .col(ColumnDef::new(Cards::CardProvider).string())
            .col(ColumnDef::new(Cards::CardProviderId).string())
            .col(ColumnDef::new(Cards::CustomFields).json_binary())
            .col(
                ColumnDef::new(Cards::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Cards::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Cards::Table, Cards::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Cards::Table, Cards::CustomerId)
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Cards::Table, Cards::AccountId)
                    .to(Accounts::Table, Accounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(cards).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Cards::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Cards {
    Table,
    Id,
    InstitutionId,
    CustomerId,
    AccountId,
    CardNumberHashed,
    CardNumberMasked,
    CardType,
    CardFormFactor,
    CardBrand,
    CardBin,
    LastFour,
    ExpiryMonth,
    ExpiryYear,
    CvvHash,
    PinHash,
    PinRetries,
    PinBlockedAt,
    CardStatus,
    ActivationDate,
    BlockingReason,
    DailyAtmLimit,
    DailyPosLimit,
    DailyOnlineLimit,
    CardholderName,
    DeliveryAddress,
    DeliveryStatus,
    DeliveredAt,
    CardProvider,
    CardProviderId,
    CustomFields,
    CreatedAt,
    UpdatedAt,
}
