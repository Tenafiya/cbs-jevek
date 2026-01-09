use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
    m20251210_172518_create_fee_applications::FeeApplications,
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
                "CREATE TYPE fee_waiver_workflows_approval AS ENUM ('PENDING', 'APPROVED', 'REJECTED')"
                    .to_string(),
            ))
            .await?;

        let fee_waiver_workflows = Table::create()
            .table(FeeWaiverWorkflows::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(FeeWaiverWorkflows::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(FeeWaiverWorkflows::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(FeeWaiverWorkflows::FeeApplicationId).big_integer())
            .col(
                ColumnDef::new(FeeWaiverWorkflows::RequestedBy)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(FeeWaiverWorkflows::RequestedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(FeeWaiverWorkflows::WaiverReason)
                    .string()
                    .not_null(),
            )
            .col(ColumnDef::new(FeeWaiverWorkflows::ApprovedBy).big_integer())
            .col(
                ColumnDef::new(FeeWaiverWorkflows::ApprovalStatus)
                    .custom("fee_waiver_workflows_approval")
                    .default("PENDING"),
            )
            .col(ColumnDef::new(FeeWaiverWorkflows::ApprovalNotes).string())
            .col(ColumnDef::new(FeeWaiverWorkflows::ApprovedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(FeeWaiverWorkflows::RequestedWaiverAmount).decimal_len(20, 4))
            .col(ColumnDef::new(FeeWaiverWorkflows::ApprovedWaiverAmount).decimal_len(20, 4))
            .col(
                ColumnDef::new(FeeWaiverWorkflows::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(FeeWaiverWorkflows::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(FeeWaiverWorkflows::Table, FeeWaiverWorkflows::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        FeeWaiverWorkflows::Table,
                        FeeWaiverWorkflows::FeeApplicationId,
                    )
                    .to(FeeApplications::Table, FeeApplications::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(FeeWaiverWorkflows::Table, FeeWaiverWorkflows::RequestedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(FeeWaiverWorkflows::Table, FeeWaiverWorkflows::ApprovedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(fee_waiver_workflows).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FeeWaiverWorkflows::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum FeeWaiverWorkflows {
    Table,
    Id,
    InstitutionId,
    FeeApplicationId,
    RequestedBy,
    RequestedAt,
    WaiverReason,
    ApprovedBy,
    ApprovalStatus,
    ApprovalNotes,
    ApprovedAt,
    RequestedWaiverAmount,
    ApprovedWaiverAmount,
    CreatedAt,
    UpdatedAt,
}
