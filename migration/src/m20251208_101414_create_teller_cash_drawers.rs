use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::m20251208_093551_create_tellers::Tellers;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE teller_cash_drawers_status AS ENUM ('OPEN', 'BALANCED', 'VARIANCE', 'FORCE_CLOSED')".to_string(),
            ))
            .await?;

        let cash_drawers = Table::create()
            .table(TellerCashDrawers::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(TellerCashDrawers::Id)
                    .big_integer()
                    .primary_key()
                    .not_null(),
            )
            .col(
                ColumnDef::new(TellerCashDrawers::TellerId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(TellerCashDrawers::OpeningBalance).decimal_len(20, 4))
            .col(ColumnDef::new(TellerCashDrawers::OpeningCash).json_binary())
            .col(
                ColumnDef::new(TellerCashDrawers::TotalCashIn)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(TellerCashDrawers::TotalCashOut)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(TellerCashDrawers::TotalCheques)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(TellerCashDrawers::TotalTransfersIn)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(TellerCashDrawers::TotalTransfersOut)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(ColumnDef::new(TellerCashDrawers::ClosingBalance).decimal_len(20, 4))
            .col(ColumnDef::new(TellerCashDrawers::ClosingCash).json_binary())
            .col(ColumnDef::new(TellerCashDrawers::ExpectedAmount).decimal_len(20, 4))
            .col(
                ColumnDef::new(TellerCashDrawers::VarianceAmount)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(ColumnDef::new(TellerCashDrawers::VarianceReason).string())
            .col(ColumnDef::new(TellerCashDrawers::Status).custom("teller_cash_drawers_status"))
            .col(
                ColumnDef::new(TellerCashDrawers::OpenedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(ColumnDef::new(TellerCashDrawers::ClosedAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(TellerCashDrawers::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(TellerCashDrawers::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(TellerCashDrawers::Table, TellerCashDrawers::TellerId)
                    .to(Tellers::Table, Tellers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(cash_drawers).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TellerCashDrawers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum TellerCashDrawers {
    Table,
    Id,
    TellerId,
    OpeningBalance,
    OpeningCash,
    TotalCashIn,
    TotalCashOut,
    TotalCheques,
    TotalTransfersIn,
    TotalTransfersOut,
    ClosingBalance,
    ClosingCash,
    ExpectedAmount,
    VarianceAmount,
    VarianceReason,
    Status,
    ClosedBySupervisor,
    OpenedAt,
    ClosedAt,
    CreatedAt,
    UpdatedAt,
}
