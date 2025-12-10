use sea_orm_migration::prelude::*;

use crate::m20251204_112805_create_institutions::Institutions;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let trn_channels = Table::create()
            .table(TransactionChannels::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(TransactionChannels::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(TransactionChannels::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(TransactionChannels::ChannelName).string())
            .col(
                ColumnDef::new(TransactionChannels::ChannelCode)
                    .string()
                    .unique_key(),
            )
            .col(ColumnDef::new(TransactionChannels::Description).string())
            .col(
                ColumnDef::new(TransactionChannels::IsActive)
                    .boolean()
                    .default(false),
            )
            .col(
                ColumnDef::new(TransactionChannels::RequiresMakerChecker)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(TransactionChannels::Metadata).json_binary())
            .col(
                ColumnDef::new(TransactionChannels::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(TransactionChannels::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        TransactionChannels::Table,
                        TransactionChannels::InstitutionId,
                    )
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(trn_channels).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TransactionChannels::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum TransactionChannels {
    Table,
    Id,
    InstitutionId,
    ChannelName,
    ChannelCode,
    Description,
    IsActive,
    RequiresMakerChecker,
    Metadata,
    CreatedAt,
    UpdatedAt,
}
