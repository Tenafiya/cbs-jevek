use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
    m20251210_190018_create_gl_postings::GlPostings,
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
                "CREATE TYPE gl_reversals_type AS ENUM ('ERROR', 'FRAUD', 'CUSTOMER_REQUEST')"
                    .to_string(),
            ))
            .await?;

        let gl_reversals = Table::create()
            .table(GlReversals::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(GlReversals::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(GlReversals::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(GlReversals::OriginalPostingId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(GlReversals::ReversalPostingId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(GlReversals::ReversalReason)
                    .text()
                    .not_null(),
            )
            .col(
                ColumnDef::new(GlReversals::ReversalType)
                    .custom("gl_reversals_type")
                    .not_null(),
            )
            .col(ColumnDef::new(GlReversals::ApprovedBy).big_integer())
            .col(ColumnDef::new(GlReversals::ApprovedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(GlReversals::ImpactAssessment).json_binary())
            .col(
                ColumnDef::new(GlReversals::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(GlReversals::Table, GlReversals::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(GlReversals::Table, GlReversals::OriginalPostingId)
                    .to(GlPostings::Table, GlPostings::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(GlReversals::Table, GlReversals::ReversalPostingId)
                    .to(GlPostings::Table, GlPostings::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(GlReversals::Table, GlReversals::ApprovedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .index(
                Index::create()
                    .name("idx_gl_reversals_original")
                    .table(GlReversals::Table)
                    .col(GlReversals::OriginalPostingId),
            )
            .index(
                Index::create()
                    .name("idx_gl_reversals_reversal")
                    .table(GlReversals::Table)
                    .col(GlReversals::ReversalPostingId),
            )
            .index(
                Index::create()
                    .name("idx_gl_reversals_type")
                    .table(GlReversals::Table)
                    .col(GlReversals::ReversalType),
            )
            .to_owned();

        manager.create_table(gl_reversals).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GlReversals::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum GlReversals {
    Table,
    Id,
    InstitutionId,
    OriginalPostingId,
    ReversalPostingId,
    ReversalReason,
    ReversalType,
    ApprovedBy,
    ApprovedAt,
    ImpactAssessment,
    CreatedAt,
}
