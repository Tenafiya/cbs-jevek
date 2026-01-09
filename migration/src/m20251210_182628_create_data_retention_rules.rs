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
                "CREATE TYPE data_retention_category AS ENUM ('TRANSACTIONS', 'AUDIT_LOGS', 'CUSTOMER_DATA', 'DOCUMENTS')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE data_retention_action_after_retention AS ENUM ('ARCHIVE', 'DELETE', 'ANONYMIZE')"
                    .to_string(),
            ))
            .await?;

        let data_retention_rules = Table::create()
            .table(DataRetentionRules::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(DataRetentionRules::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(DataRetentionRules::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(DataRetentionRules::RuleName)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(DataRetentionRules::DataCategory)
                    .custom("data_retention_category")
                    .not_null(),
            )
            .col(
                ColumnDef::new(DataRetentionRules::RetentionPeriodDays)
                    .integer()
                    .not_null(),
            )
            .col(ColumnDef::new(DataRetentionRules::RetentionPeriodMonths).integer())
            .col(ColumnDef::new(DataRetentionRules::RetentionPeriodYears).integer())
            .col(
                ColumnDef::new(DataRetentionRules::ActionAfterRetention)
                    .custom("data_retention_action_after_retention")
                    .default("ARCHIVE"),
            )
            .col(
                ColumnDef::new(DataRetentionRules::IsActive)
                    .boolean()
                    .default(true),
            )
            .col(ColumnDef::new(DataRetentionRules::LastAppliedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(DataRetentionRules::CreatedBy).big_integer())
            .col(
                ColumnDef::new(DataRetentionRules::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(DataRetentionRules::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(DataRetentionRules::Table, DataRetentionRules::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(DataRetentionRules::Table, DataRetentionRules::CreatedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .to_owned();

        manager.create_table(data_retention_rules).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DataRetentionRules::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum DataRetentionRules {
    Table,
    Id,
    InstitutionId,
    RuleName,
    DataCategory,
    RetentionPeriodDays,
    RetentionPeriodMonths,
    RetentionPeriodYears,
    ActionAfterRetention,
    IsActive,
    LastAppliedAt,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
