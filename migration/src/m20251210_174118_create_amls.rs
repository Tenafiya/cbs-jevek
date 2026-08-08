use sea_orm_migration::prelude::*;

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
            .execute_unprepared(
                r#"
                    CREATE TYPE aml_risk_level_enum AS ENUM ('LOW', 'MEDIUM', 'HIGH', 'CRITICAL')
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE TYPE aml_cases_priority AS ENUM (
                        'LOW',
                        'NORMAL',
                        'HIGH',
                        'URGENT',
                        'CRITICAL'
                    )
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE TYPE aml_rules_rule_type AS ENUM (
                        'TRANSACTION_AMOUNT',
                        'TRANSACTION_VELOCITY',
                        'TRANSACTION_PATTERN',
                        'STRUCTURING',
                        'GEOGRAPHIC_RISK',
                        'SANCTIONS_SCREENING',
                        'PEP_SCREENING',
                        'ADVERSE_MEDIA_SCREENING',
                        'CUSTOMER_RISK',
                        'ACCOUNT_ACTIVITY',
                        'BENEFICIARY_RISK',
                        'DEVICE_RISK',
                        'IMPOSSIBLE_TRAVEL',
                        'BEHAVIOURAL_ANOMALY',
                        'DORMANT_ACCOUNT_ACTIVITY',
                        'CASH_ACTIVITY',
                        'ACCOUNT_TAKEOVER',
                        'MULE_ACCOUNT',
                        'FUNDS_CYCLING',
                        'ROUND_TRIPPING',
                        'CUSTOM_RULE'
                    )
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE TYPE aml_alerts_alert_type AS ENUM (
                        'LARGE_TRANSACTION',
                        'STRUCTURING',
                        'RAPID_MOVEMENT_OF_FUNDS',
                        'UNUSUAL_TRANSACTION',
                        'UNUSUAL_TRANSACTION_PATTERN',
                        'HIGH_RISK_COUNTRY',
                        'SANCTIONS_MATCH',
                        'PEP_MATCH',
                        'ADVERSE_MEDIA',
                        'SUSPICIOUS_BENEFICIARY',
                        'SUSPICIOUS_ACCOUNT',
                        'DORMANT_ACCOUNT_ACTIVITY',
                        'UNUSUAL_CASH_ACTIVITY',
                        'UNUSUAL_DEPOSIT',
                        'UNUSUAL_WITHDRAWAL',
                        'UNUSUAL_TRANSFER',
                        'ACCOUNT_TAKEOVER',
                        'IDENTITY_MISMATCH',
                        'MULTIPLE_ACCOUNTS',
                        'MULE_ACCOUNT',
                        'FUNDS_CYCLING',
                        'ROUND_TRIPPING',
                        'FRAUD_SUSPECTED',
                        'OTHER'
                    )
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE TYPE aml_rules_action_on_trigger AS ENUM ('FLAG', 'FREEZE_ACCOUNT', 'ALERT', 'BLOCK_TRANSACTION')
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE TYPE aml_alerts_status AS ENUM ('NEW', 'PENDING_REVIEW', 'INVESTIGATING', 'RESOLVED', 'FALSE_POSITIVE', 'ESCALATED', 'CONFIRMED_SUSPICIOUS', 'DISMISSED')
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE TYPE aml_case_status AS ENUM ('OPEN', 'ASSIGNED', 'INVESTIGATING', 'WAITING_FOR_CUSTOMER', 'ESCALATED', 'CLOSED', 'ARCHIVED')
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE TYPE aml_rule_actions AS ENUM ('LOG_ONLY', 'GENERATE_ALERT', 'REQUIRE_ADDITIONAL_AUTHENTICATION', 'HOLD_TRANSACTION', 'REJECT_TRANSACTION', 'FREEZE_ACCOUNT', 'ESCALATE_TO_INVESTIGATOR', 'FILE_SAR_AUTOMATICALLY')
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE TYPE aml_watchlists_list_type AS ENUM (
                        'SANCTIONS',
                        'PEP',
                        'TERRORIST_FINANCING',
                        'WANTED_PERSON',
                        'LAW_ENFORCEMENT',
                        'ADVERSE_MEDIA',
                        'INTERNAL_BLACKLIST',
                        'INTERNAL_WATCHLIST',
                        'FRAUD',
                        'MONEY_LAUNDERING',
                        'HIGH_RISK_ENTITY',
                        'HIGH_RISK_COUNTRY',
                        'REGULATORY',
                        'OTHER'
                    )
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE TYPE aml_entity_type AS ENUM (
                        'CUSTOMER',
                        'ACCOUNT',
                        'BENEFICIARY',
                        'BUSINESS',
                        'ORGANIZATION',
                        'INDIVIDUAL',
                        'DEVICE',
                        'IP_ADDRESS',
                        'PHONE_NUMBER',
                        'EMAIL_ADDRESS',
                        'COUNTRY',
                        'TRANSACTION',
                        'OTHER'
                    )
                "#,
            )
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
            .col(ColumnDef::new(AmlRules::IsEnabled).boolean())
            .col(ColumnDef::new(AmlRules::Priority).integer().default(1))
            .col(ColumnDef::new(AmlRules::StopProcessing).boolean())
            .col(ColumnDef::new(AmlRules::Version).integer())
            .col(ColumnDef::new(AmlRules::EffectiveFrom).timestamp_with_time_zone())
            .col(ColumnDef::new(AmlRules::EffectiveTo).timestamp_with_time_zone())
            .col(ColumnDef::new(AmlRules::CreatedBy).big_integer())
            .col(ColumnDef::new(AmlRules::UpdatedBy).big_integer())
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
            .foreign_key(
                ForeignKey::create()
                    .from(AmlRules::Table, AmlRules::UpdatedBy)
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
            .col(ColumnDef::new(AmlAlerts::CaseId).big_integer())
            .col(
                ColumnDef::new(AmlAlerts::RiskLevel)
                    .custom("aml_risk_level_enum")
                    .default("LOW"),
            )
            .col(
                ColumnDef::new(AmlAlerts::AlertType)
                    .custom("aml_alerts_alert_type")
                    .not_null(),
            )
            .col(ColumnDef::new(AmlAlerts::CustomerId).big_integer())
            .col(ColumnDef::new(AmlAlerts::AccountId).big_integer())
            .col(ColumnDef::new(AmlAlerts::TransactionId).big_integer())
            .col(
                ColumnDef::new(AmlAlerts::AlertDetails)
                    .json_binary()
                    .not_null(),
            )
            .col(ColumnDef::new(AmlAlerts::RiskBreakdown).json_binary())
            .col(ColumnDef::new(AmlAlerts::RiskScore).decimal_len(10, 6))
            .col(
                ColumnDef::new(AmlAlerts::Status)
                    .custom("aml_alerts_status")
                    .default("NEW"),
            )
            .col(ColumnDef::new(AmlAlerts::DetectedAt).timestamp_with_time_zone())
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
            .to_owned();

        manager.create_table(aml_alerts).await?;

        let aml_executionss = Table::create()
            .table(AmlRuleExecutions::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(AmlRuleExecutions::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(AmlRuleExecutions::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AmlRuleExecutions::RuleId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(AmlRuleExecutions::CustomerId).big_integer())
            .col(ColumnDef::new(AmlRuleExecutions::AccountId).big_integer())
            .col(ColumnDef::new(AmlRuleExecutions::TransactionId).big_integer())
            .col(
                ColumnDef::new(AmlRuleExecutions::IsMatched)
                    .boolean()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AmlRuleExecutions::RiskScore)
                    .decimal_len(10, 6)
                    .not_null(),
            )
            .col(
                ColumnDef::new(AmlRuleExecutions::ExecutionTimeMs)
                    .integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AmlRuleExecutions::EvaluationDetails)
                    .json_binary()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AmlRuleExecutions::ExecutedAt)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AmlRuleExecutions::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp())
                    .not_null(),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AmlRuleExecutions::Table, AmlRuleExecutions::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AmlRuleExecutions::Table, AmlRuleExecutions::RuleId)
                    .to(AmlRules::Table, AmlRules::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AmlRuleExecutions::Table, AmlRuleExecutions::CustomerId)
                    .to(Customers::Table, Customers::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AmlRuleExecutions::Table, AmlRuleExecutions::AccountId)
                    .to(Accounts::Table, Accounts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AmlRuleExecutions::Table, AmlRuleExecutions::TransactionId)
                    .to(Transactions::Table, Transactions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(aml_executionss).await?;

        let aml_cases = Table::create()
            .table(AmlCases::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(AmlCases::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(AmlCases::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(AmlCases::CaseNumber).string().not_null())
            .col(ColumnDef::new(AmlCases::Title).string().not_null())
            .col(ColumnDef::new(AmlCases::Description).string())
            .col(
                ColumnDef::new(AmlCases::Priority)
                    .custom("aml_cases_priority")
                    .not_null(),
            )
            .col(
                ColumnDef::new(AmlCases::Status)
                    .custom("aml_case_status")
                    .not_null(),
            )
            .col(ColumnDef::new(AmlCases::AssignedInvestigator).big_integer())
            .col(ColumnDef::new(AmlCases::Resolution).string())
            .col(ColumnDef::new(AmlCases::ResolvedBy).big_integer())
            .col(ColumnDef::new(AmlCases::ResolvedAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(AmlCases::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp())
                    .not_null(),
            )
            .col(
                ColumnDef::new(AmlCases::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp())
                    .not_null(),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AmlCases::Table, AmlCases::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AmlCases::Table, AmlCases::AssignedInvestigator)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AmlCases::Table, AmlCases::ResolvedBy)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(aml_cases).await?;

        let notes = Table::create()
            .table(AmlCaseNotes::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(AmlCaseNotes::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(AmlCaseNotes::CaseId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AmlCaseNotes::InvestigatorId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(AmlCaseNotes::Note).string().not_null())
            .col(
                ColumnDef::new(AmlCaseNotes::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp())
                    .not_null(),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AmlCaseNotes::Table, AmlCaseNotes::CaseId)
                    .to(AmlCases::Table, AmlCases::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AmlCaseNotes::Table, AmlCaseNotes::InvestigatorId)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(notes).await?;

        let aml_actions = Table::create()
            .table(AmlActions::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(AmlActions::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(AmlActions::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(AmlActions::CaseId).big_integer().not_null())
            .col(ColumnDef::new(AmlActions::AlertId).big_integer().not_null())
            .col(ColumnDef::new(AmlActions::ActionType).custom("aml_rule_actions"))
            .col(ColumnDef::new(AmlActions::Performedby).big_integer())
            .col(ColumnDef::new(AmlActions::Metadata).json_binary())
            .col(
                ColumnDef::new(AmlActions::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp())
                    .not_null(),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AmlActions::Table, AmlActions::CaseId)
                    .to(AmlCases::Table, AmlCases::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AmlActions::Table, AmlActions::AlertId)
                    .to(AmlAlerts::Table, AmlAlerts::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AmlActions::Table, AmlActions::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AmlActions::Table, AmlActions::Performedby)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(aml_actions).await?;

        let aml_watchers = Table::create()
            .table(AmlWatchlists::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(AmlWatchlists::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(AmlWatchlists::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(AmlWatchlists::ListType)
                    .custom("aml_watchlists_list_type")
                    .not_null(),
            )
            .col(ColumnDef::new(AmlWatchlists::ExternalReferences).string())
            .col(ColumnDef::new(AmlWatchlists::FullName).string().not_null())
            .col(ColumnDef::new(AmlWatchlists::Country).string())
            .col(ColumnDef::new(AmlWatchlists::DateOfBirth).date())
            .col(
                ColumnDef::new(AmlWatchlists::Metadata)
                    .json_binary()
                    .not_null(),
            )
            .col(ColumnDef::new(AmlWatchlists::IsActive).boolean().not_null())
            .col(
                ColumnDef::new(AmlWatchlists::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp())
                    .not_null(),
            )
            .col(
                ColumnDef::new(AmlWatchlists::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp())
                    .not_null(),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AmlWatchlists::Table, AmlWatchlists::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(aml_watchers).await?;

        let aml_whitelist = Table::create()
            .table(AmlWhitelists::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(AmlWhitelists::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(AmlWhitelists::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(AmlWhitelists::EntityType).custom("aml_entity_type"))
            .col(ColumnDef::new(AmlWhitelists::EntityId).big_integer())
            .col(ColumnDef::new(AmlWhitelists::Reason).string())
            .col(ColumnDef::new(AmlWhitelists::ExpiresAt).timestamp_with_time_zone())
            .col(
                ColumnDef::new(AmlWhitelists::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp())
                    .not_null(),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AmlWhitelists::Table, AmlWhitelists::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(aml_whitelist).await?;

        let aml_blacklist = Table::create()
            .table(AmlBlacklists::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(AmlBlacklists::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(AmlBlacklists::InstitutionId)
                    .big_integer()
                    .not_null(),
            )
            .col(ColumnDef::new(AmlBlacklists::EntityType).custom("aml_entity_type"))
            .col(ColumnDef::new(AmlBlacklists::EntityId).big_integer())
            .col(ColumnDef::new(AmlBlacklists::Reason).string())
            .col(ColumnDef::new(AmlBlacklists::Severity).custom("aml_risk_level_enum"))
            .col(ColumnDef::new(AmlBlacklists::IsActive).boolean().not_null())
            .col(
                ColumnDef::new(AmlBlacklists::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp())
                    .not_null(),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(AmlBlacklists::Table, AmlBlacklists::InstitutionId)
                    .to(Institutions::Table, Institutions::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(aml_blacklist).await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE INDEX idx_aml_alerts_institution ON aml_alerts(institution_id);
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE INDEX idx_aml_alerts_customer ON aml_alerts(customer_id);
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE INDEX idx_aml_alerts_status ON aml_alerts(status);
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    CREATE INDEX idx_aml_alerts_date ON aml_alerts(created_at);
                "#,
            )
            .await?;

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
            .execute_unprepared(
                r#"
                    DROP TYPE IF EXISTS risk_level_enum
                "#,
            )
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
    IsEnabled,
    Priority,
    StopProcessing,
    Version,
    EffectiveFrom,
    EffectiveTo,
    CreatedBy,
    UpdatedBy,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum AmlAlerts {
    Table,
    Id,
    InstitutionId,
    RuleId,
    CaseId,
    AlertType,
    RiskLevel,
    CustomerId,
    AccountId,
    TransactionId,
    AlertDetails,
    RiskScore,
    Status,
    RiskBreakdown,
    DetectedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum AmlRuleExecutions {
    Table,
    Id,
    InstitutionId,
    RuleId,
    CustomerId,
    AccountId,
    TransactionId,
    IsMatched,
    RiskScore,
    ExecutionTimeMs,
    EvaluationDetails,
    ExecutedAt,
    CreatedAt,
}

#[derive(DeriveIden)]
pub enum AmlCases {
    Table,
    Id,
    InstitutionId,
    CaseNumber,
    Title,
    Description,
    Priority,
    Status,
    AssignedInvestigator,
    Resolution,
    ResolvedBy,
    ResolvedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum AmlCaseNotes {
    Table,
    Id,
    CaseId,
    InvestigatorId,
    Note,
    CreatedAt,
}

#[derive(DeriveIden)]
pub enum AmlActions {
    Table,
    Id,
    InstitutionId,
    CaseId,
    AlertId,
    ActionType,
    Performedby,
    Metadata,
    CreatedAt,
}

#[derive(DeriveIden)]
pub enum AmlWatchlists {
    Table,
    Id,
    InstitutionId,
    ListType,
    ExternalReferences,
    FullName,
    Country,
    DateOfBirth,
    Metadata,
    IsActive,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum AmlWhitelists {
    Table,
    Id,
    InstitutionId,
    EntityType,
    EntityId,
    Reason,
    ExpiresAt,
    CreatedAt,
}

#[derive(DeriveIden)]
pub enum AmlBlacklists {
    Table,
    Id,
    InstitutionId,
    EntityType,
    EntityId,
    Reason,
    Severity,
    IsActive,
    CreatedAt,
}
