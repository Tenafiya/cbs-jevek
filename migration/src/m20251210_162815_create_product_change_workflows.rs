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
                "CREATE TYPE product_workflows_product_type AS ENUM ('LOAN_PRODUCT', 'SAVINGS_PRODUCT', 'ACCOUNT_TYPE')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE product_workflows_status AS ENUM ('PENDING', 'APPROVED', 'REJECTED')"
                    .to_string(),
            ))
            .await?;

        let product_change_workflows = Table::create()
            .table(ProductChangeWorkflows::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(ProductChangeWorkflows::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(ProductChangeWorkflows::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(ProductChangeWorkflows::ProductType)
                    .custom("product_workflows_product_type")
                    .not_null(),
            )
            .col(
                ColumnDef::new(ProductChangeWorkflows::ProductId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(ProductChangeWorkflows::ChangeDescription)
                    .text()
                    .not_null(),
            )
            .col(ColumnDef::new(ProductChangeWorkflows::OldConfig).json_binary())
            .col(ColumnDef::new(ProductChangeWorkflows::NewConfig).json_binary())
            .col(
                ColumnDef::new(ProductChangeWorkflows::RequesterId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(ProductChangeWorkflows::ApproverId).big_integer())
            .col(
                ColumnDef::new(ProductChangeWorkflows::Status)
                    .custom("product_workflows_status")
                    .default("PENDING"),
            )
            .col(ColumnDef::new(ProductChangeWorkflows::RequestReason).text())
            .col(ColumnDef::new(ProductChangeWorkflows::RejectionReason).text())
            .col(
                ColumnDef::new(ProductChangeWorkflows::RequestedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(ColumnDef::new(ProductChangeWorkflows::ApprovedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(ProductChangeWorkflows::ImplementedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(ProductChangeWorkflows::ImplementedBy).big_integer())
            .col(
                ColumnDef::new(ProductChangeWorkflows::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(ProductChangeWorkflows::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        ProductChangeWorkflows::Table,
                        ProductChangeWorkflows::InstitutionId,
                    )
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        ProductChangeWorkflows::Table,
                        ProductChangeWorkflows::RequesterId,
                    )
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        ProductChangeWorkflows::Table,
                        ProductChangeWorkflows::ApproverId,
                    )
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        ProductChangeWorkflows::Table,
                        ProductChangeWorkflows::ImplementedBy,
                    )
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .to_owned();

        manager.create_table(product_change_workflows).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ProductChangeWorkflows::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum ProductChangeWorkflows {
    Table,
    Id,
    InstitutionId,
    ProductType,
    ProductId,
    ChangeDescription,
    OldConfig,
    NewConfig,
    RequesterId,
    ApproverId,
    Status,
    RequestReason,
    RejectionReason,
    RequestedAt,
    ApprovedAt,
    ImplementedAt,
    ImplementedBy,
    CreatedAt,
    UpdatedAt,
}
