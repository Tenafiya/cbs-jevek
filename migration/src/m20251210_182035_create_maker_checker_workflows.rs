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
                "CREATE TYPE maker_checker_reference_type AS ENUM ('TRANSACTION', 'LOAN_APPROVAL', 'PRODUCT_CHANGE')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE maker_checker_status AS ENUM ('PENDING', 'APPROVED', 'REJECTED', 'EXPIRED')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE maker_checker_checker_action AS ENUM ('APPROVED', 'REJECTED')"
                    .to_string(),
            ))
            .await?;

        let maker_checker_workflows = Table::create()
            .table(MakerCheckerWorkflows::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(MakerCheckerWorkflows::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(MakerCheckerWorkflows::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(MakerCheckerWorkflows::ReferenceType)
                    .custom("maker_checker_reference_type")
                    .not_null(),
            )
            .col(
                ColumnDef::new(MakerCheckerWorkflows::ReferenceId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(MakerCheckerWorkflows::MakerId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(MakerCheckerWorkflows::MakerAction)
                    .string()
                    .not_null(),
            )
            .col(ColumnDef::new(MakerCheckerWorkflows::MakerNotes).string())
            .col(
                ColumnDef::new(MakerCheckerWorkflows::RequestData)
                    .json_binary()
                    .not_null(),
            )
            .col(ColumnDef::new(MakerCheckerWorkflows::CheckerId).big_integer())
            .col(
                ColumnDef::new(MakerCheckerWorkflows::CheckerAction)
                    .custom("maker_checker_checker_action"),
            )
            .col(ColumnDef::new(MakerCheckerWorkflows::CheckerNotes).string())
            .col(ColumnDef::new(MakerCheckerWorkflows::CheckedAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(MakerCheckerWorkflows::Status)
                    .custom("maker_checker_status")
                    .default("PENDING"),
            )
            .col(
                ColumnDef::new(MakerCheckerWorkflows::RequestedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(ColumnDef::new(MakerCheckerWorkflows::ExpiresAt).timestamp_with_time_zone())
            .col(ColumnDef::new(MakerCheckerWorkflows::ImplementedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(MakerCheckerWorkflows::ImplementationResult).json_binary())
            .foreign_key(
                ForeignKey::create()
                    .from(
                        MakerCheckerWorkflows::Table,
                        MakerCheckerWorkflows::InstitutionId,
                    )
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(MakerCheckerWorkflows::Table, MakerCheckerWorkflows::MakerId)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        MakerCheckerWorkflows::Table,
                        MakerCheckerWorkflows::CheckerId,
                    )
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .index(
                Index::create()
                    .name("idx_maker_checker_status")
                    .table(MakerCheckerWorkflows::Table)
                    .col(MakerCheckerWorkflows::Status),
            )
            .index(
                Index::create()
                    .name("idx_maker_checker_maker")
                    .table(MakerCheckerWorkflows::Table)
                    .col(MakerCheckerWorkflows::MakerId),
            )
            .index(
                Index::create()
                    .name("idx_maker_checker_checker")
                    .table(MakerCheckerWorkflows::Table)
                    .col(MakerCheckerWorkflows::CheckerId),
            )
            .index(
                Index::create()
                    .name("idx_maker_checker_expires")
                    .table(MakerCheckerWorkflows::Table)
                    .col(MakerCheckerWorkflows::ExpiresAt),
            )
            .to_owned();

        manager.create_table(maker_checker_workflows).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MakerCheckerWorkflows::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum MakerCheckerWorkflows {
    Table,
    Id,
    InstitutionId,
    ReferenceType,
    ReferenceId,
    MakerId,
    MakerAction,
    MakerNotes,
    RequestData,
    CheckerId,
    CheckerAction,
    CheckerNotes,
    CheckedAt,
    Status,
    RequestedAt,
    ExpiresAt,
    ImplementedAt,
    ImplementationResult,
}
