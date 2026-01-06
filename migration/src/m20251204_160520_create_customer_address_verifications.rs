use sea_orm_migration::prelude::*;

use crate::{
    m20251204_150208_create_branches::Staff, m20251204_152312_create_customers::Customers,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let add_verify = Table::create()
            .table(CustomerAddressVerifications::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CustomerAddressVerifications::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(CustomerAddressVerifications::CustomerId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(CustomerAddressVerifications::DocumentType).string())
            .col(ColumnDef::new(CustomerAddressVerifications::DocumentUrl).string())
            .col(
                ColumnDef::new(CustomerAddressVerifications::IsAddressMatched)
                    .boolean()
                    .default(false),
            )
            .col(
                ColumnDef::new(CustomerAddressVerifications::VerificationStatus)
                    .custom("verification_status_type")
                    .default("PENDING"),
            )
            .col(
                ColumnDef::new(CustomerAddressVerifications::VerifiedAt).timestamp_with_time_zone(),
            )
            .col(ColumnDef::new(CustomerAddressVerifications::RejectedReason).string())
            .col(ColumnDef::new(CustomerAddressVerifications::VerifiedBy).big_integer())
            .col(
                ColumnDef::new(CustomerAddressVerifications::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(CustomerAddressVerifications::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        CustomerAddressVerifications::Table,
                        CustomerAddressVerifications::VerifiedBy,
                    )
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        CustomerAddressVerifications::Table,
                        CustomerAddressVerifications::CustomerId,
                    )
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(add_verify).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(CustomerAddressVerifications::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum CustomerAddressVerifications {
    Table,
    Id,
    CustomerId,
    DocumentType,
    DocumentUrl,
    IsAddressMatched,
    VerificationStatus,
    VerifiedBy,
    VerifiedAt,
    RejectedReason,
    CreatedAt,
    UpdatedAt,
}
