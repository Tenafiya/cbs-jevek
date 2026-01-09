use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::{
    m20251204_112805_create_institutions::Institutions, m20251204_150208_create_branches::Staff,
    m20251204_152312_create_customers::Customers, m20251205_154503_create_accounts::Accounts,
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
                "CREATE TYPE aml_risk_level_enum AS ENUM ('LOW', 'MEDIUM', 'HIGH')".to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE aml_rules_rule_type AS ENUM ('VELOCITY', 'STRUCTURING', 'SANCTIONS', 'PEP')".to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE aml_rules_action_on_trigger AS ENUM ('FLAG', 'FREEZE_ACCOUNT', 'ALERT', 'BLOCK_TRANSACTION')".to_string(),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE TYPE aml_alerts_status AS ENUM ('OPEN', 'INVESTIGATING', 'RESOLVED', 'FALSE_POSITIVE')".to_string(),
            ))
            .await?;

        // AML rules table
        let aml_rules = Table::create()
            .table(AmlRules::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(AmlRules::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(AmlRules::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(AmlRules::RuleName).string().not_null())
            .col(ColumnDef::new(AmlRules::RuleDescription).text())
            .col(
                ColumnDef::new(AmlRules::RuleType)
                    .custom("aml_rules_rule_type")
                    .not_null(),
            )
            .col(
                ColumnDef::new(AmlRules::ConditionLogic)
                    .json_binary()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AmlRules::ActionOnTrigger)
                    .custom("aml_rules_action_on_trigger")
                    .not_null(),
            )
            .col(ColumnDef::new(AmlRules::IsActive).boolean().default(true))
            .col(ColumnDef::new(AmlRules::Priority).integer().default(1))
            .col(ColumnDef::new(AmlRules::CreatedBy).big_integer())
            .col(
                ColumnDef::new(AmlRules::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(AmlRules::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AmlRules::Table, AmlRules::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AmlRules::Table, AmlRules::CreatedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(aml_rules).await?;

        // AML alerts table
        let aml_alerts = Table::create()
            .table(AmlAlerts::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(AmlAlerts::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(AmlAlerts::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(AmlAlerts::RuleId).big_integer())
            .col(ColumnDef::new(AmlAlerts::AlertType).string().not_null())
            .col(
                ColumnDef::new(AmlAlerts::RiskLevel)
                    .custom("aml_risk_level_enum")
                    .default("MEDIUM"),
            )
            .col(ColumnDef::new(AmlAlerts::CustomerId).big_integer())
            .col(ColumnDef::new(AmlAlerts::AccountId).big_integer())
            .col(ColumnDef::new(AmlAlerts::TransactionId).big_integer())
            .col(
                ColumnDef::new(AmlAlerts::AlertDetails)
                    .json_binary()
                    .not_null(),
            )
            .col(ColumnDef::new(AmlAlerts::AlertScore).decimal_len(10, 6))
            .col(
                ColumnDef::new(AmlAlerts::Status)
                    .custom("aml_alerts_status")
                    .default("OPEN"),
            )
            .col(ColumnDef::new(AmlAlerts::AssignedInvestigator).big_integer())
            .col(ColumnDef::new(AmlAlerts::InvestigationNotes).string())
            .col(ColumnDef::new(AmlAlerts::Resolution).string())
            .col(ColumnDef::new(AmlAlerts::ResolvedBy).big_integer())
            .col(ColumnDef::new(AmlAlerts::ResolvedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(AmlAlerts::ActionsTaken).json_binary())
            .col(
                ColumnDef::new(AmlAlerts::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(AmlAlerts::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AmlAlerts::Table, AmlAlerts::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AmlAlerts::Table, AmlAlerts::RuleId)
                    .to(AmlRules::Table, AmlRules::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AmlAlerts::Table, AmlAlerts::CustomerId)
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AmlAlerts::Table, AmlAlerts::AccountId)
                    .to(Accounts::Table, Accounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AmlAlerts::Table, AmlAlerts::TransactionId)
                    .to(Transactions::Table, Transactions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AmlAlerts::Table, AmlAlerts::AssignedInvestigator)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AmlAlerts::Table, AmlAlerts::ResolvedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(aml_alerts).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AmlAlerts::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(AmlRules::Table).to_owned())
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "DROP TYPE IF EXISTS risk_level_enum".to_string(),
            ))
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum AmlRules {
    Table,
    Id,
    InstitutionId,
    RuleName,
    RuleDescription,
    RuleType,
    ConditionLogic,
    ActionOnTrigger,
    IsActive,
    Priority,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum AmlAlerts {
    Table,
    Id,
    InstitutionId,
    RuleId,
    AlertType,
    RiskLevel,
    CustomerId,
    AccountId,
    TransactionId,
    AlertDetails,
    AlertScore,
    Status,
    AssignedInvestigator,
    InvestigationNotes,
    Resolution,
    ResolvedBy,
    ResolvedAt,
    ActionsTaken,
    CreatedAt,
    UpdatedAt,
}
