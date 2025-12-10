use sea_orm_migration::prelude::*;

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let product_visibility_rules = Table::create()
            .table(ProductVisibilityRules::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(ProductVisibilityRules::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(ProductVisibilityRules::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(ProductVisibilityRules::ProductType)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(ProductVisibilityRules::ProductId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(ProductVisibilityRules::VisibleToBranches).json_binary())
            .col(ColumnDef::new(ProductVisibilityRules::VisibleToCustomers).json_binary())
            .col(ColumnDef::new(ProductVisibilityRules::VisibleToKycTiers).json_binary())
            .col(ColumnDef::new(ProductVisibilityRules::MinCustomerAge).integer())
            .col(ColumnDef::new(ProductVisibilityRules::MaxCustomerAge).integer())
            .col(ColumnDef::new(ProductVisibilityRules::MinCreditScore).integer())
            .col(
                ColumnDef::new(ProductVisibilityRules::IsActive)
                    .boolean()
                    .default(true),
            )
            .col(ColumnDef::new(ProductVisibilityRules::EffectiveFrom).timestamp_with_time_zone())
            .col(ColumnDef::new(ProductVisibilityRules::EffectiveTo).timestamp_with_time_zone())
            .col(ColumnDef::new(ProductVisibilityRules::CreatedBy).big_integer())
            .col(
                ColumnDef::new(ProductVisibilityRules::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(ProductVisibilityRules::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        ProductVisibilityRules::Table,
                        ProductVisibilityRules::InstitutionId,
                    )
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        ProductVisibilityRules::Table,
                        ProductVisibilityRules::CreatedBy,
                    )
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .index(
                Index::create()
                    .name("idx_product_visibility_rules_institution")
                    .table(ProductVisibilityRules::Table)
                    .col(ProductVisibilityRules::InstitutionId),
            )
            .index(
                Index::create()
                    .name("idx_product_visibility_rules_product")
                    .table(ProductVisibilityRules::Table)
                    .col(ProductVisibilityRules::ProductType)
                    .col(ProductVisibilityRules::ProductId),
            )
            .index(
                Index::create()
                    .name("idx_product_visibility_rules_active")
                    .table(ProductVisibilityRules::Table)
                    .col(ProductVisibilityRules::IsActive),
            )
            .to_owned();

        manager.create_table(product_visibility_rules).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ProductVisibilityRules::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum ProductVisibilityRules {
    Table,
    Id,
    InstitutionId,
    ProductType,
    ProductId,
    VisibleToBranches,
    VisibleToCustomers,
    VisibleToKycTiers,
    MinCustomerAge,
    MaxCustomerAge,
    MinCreditScore,
    IsActive,
    EffectiveFrom,
    EffectiveTo,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
