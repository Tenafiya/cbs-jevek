use sea_orm_migration::prelude::*;

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
    m20251204_152312_create_customers::Customers,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let bl = Table::create()
            .table(CustomerBlacklistRecords::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CustomerBlacklistRecords::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(CustomerBlacklistRecords::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CustomerBlacklistRecords::CustomerId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(CustomerBlacklistRecords::ExternalCustomerNumber).string())
            .col(ColumnDef::new(CustomerBlacklistRecords::BlacklistType).string())
            .col(ColumnDef::new(CustomerBlacklistRecords::BlacklistReason).string())
            .col(ColumnDef::new(CustomerBlacklistRecords::BlacklistSource).string())
            .col(
                ColumnDef::new(CustomerBlacklistRecords::IsActive)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(CustomerBlacklistRecords::ReportedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(CustomerBlacklistRecords::RemovedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(CustomerBlacklistRecords::RemovalReason).string())
            .col(ColumnDef::new(CustomerBlacklistRecords::ReportedBy).big_integer())
            .col(ColumnDef::new(CustomerBlacklistRecords::RemovedBy).big_integer())
            .col(
                ColumnDef::new(CustomerBlacklistRecords::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(CustomerBlacklistRecords::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        CustomerBlacklistRecords::Table,
                        CustomerBlacklistRecords::InstitutionId,
                    )
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        CustomerBlacklistRecords::Table,
                        CustomerBlacklistRecords::ReportedBy,
                    )
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        CustomerBlacklistRecords::Table,
                        CustomerBlacklistRecords::RemovedBy,
                    )
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        CustomerBlacklistRecords::Table,
                        CustomerBlacklistRecords::CustomerId,
                    )
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(bl).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(CustomerBlacklistRecords::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum CustomerBlacklistRecords {
    Table,
    Id,
    InstitutionId,
    CustomerId,
    ExternalCustomerNumber,
    BlacklistType,
    BlacklistReason,
    BlacklistSource,
    IsActive,
    ReportedBy,
    ReportedAt,
    RemovedAt,
    RemovedBy,
    RemovalReason,
    CreatedAt,
    UpdatedAt,
}
