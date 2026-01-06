use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions,
    m20251204_150208_create_branches::{Branches, Staff},
    m20251205_154503_create_accounts::Accounts,
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
                "CREATE TYPE agent_entity_type AS ENUM ('INDIVIDUAL', 'BUSINESS', 'SACCO', 'GROUP')"
                    .to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE agent_status AS ENUM ('PENDING', 'ACTIVE', 'SUSPENDED', 'TERMINATED')"
                    .to_string(),
            ))
            .await?;

        let agents = Table::create()
            .table(Agents::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Agents::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(Agents::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(Agents::BranchId).big_integer().not_null())
            .col(
                ColumnDef::new(Agents::AgentNumber)
                    .string()
                    .not_null()
                    .unique_key(),
            )
            .col(ColumnDef::new(Agents::AgentName).string())
            .col(ColumnDef::new(Agents::BusinessName).string())
            .col(ColumnDef::new(Agents::EntityType).custom("agent_entity_type"))
            .col(ColumnDef::new(Agents::BusinessRegistrationNumber).string())
            .col(ColumnDef::new(Agents::PhoneCountryCode).string())
            .col(ColumnDef::new(Agents::PhoneNumber).string())
            .col(ColumnDef::new(Agents::Email).string())
            .col(ColumnDef::new(Agents::OperatingAddress).json_binary())
            .col(ColumnDef::new(Agents::BusinessAddress).json_binary())
            .col(ColumnDef::new(Agents::KycCompletedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(Agents::SettlementAccountId).big_integer())
            .col(ColumnDef::new(Agents::CommissionAccountId).big_integer())
            .col(
                ColumnDef::new(Agents::TransactionVolumeToday)
                    .decimal_len(20, 4)
                    .default(0.00),
            )
            .col(
                ColumnDef::new(Agents::TransactionCountToday)
                    .integer()
                    .default(0),
            )
            .col(ColumnDef::new(Agents::LastTransactionAt).timestamp_with_time_zone())
            .col(ColumnDef::new(Agents::Status).custom("agent_status"))
            .col(ColumnDef::new(Agents::SuspensionReason).string())
            .col(ColumnDef::new(Agents::TerminatedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(Agents::GeofenceRadius).json_binary())
            .col(ColumnDef::new(Agents::CustomFields).json_binary())
            .col(ColumnDef::new(Agents::KycStatus).json_binary())
            .col(ColumnDef::new(Agents::VerifiedBy).big_integer())
            .col(
                ColumnDef::new(Agents::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Agents::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Agents::Table, Agents::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Agents::Table, Agents::BranchId)
                    .to(Branches::Table, Branches::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Agents::Table, Agents::SettlementAccountId)
                    .to(Accounts::Table, Accounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Agents::Table, Agents::CommissionAccountId)
                    .to(Accounts::Table, Accounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Agents::Table, Agents::VerifiedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(agents).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Agents::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Agents {
    Table,
    Id,
    InstitutionId,
    BranchId,
    AgentNumber,
    AgentName,
    BusinessName,
    EntityType,
    BusinessRegistrationNumber,
    PhoneCountryCode,
    PhoneNumber,
    Email,
    OperatingAddress,
    BusinessAddress,
    KycStatus,
    KycCompletedAt,
    VerifiedBy,
    SettlementAccountId,
    CommissionAccountId,
    TransactionVolumeToday,
    TransactionCountToday,
    LastTransactionAt,
    Status,
    SuspensionReason,
    TerminatedAt,
    GeofenceRadius,
    CustomFields,
    CreatedAt,
    UpdatedAt,
}
