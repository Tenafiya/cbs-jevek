use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
    m20251204_152312_create_customers::Customers, m20251205_154503_create_accounts::Accounts,
    m20251210_165956_create_fee_types::FeeTypes,
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
                "CREATE TYPE fee_application_status AS ENUM ('PENDING', 'CHARGED', 'WAIVED', 'REVERSED')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE fee_application_ref_type AS ENUM ('TRANSACTION', 'ACCOUNT', 'LOAN')"
                    .to_string(),
            ))
            .await?;

        let fee_applications = Table::create()
            .table(FeeApplications::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(FeeApplications::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(FeeApplications::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(FeeApplications::FeeTypeId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(FeeApplications::ReferenceId).big_integer())
            .col(ColumnDef::new(FeeApplications::ReferenceType).custom("fee_application_ref_type"))
            .col(ColumnDef::new(FeeApplications::CustomerId).big_integer())
            .col(ColumnDef::new(FeeApplications::AccountId).big_integer())
            .col(
                ColumnDef::new(FeeApplications::BaseAmount)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(FeeApplications::FeeAmount)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(FeeApplications::VatAmount)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(FeeApplications::TotalAmount)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(FeeApplications::Status)
                    .custom("fee_application_status")
                    .default("PENDING"),
            )
            .col(ColumnDef::new(FeeApplications::WaivedBy).big_integer())
            .col(ColumnDef::new(FeeApplications::WaivedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(FeeApplications::WaiverReason).text())
            .col(ColumnDef::new(FeeApplications::WaiverApprovedBy).big_integer())
            .col(
                ColumnDef::new(FeeApplications::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(FeeApplications::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(FeeApplications::Table, FeeApplications::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(FeeApplications::Table, FeeApplications::FeeTypeId)
                    .to(FeeTypes::Table, FeeTypes::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(FeeApplications::Table, FeeApplications::CustomerId)
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(FeeApplications::Table, FeeApplications::AccountId)
                    .to(Accounts::Table, Accounts::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(FeeApplications::Table, FeeApplications::WaivedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(FeeApplications::Table, FeeApplications::WaiverApprovedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .index(
                Index::create()
                    .name("idx_fee_applications_status")
                    .table(FeeApplications::Table)
                    .col(FeeApplications::Status),
            )
            .index(
                Index::create()
                    .name("idx_fee_applications_customer")
                    .table(FeeApplications::Table)
                    .col(FeeApplications::CustomerId),
            )
            .index(
                Index::create()
                    .name("idx_fee_applications_account")
                    .table(FeeApplications::Table)
                    .col(FeeApplications::AccountId),
            )
            .to_owned();

        manager.create_table(fee_applications).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FeeApplications::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum FeeApplications {
    Table,
    Id,
    InstitutionId,
    FeeTypeId,
    ReferenceId,
    ReferenceType,
    CustomerId,
    AccountId,
    BaseAmount,
    FeeAmount,
    VatAmount,
    TotalAmount,
    Status,
    WaivedBy,
    WaivedAt,
    WaiverReason,
    WaiverApprovedBy,
    CreatedAt,
    UpdatedAt,
}
