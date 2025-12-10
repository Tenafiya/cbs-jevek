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
                "CREATE TYPE ledger_lock_type AS ENUM ('MONTH_END', 'YEAR_END', 'AUDIT')"
                    .to_string(),
            ))
            .await?;

        let ledger_lock_periods = Table::create()
            .table(LedgerLockPeriods::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(LedgerLockPeriods::Id)
                    .big_integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(
                ColumnDef::new(LedgerLockPeriods::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(LedgerLockPeriods::StartDate)
                    .date()
                    .not_null(),
            )
            .col(ColumnDef::new(LedgerLockPeriods::EndDate).date().not_null())
            .col(
                ColumnDef::new(LedgerLockPeriods::LockType)
                    .custom("ledger_lock_type")
                    .not_null(),
            )
            .col(
                ColumnDef::new(LedgerLockPeriods::IsLocked)
                    .boolean()
                    .default(true),
            )
            .col(ColumnDef::new(LedgerLockPeriods::LockedBy).big_integer())
            .col(ColumnDef::new(LedgerLockPeriods::LockedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(LedgerLockPeriods::UnlockedBy).big_integer())
            .col(ColumnDef::new(LedgerLockPeriods::UnlockedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(LedgerLockPeriods::UnlockReason).text())
            .col(
                ColumnDef::new(LedgerLockPeriods::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(LedgerLockPeriods::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LedgerLockPeriods::Table, LedgerLockPeriods::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LedgerLockPeriods::Table, LedgerLockPeriods::LockedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(LedgerLockPeriods::Table, LedgerLockPeriods::UnlockedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .index(
                Index::create()
                    .name("uk_ledger_lock_periods")
                    .table(LedgerLockPeriods::Table)
                    .col(LedgerLockPeriods::InstitutionId)
                    .col(LedgerLockPeriods::StartDate)
                    .col(LedgerLockPeriods::EndDate)
                    .unique(),
            )
            .index(
                Index::create()
                    .name("idx_ledger_locks_type")
                    .table(LedgerLockPeriods::Table)
                    .col(LedgerLockPeriods::LockType),
            )
            .index(
                Index::create()
                    .name("idx_ledger_locks_status")
                    .table(LedgerLockPeriods::Table)
                    .col(LedgerLockPeriods::IsLocked),
            )
            .to_owned();

        manager.create_table(ledger_lock_periods).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LedgerLockPeriods::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum LedgerLockPeriods {
    Table,
    Id,
    InstitutionId,
    StartDate,
    EndDate,
    LockType,
    IsLocked,
    LockedBy,
    LockedAt,
    UnlockedBy,
    UnlockedAt,
    UnlockReason,
    CreatedAt,
    UpdatedAt,
}
