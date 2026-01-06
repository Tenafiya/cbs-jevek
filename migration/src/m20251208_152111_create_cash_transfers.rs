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
                "CREATE TYPE cash_transfers_type AS ENUM ('VAULT', 'TELLER_DRAWER')".to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE cash_transfers_status AS ENUM ('PENDING', 'COMPLETED', 'CANCELLED')"
                    .to_string(),
            ))
            .await?;

        let cash_trans = Table::create()
            .table(CashTransfers::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CashTransfers::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(CashTransfers::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(CashTransfers::SourceType).custom("cash_transfers_type"))
            .col(ColumnDef::new(CashTransfers::SourceId).big_integer())
            .col(ColumnDef::new(CashTransfers::DestinationType).custom("cash_transfers_type"))
            .col(ColumnDef::new(CashTransfers::DestinationId).big_integer())
            .col(
                ColumnDef::new(CashTransfers::Amount)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(ColumnDef::new(CashTransfers::CashBreakdown).json_binary())
            .col(ColumnDef::new(CashTransfers::Currency).json_binary())
            .col(
                ColumnDef::new(CashTransfers::SourceBalanceBefore)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(
                ColumnDef::new(CashTransfers::DestinationBalanceBefore)
                    .decimal_len(20, 4)
                    .not_null(),
            )
            .col(ColumnDef::new(CashTransfers::Status).custom("cash_transfers_status"))
            .col(ColumnDef::new(CashTransfers::ApprovedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(CashTransfers::RequestedBy).big_integer())
            .col(ColumnDef::new(CashTransfers::ApprovedBy).big_integer())
            .col(
                ColumnDef::new(CashTransfers::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(CashTransfers::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(CashTransfers::Table, CashTransfers::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(CashTransfers::Table, CashTransfers::RequestedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(CashTransfers::Table, CashTransfers::ApprovedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(cash_trans).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CashTransfers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum CashTransfers {
    Table,
    Id,
    InstitutionId,
    SourceType,
    SourceId,
    DestinationType,
    DestinationId,
    Amount,
    Currency,
    CashBreakdown,
    SourceBalanceBefore,
    DestinationBalanceBefore,
    Status,
    RequestedBy,
    ApprovedBy,
    ApprovedAt,
    CreatedAt,
    UpdatedAt,
}
