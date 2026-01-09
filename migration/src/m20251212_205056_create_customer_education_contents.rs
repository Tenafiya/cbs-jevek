use sea_orm_migration::prelude::*;

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let customer_education_content = Table::create()
            .table(CustomerEducationContent::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CustomerEducationContent::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(CustomerEducationContent::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CustomerEducationContent::ContentTitle)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(CustomerEducationContent::ContentType)
                    .string()
                    .not_null(),
            )
            .col(ColumnDef::new(CustomerEducationContent::ContentCategory).string())
            .col(ColumnDef::new(CustomerEducationContent::ContentBody).text())
            .col(ColumnDef::new(CustomerEducationContent::ContentUrl).text())
            .col(ColumnDef::new(CustomerEducationContent::ContentDurationMinutes).integer())
            .col(ColumnDef::new(CustomerEducationContent::ThumbnailUrl).text())
            .col(ColumnDef::new(CustomerEducationContent::Tags).json_binary())
            .col(ColumnDef::new(CustomerEducationContent::DifficultyLevel).string())
            .col(
                ColumnDef::new(CustomerEducationContent::IsPublished)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(CustomerEducationContent::PublishedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(CustomerEducationContent::PublishedBy).big_integer())
            .col(
                ColumnDef::new(CustomerEducationContent::ViewCount)
                    .integer()
                    .default(0),
            )
            .col(ColumnDef::new(CustomerEducationContent::CompletionRate).decimal_len(5, 2))
            .col(
                ColumnDef::new(CustomerEducationContent::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(CustomerEducationContent::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        CustomerEducationContent::Table,
                        CustomerEducationContent::InstitutionId,
                    )
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        CustomerEducationContent::Table,
                        CustomerEducationContent::PublishedBy,
                    )
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .to_owned();

        manager.create_table(customer_education_content).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(CustomerEducationContent::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum CustomerEducationContent {
    Table,
    Id,
    InstitutionId,
    ContentTitle,
    ContentType,
    ContentCategory,
    ContentBody,
    ContentUrl,
    ContentDurationMinutes,
    ThumbnailUrl,
    Tags,
    DifficultyLevel,
    IsPublished,
    PublishedAt,
    PublishedBy,
    ViewCount,
    CompletionRate,
    CreatedAt,
    UpdatedAt,
}
