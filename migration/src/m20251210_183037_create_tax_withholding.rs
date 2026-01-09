use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions,
    m20251204_152312_create_customers::Customers,
    m20251205_193221_create_transactions::Transactions,
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
                "CREATE TYPE tax_withholding_type AS ENUM ('VAT', 'WITHHOLDING_TAX')".to_string(),
            ))
            .await?;

        let tax_withholding = Table::create()
            .table(TaxWithholding::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(TaxWithholding::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(TaxWithholding::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(TaxWithholding::TransactionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(TaxWithholding::CustomerId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(TaxWithholding::TaxType)
                    .custom("tax_withholding_type")
                    .not_null(),
            )
            .col(
                ColumnDef::new(TaxWithholding::TaxRate)
                    .decimal_len(10, 6)
                    .not_null(),
            )
            .col(
                ColumnDef::new(TaxWithholding::TaxableAmount)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(TaxWithholding::TaxAmount)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(ColumnDef::new(TaxWithholding::TaxInvoiceNumber).string())
            .col(ColumnDef::new(TaxWithholding::TaxPeriod).date().not_null())
            .col(
                ColumnDef::new(TaxWithholding::IsRemittedToTaxAuthority)
                    .boolean()
                    .default(false),
            )
            .col(ColumnDef::new(TaxWithholding::RemittanceReference).string())
            .col(ColumnDef::new(TaxWithholding::RemittedAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(TaxWithholding::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(TaxWithholding::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(TaxWithholding::Table, TaxWithholding::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(TaxWithholding::Table, TaxWithholding::TransactionId)
                    .to(Transactions::Table, Transactions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(TaxWithholding::Table, TaxWithholding::CustomerId)
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(tax_withholding).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TaxWithholding::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum TaxWithholding {
    Table,
    Id,
    InstitutionId,
    TransactionId,
    CustomerId,
    TaxType,
    TaxRate,
    TaxableAmount,
    TaxAmount,
    TaxInvoiceNumber,
    TaxPeriod,
    IsRemittedToTaxAuthority,
    RemittanceReference,
    RemittedAt,
    CreatedAt,
    UpdatedAt,
}
