use sea_orm_migration::prelude::*;

use crate::{m20251204_150208_create_branches::Staff, m20251208_093551_create_tellers::Tellers};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE TYPE teller_cash_drawers_status AS ENUM ('OPEN', 'BALANCED', 'VARIANCE', 'FORCE_CLOSED')
                "#,
            )
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
            .col(ColumnDef::new(TellerCashDrawers::OpeningBalance).big_integer())
            .col(ColumnDef::new(TellerCashDrawers::OpeningCash).json_binary())
            .col(
                ColumnDef::new(TellerCashDrawers::TotalCashIn)
                    .big_integer()
                    .default(0),
            )
            .col(
                ColumnDef::new(TellerCashDrawers::TotalCashOut)
                    .big_integer()
                    .default(0),
            )
            .col(
                ColumnDef::new(TellerCashDrawers::TotalCheques)
                    .big_integer()
                    .default(0),
            )
            .col(
                ColumnDef::new(TellerCashDrawers::TotalTransfersIn)
                    .big_integer()
                    .default(0),
            )
            .col(
                ColumnDef::new(TellerCashDrawers::TotalTransfersOut)
                    .big_integer()
                    .default(0),
            )
            .col(ColumnDef::new(TellerCashDrawers::ClosingBalance).big_integer())
            .col(ColumnDef::new(TellerCashDrawers::ClosingCash).json_binary())
            .col(ColumnDef::new(TellerCashDrawers::ExpectedAmount).big_integer())
            .col(
                ColumnDef::new(TellerCashDrawers::VarianceAmount)
                    .big_integer()
                    .default(0),
            )
            .col(ColumnDef::new(TellerCashDrawers::VarianceReason).string())
            .col(ColumnDef::new(TellerCashDrawers::Status).custom("teller_cash_drawers_status"))
            .col(
                ColumnDef::new(TellerCashDrawers::OpenedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(ColumnDef::new(TellerCashDrawers::ClosedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(TellerCashDrawers::ClosedBySupervisor).big_integer())
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
            .foreign_key(
                ForeignKey::create()
                    .from(
                        TellerCashDrawers::Table,
                        TellerCashDrawers::ClosedBySupervisor,
                    )
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(cash_drawers).await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    ALTER TABLE teller_cash_drawers
                    ADD CONSTRAINT unique_tel_cash_teller_opened
                    UNIQUE (teller_id, opened_at);
                "#,
            )
            .await?;

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
