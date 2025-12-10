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
                "CREATE TYPE statement_type AS ENUM ('BALANCE_SHEET', 'PROFIT_LOSS', 'CASH_FLOW')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE statement_status AS ENUM ('DRAFT', 'FINAL', 'AUDITED')".to_string(),
            ))
            .await?;

        let financial_statements = Table::create()
            .table(FinancialStatements::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(FinancialStatements::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(FinancialStatements::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(FinancialStatements::StatementType)
                    .custom("statement_type")
                    .not_null(),
            )
            .col(
                ColumnDef::new(FinancialStatements::StatementPeriod)
                    .date()
                    .not_null(),
            )
            .col(
                ColumnDef::new(FinancialStatements::StatementData)
                    .json_binary()
                    .not_null(),
            )
            .col(
                ColumnDef::new(FinancialStatements::StatementFormat)
                    .string()
                    .default("JSON"),
            )
            .col(
                ColumnDef::new(FinancialStatements::Status)
                    .custom("statement_status")
                    .default("DRAFT"),
            )
            .col(ColumnDef::new(FinancialStatements::PreparedBy).big_integer())
            .col(ColumnDef::new(FinancialStatements::ReviewedBy).big_integer())
            .col(ColumnDef::new(FinancialStatements::ApprovedBy).big_integer())
            .col(ColumnDef::new(FinancialStatements::ApprovedAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(FinancialStatements::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(FinancialStatements::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        FinancialStatements::Table,
                        FinancialStatements::InstitutionId,
                    )
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(FinancialStatements::Table, FinancialStatements::PreparedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(FinancialStatements::Table, FinancialStatements::ReviewedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(FinancialStatements::Table, FinancialStatements::ApprovedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .index(
                Index::create()
                    .name("idx_financial_statements")
                    .table(FinancialStatements::Table)
                    .col(FinancialStatements::InstitutionId)
                    .col(FinancialStatements::StatementType)
                    .col(FinancialStatements::StatementPeriod)
                    .unique(),
            )
            .index(
                Index::create()
                    .name("idx_financial_statements_type")
                    .table(FinancialStatements::Table)
                    .col(FinancialStatements::StatementType),
            )
            .index(
                Index::create()
                    .name("idx_financial_statements_status")
                    .table(FinancialStatements::Table)
                    .col(FinancialStatements::Status),
            )
            .to_owned();

        manager.create_table(financial_statements).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FinancialStatements::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum FinancialStatements {
    Table,
    Id,
    InstitutionId,
    StatementType,
    StatementPeriod,
    StatementData,
    StatementFormat,
    Status,
    PreparedBy,
    ReviewedBy,
    ApprovedBy,
    ApprovedAt,
    CreatedAt,
    UpdatedAt,
}
