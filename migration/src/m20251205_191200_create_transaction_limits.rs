use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions,
    m20251205_150707_create_account_categories::AccountCategories,
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
                "CREATE TYPE transaction_limits_limit_type AS ENUM ('PER_TRANSACTION', 'DAILY', 'WEEKLY', 'MONTHLY')".to_string(),
            ))
            .await?;

        let trans_limit = Table::create()
            .table(TransactionLimits::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(TransactionLimits::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(TransactionLimits::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(TransactionLimits::TransactionChannelId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(TransactionLimits::CustomerType)
                    .custom("customer_type")
                    .default("INDIVIDUAL"),
            )
            .col(
                ColumnDef::new(TransactionLimits::AccountCategoryId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(TransactionLimits::LimitType)
                    .custom("transaction_limits_limit_type"),
            )
            .col(ColumnDef::new(TransactionLimits::MaxAmount).decimal_len(20, 4))
            .col(ColumnDef::new(TransactionLimits::MaxCount).integer())
            .col(ColumnDef::new(TransactionLimits::Currency).json_binary())
            .col(
                ColumnDef::new(TransactionLimits::IsActive)
                    .boolean()
                    .default(true),
            )
            .col(ColumnDef::new(TransactionLimits::EffectiveFrom).timestamp_with_time_zone())
            .col(ColumnDef::new(TransactionLimits::EffectiveTo).timestamp_with_time_zone())
            .col(ColumnDef::new(TransactionLimits::KycTier).json_binary())
            .col(
                ColumnDef::new(TransactionLimits::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(TransactionLimits::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(TransactionLimits::Table, TransactionLimits::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        TransactionLimits::Table,
                        TransactionLimits::TransactionChannelId,
                    )
                    .to(TransactionChannels::Table, TransactionChannels::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        TransactionLimits::Table,
                        TransactionLimits::AccountCategoryId,
                    )
                    .to(AccountCategories::Table, AccountCategories::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(trans_limit).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TransactionLimits::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum TransactionLimits {
    Table,
    Id,
    InstitutionId,
    TransactionChannelId,
    CustomerType,
    KycTier,
    AccountCategoryId,
    LimitType,
    MaxAmount,
    MaxCount,
    Currency,
    IsActive,
    EffectiveFrom,
    EffectiveTo,
    CreatedAt,
    UpdatedAt,
}
