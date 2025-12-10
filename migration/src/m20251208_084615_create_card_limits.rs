use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::m20251207_212120_create_cards::Cards;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE card_limit_type AS ENUM ('ATM_WITHDRAWAL', 'POS_PURCHASE', 'ONLINE_PURCHASE', 'DAILY_TOTAL')".to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE card_limit_reset AS ENUM ('DAILY', 'MONTHLY', 'QUARTERLY', 'YEARLY')"
                    .to_string(),
            ))
            .await?;

        let limit = Table::create()
            .table(CardLimits::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CardLimits::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(CardLimits::CardId).big_integer().not_null())
            .col(ColumnDef::new(CardLimits::LimitType).custom("card_limit_type"))
            .col(ColumnDef::new(CardLimits::LimitAmount).decimal_len(20, 4))
            .col(
                ColumnDef::new(CardLimits::CurrentUsage)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(CardLimits::ResetFreq)
                    .custom("card_limit_reset")
                    .default("DAILY"),
            )
            .col(
                ColumnDef::new(CardLimits::LastResetAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(ColumnDef::new(CardLimits::EffectiveFrom).timestamp_with_time_zone())
            .col(ColumnDef::new(CardLimits::EffectiveTo).timestamp_with_time_zone())
            .col(
                ColumnDef::new(CardLimits::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(CardLimits::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(CardLimits::Table, CardLimits::CardId)
                    .to(Cards::Table, Cards::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(limit).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CardLimits::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum CardLimits {
    Table,
    Id,
    CardId,
    LimitType,
    LimitAmount,
    CurrentUsage,
    ResetFreq,
    LastResetAt,
    EffectiveFrom,
    EffectiveTo,
    CreatedAt,
    UpdatedAt,
}
