use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::m20251208_101414_create_teller_cash_drawers::TellerCashDrawers;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE teller_recon_type AS ENUM ('DAILY', 'FORCE_CLOSE', 'EMERGENCY')"
                    .to_string(),
            ))
            .await?;

        let teller_recon = Table::create()
            .table(TellerReconciliations::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(TellerReconciliations::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(TellerReconciliations::CashDrawerId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(TellerReconciliations::ReconciliationType)
                    .custom("teller_recon_type"),
            )
            .col(ColumnDef::new(TellerReconciliations::Notes).string())
            .col(
                ColumnDef::new(TellerReconciliations::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(TellerReconciliations::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(
                        TellerReconciliations::Table,
                        TellerReconciliations::CashDrawerId,
                    )
                    .to(TellerCashDrawers::Table, TellerCashDrawers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(teller_recon).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TellerReconciliations::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum TellerReconciliations {
    Table,
    Id,
    CashDrawerId,
    SupervisorId,
    ReconciliationType,
    Notes,
    CreatedAt,
    UpdatedAt,
}
