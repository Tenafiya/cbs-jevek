use sea_orm_migration::prelude::*;
use sea_orm::Statement;

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
    m20251204_151411_create_chart_of_accounts::ChartOfAccounts,
    m20251205_193221_create_transactions::Transactions,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let gl_postings = Table::create()
            .table(GlPostings::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(GlPostings::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(GlPostings::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(GlPostings::TransactionId).big_integer())
            .col(ColumnDef::new(GlPostings::ReferenceNumber).string())
            .col(ColumnDef::new(GlPostings::ValueDate).date().not_null())
            .col(
                ColumnDef::new(GlPostings::PostingDate)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(GlPostings::DebitAccountId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(GlPostings::DebitAmount)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(GlPostings::CreditAccountId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(GlPostings::CreditAmount)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(GlPostings::Narration).text().not_null())
            .col(
                ColumnDef::new(GlPostings::IsReversed)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(GlPostings::ReversalPostingId).big_integer())
            .col(ColumnDef::new(GlPostings::PostedBy).big_integer())
            .foreign_key(
                ForeignKey::create()
                    .from(GlPostings::Table, GlPostings::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(GlPostings::Table, GlPostings::TransactionId)
                    .to(Transactions::Table, Transactions::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(GlPostings::Table, GlPostings::DebitAccountId)
                    .to(ChartOfAccounts::Table, ChartOfAccounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(GlPostings::Table, GlPostings::CreditAccountId)
                    .to(ChartOfAccounts::Table, ChartOfAccounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(GlPostings::Table, GlPostings::ReversalPostingId)
                    .to(GlPostings::Table, GlPostings::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(GlPostings::Table, GlPostings::PostedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(gl_postings).await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                    CREATE UNIQUE INDEX unique_transaction_id_not_null
                    ON gl_postings (transaction_id)
                    WHERE transaction_id IS NOT NULL;
                "#
                .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                CREATE INDEX idx_gl_postings_debit_account ON gl_postings(debit_account_id, value_date);
            "#
                .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                CREATE INDEX idx_gl_postings_credit_account ON gl_postings(credit_account_id, value_date);
            "#
                .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                CREATE INDEX idx_gl_postings_transaction ON gl_postings(transaction_id);
            "#
                .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
                CREATE INDEX idx_gl_postings_date ON gl_postings(value_date);
            "#
                .to_string(),
            ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GlPostings::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum GlPostings {
    Table,
    Id,
    InstitutionId,
    TransactionId,
    ReferenceNumber,
    ValueDate,
    PostingDate,
    DebitAccountId,
    DebitAmount,
    CreditAccountId,
    CreditAmount,
    Narration,
    IsReversed,
    ReversalPostingId,
    PostedBy,
}
