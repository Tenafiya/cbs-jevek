use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_103935_create_countries::Countries, m20251204_152312_create_customers::Customers,
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
                "CREATE TYPE verification_status_type AS ENUM ('PENDING', 'VERIFIED', 'REJECTED', 'EXPIRED')"
                    .to_string(),
            ))
            .await?;

        let cus_iden = Table::create()
            .table(CustomerIdentifications::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CustomerIdentifications::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(CustomerIdentifications::CustomerId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(CustomerIdentifications::IdType).string())
            .col(ColumnDef::new(CustomerIdentifications::IdNumber).string())
            .col(
                ColumnDef::new(CustomerIdentifications::IssuingCountryId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(CustomerIdentifications::IssueDate).date())
            .col(ColumnDef::new(CustomerIdentifications::ExpiryDate).date())
            .col(ColumnDef::new(CustomerIdentifications::FrontImageUrl).string())
            .col(ColumnDef::new(CustomerIdentifications::BackImageUrl).string())
            .col(ColumnDef::new(CustomerIdentifications::SelfieImageUrl).string())
            .col(ColumnDef::new(CustomerIdentifications::FaceMatchScore).decimal_len(5, 4))
            .col(
                ColumnDef::new(CustomerIdentifications::FaceMatchPassed)
                    .boolean()
                    .default(false),
            )
            .col(
                ColumnDef::new(CustomerIdentifications::VerificationStatus)
                    .custom("verification_status_type")
                    .default("PENDING"),
            )
            .col(ColumnDef::new(CustomerIdentifications::VerifiedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(CustomerIdentifications::RejectionReason).string())
            .col(ColumnDef::new(CustomerIdentifications::Metadata).json_binary())
            .col(
                ColumnDef::new(CustomerIdentifications::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(CustomerIdentifications::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        CustomerIdentifications::Table,
                        CustomerIdentifications::CustomerId,
                    )
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        CustomerIdentifications::Table,
                        CustomerIdentifications::IssuingCountryId,
                    )
                    .to(Countries::Table, Countries::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(cus_iden).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(CustomerIdentifications::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum CustomerIdentifications {
    Table,
    Id,
    CustomerId,
    IdType,
    IdNumber,
    IssuingCountryId,
    IssueDate,
    ExpiryDate,
    FrontImageUrl,
    BackImageUrl,
    SelfieImageUrl,
    FaceMatchScore,
    FaceMatchPassed,
    VerificationStatus,
    VerifiedBy,
    VerifiedAt,
    RejectionReason,
    Metadata,
    CreatedAt,
    UpdatedAt,
}
