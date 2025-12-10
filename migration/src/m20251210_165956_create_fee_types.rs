use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
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
                "CREATE TYPE fee_types_category AS ENUM ('TRANSACTION', 'MAINTENANCE', 'LOAN_PROCESSING', 'CARD', 'PENALTY')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE fee_types_calc_method AS ENUM ('PERCENTAGE', 'FLAT', 'TIERED', 'SLAB')"
                    .to_string(),
            ))
            .await?;

        let fee_types = Table::create()
            .table(FeeTypes::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(FeeTypes::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(FeeTypes::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(FeeTypes::FeeName).string().not_null())
            .col(ColumnDef::new(FeeTypes::FeeCode).string().not_null())
            .col(
                ColumnDef::new(FeeTypes::FeeCategory)
                    .custom("fee_types_category")
                    .not_null(),
            )
            .col(
                ColumnDef::new(FeeTypes::FeeCalculationMethod)
                    .custom("fee_types_calc_method")
                    .not_null(),
            )
            .col(
                ColumnDef::new(FeeTypes::FeeValue)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(FeeTypes::MinimumFee)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(ColumnDef::new(FeeTypes::MaximumFee).decimal_len(20, 4))
            .col(ColumnDef::new(FeeTypes::Currency).json_binary())
            .col(
                ColumnDef::new(FeeTypes::IsVatApplicable)
                    .boolean()
                    .default(false),
            )
            .col(
                ColumnDef::new(FeeTypes::VatRate)
                    .decimal_len(10, 6)
                    .default(0.00),
            )
            .col(ColumnDef::new(FeeTypes::AppliesToTransactionTypes).json_binary())
            .col(ColumnDef::new(FeeTypes::AppliesToAccounts).json_binary())
            .col(ColumnDef::new(FeeTypes::AppliesToCustomers).json_binary())
            .col(
                ColumnDef::new(FeeTypes::IsWaiverAllowed)
                    .boolean()
                    .default(false),
            )
            .col(
                ColumnDef::new(FeeTypes::IsWaiverApprovalRequired)
                    .boolean()
                    .default(true),
            )
            .col(ColumnDef::new(FeeTypes::IsActive).boolean().default(true))
            .col(ColumnDef::new(FeeTypes::EffectiveFrom).timestamp_with_time_zone())
            .col(ColumnDef::new(FeeTypes::EffectiveTo).timestamp_with_time_zone())
            .col(ColumnDef::new(FeeTypes::CreatedBy).big_integer())
            .col(
                ColumnDef::new(FeeTypes::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(FeeTypes::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(FeeTypes::Table, FeeTypes::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(FeeTypes::Table, FeeTypes::CreatedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .index(
                Index::create()
                    .name("uk_fee_types_code")
                    .table(FeeTypes::Table)
                    .col(FeeTypes::InstitutionId)
                    .col(FeeTypes::FeeCode)
                    .unique(),
            )
            .index(
                Index::create()
                    .name("idx_fee_types_category")
                    .table(FeeTypes::Table)
                    .col(FeeTypes::FeeCategory),
            )
            .index(
                Index::create()
                    .name("idx_fee_types_active")
                    .table(FeeTypes::Table)
                    .col(FeeTypes::IsActive),
            )
            .to_owned();

        manager.create_table(fee_types).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FeeTypes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum FeeTypes {
    Table,
    Id,
    InstitutionId,
    FeeName,
    FeeCode,
    FeeCategory,
    FeeCalculationMethod,
    FeeValue,
    MinimumFee,
    MaximumFee,
    Currency,
    IsVatApplicable,
    VatRate,
    AppliesToTransactionTypes,
    AppliesToAccounts,
    AppliesToCustomers,
    IsWaiverAllowed,
    IsWaiverApprovalRequired,
    IsActive,
    EffectiveFrom,
    EffectiveTo,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
