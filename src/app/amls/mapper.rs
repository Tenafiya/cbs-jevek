use chrono::{DateTime, FixedOffset};
use entity::sea_orm_active_enums::{
    AmlAlertsAlertType, AmlAlertsStatus, AmlCaseStatus, AmlCasesPriority, AmlRiskLevelEnum,
    AmlRulesActionOnTrigger, AmlRulesRuleType, CustomerType, StaffEmploymentEnum,
    TransactionCategoryType, TransactionStatus, TransactionType,
};
use sea_orm::{FromQueryResult, prelude::*};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::models::{
    AccountSummary, AmlCaseSummary, AmlRuleSummary, CustomerSummary, StaffSelectFields,
    StaffSummary, TransactionSummary,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmlRuleRow {
    #[serde(rename = "_id")]
    pub id: String,
    pub institution_id: String,
    pub rule_name: String,
    pub rule_description: Option<String>,
    pub rule_type: AmlRulesRuleType,
    pub condition_logic: Value,
    pub action_on_trigger: AmlRulesActionOnTrigger,
    pub is_enabled: Option<bool>,
    pub priority: Option<i32>,
    pub stop_processing: Option<bool>,
    pub version: Option<i32>,
    pub effective_from: Option<DateTime<FixedOffset>>,
    pub effective_to: Option<DateTime<FixedOffset>>,
    pub created_at: Option<DateTime<FixedOffset>>,
    pub updated_at: Option<DateTime<FixedOffset>>,

    pub creator: Option<StaffSummary>,
    pub updater: Option<StaffSummary>,
}

#[derive(FromQueryResult, Debug, Clone)]
pub struct AmlRuleFlat {
    pub id: i64,
    pub institution_id: i64,
    pub rule_name: String,
    pub rule_description: Option<String>,
    pub rule_type: AmlRulesRuleType,
    pub condition_logic: Value,
    pub action_on_trigger: AmlRulesActionOnTrigger,
    pub is_enabled: Option<bool>,
    pub priority: Option<i32>,
    pub stop_processing: Option<bool>,
    pub version: Option<i32>,
    pub effective_from: Option<DateTime<FixedOffset>>,
    pub effective_to: Option<DateTime<FixedOffset>>,
    pub created_at: Option<DateTime<FixedOffset>>,
    pub updated_at: Option<DateTime<FixedOffset>>,

    pub creator_id: Option<i64>,
    pub creator_employee_number: String,
    pub creator_full_name: Option<String>,
    pub creator_first_name: String,
    pub creator_last_name: String,
    pub creator_phone_number: String,
    pub creator_email_address: String,
    pub creator_job_title: Option<String>,
    pub creator_department: Option<String>,
    pub creator_employment_status: Option<StaffEmploymentEnum>,

    pub updater_id: Option<i64>,
    pub updater_employee_number: String,
    pub updater_full_name: Option<String>,
    pub updater_first_name: String,
    pub updater_last_name: String,
    pub updater_phone_number: String,
    pub updater_email_address: String,
    pub updater_job_title: Option<String>,
    pub updater_department: Option<String>,
    pub updater_employment_status: Option<StaffEmploymentEnum>,
}

impl From<AmlRuleFlat> for AmlRuleRow {
    fn from(value: AmlRuleFlat) -> Self {
        Self {
            id: value.id.to_string(),
            institution_id: value.institution_id.to_string(),
            rule_name: value.rule_name,
            rule_description: value.rule_description,
            rule_type: value.rule_type,
            condition_logic: value.condition_logic,
            action_on_trigger: value.action_on_trigger,
            is_enabled: value.is_enabled,
            priority: value.priority,
            stop_processing: value.stop_processing,
            version: value.version,
            effective_from: value.effective_from,
            effective_to: value.effective_to,
            created_at: value.created_at,
            updated_at: value.updated_at,

            creator: value.creator_id.map(|id| StaffSummary {
                id: id.to_string(),
                employee_number: value.creator_employee_number,
                full_name: value.creator_full_name,
                first_name: value.creator_first_name,
                last_name: value.creator_last_name,
                phone_number: value.creator_phone_number,
                email_address: value.creator_email_address,
                job_title: value.creator_job_title,
                department: value.creator_department,
                employment_status: value.creator_employment_status,
            }),

            updater: value.updater_id.map(|id| StaffSummary {
                id: id.to_string(),
                employee_number: value.updater_employee_number,
                full_name: value.updater_full_name,
                first_name: value.updater_first_name,
                last_name: value.updater_last_name,
                phone_number: value.updater_phone_number,
                email_address: value.updater_email_address,
                job_title: value.updater_job_title,
                department: value.updater_department,
                employment_status: value.updater_employment_status,
            }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmlCaseRow {
    #[serde(rename = "_id")]
    pub id: String,
    pub institution_id: String,
    pub case_number: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: AmlCasesPriority,
    pub status: AmlCaseStatus,
    pub resolution: Option<String>,
    pub resolved_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,

    pub assigned_investigator: Option<StaffSummary>,
    pub resolved_by: Option<StaffSummary>,
}

#[derive(Debug, FromQueryResult, Clone)]
pub struct AmlCaseFlat {
    pub id: i64,
    pub institution_id: i64,
    pub case_number: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: AmlCasesPriority,
    pub status: AmlCaseStatus,
    pub resolution: Option<String>,
    pub resolved_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,

    pub assigned_investigator: Option<i64>,
    pub investigator_employee_number: String,
    pub investigator_full_name: Option<String>,
    pub investigator_first_name: String,
    pub investigator_last_name: String,
    pub investigator_phone_number: String,
    pub investigator_email_address: String,
    pub investigator_job_title: Option<String>,
    pub investigator_department: Option<String>,
    pub investigator_employment_status: Option<StaffEmploymentEnum>,

    pub resolved_by: Option<i64>,
    pub resolved_by_employee_number: String,
    pub resolved_by_full_name: Option<String>,
    pub resolved_by_first_name: String,
    pub resolved_by_last_name: String,
    pub resolved_by_phone_number: String,
    pub resolved_by_email_address: String,
    pub resolved_by_job_title: Option<String>,
    pub resolved_by_department: Option<String>,
    pub resolved_by_employment_status: Option<StaffEmploymentEnum>,
}

impl From<AmlCaseFlat> for AmlCaseRow {
    fn from(value: AmlCaseFlat) -> Self {
        Self {
            id: value.id.to_string(),
            institution_id: value.institution_id.to_string(),
            case_number: value.case_number,
            title: value.title,
            description: value.description,
            priority: value.priority,
            status: value.status,
            resolution: value.resolution,
            resolved_at: value.resolved_at,
            created_at: value.created_at,
            updated_at: value.updated_at,

            assigned_investigator: value.assigned_investigator.map(|id| StaffSummary {
                id: id.to_string(),
                employee_number: value.investigator_employee_number,
                full_name: value.investigator_full_name,
                first_name: value.investigator_first_name,
                last_name: value.investigator_last_name,
                phone_number: value.investigator_phone_number,
                email_address: value.investigator_email_address,
                job_title: value.investigator_job_title,
                department: value.investigator_department,
                employment_status: value.investigator_employment_status,
            }),
            resolved_by: value.resolved_by.map(|id| StaffSummary {
                id: id.to_string(),
                employee_number: value.resolved_by_employee_number,
                full_name: value.resolved_by_full_name,
                first_name: value.resolved_by_first_name,
                last_name: value.resolved_by_last_name,
                phone_number: value.resolved_by_phone_number,
                email_address: value.resolved_by_email_address,
                job_title: value.resolved_by_job_title,
                department: value.resolved_by_department,
                employment_status: value.resolved_by_employment_status,
            }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmlAlertRow {
    #[serde(rename = "_id")]
    pub id: String,
    pub institution_id: String,
    pub risk_level: Option<AmlRiskLevelEnum>,
    pub alert_type: AmlAlertsAlertType,
    pub alert_details: Option<Value>,
    pub risk_breakdown: Option<Value>,
    pub risk_score: Option<Decimal>,
    pub status: Option<AmlAlertsStatus>,
    pub detected_at: Option<DateTime<FixedOffset>>,
    pub created_at: Option<DateTime<FixedOffset>>,
    pub updated_at: Option<DateTime<FixedOffset>>,

    pub rule_id: Option<AmlRuleSummary>,

    pub case_id: Option<AmlCaseSummary>,

    pub customer: Option<CustomerSummary>,

    pub account_id: Option<AccountSummary>,

    pub transaction_id: Option<TransactionSummary>,
}

#[derive(Debug, FromQueryResult, Clone)]
pub struct AmlAlertFlat {
    pub id: i64,
    pub institution_id: i64,
    pub risk_level: Option<AmlRiskLevelEnum>,
    pub alert_type: AmlAlertsAlertType,
    pub alert_details: Option<Value>,
    pub risk_breakdown: Option<Value>,
    pub risk_score: Option<Decimal>,
    pub status: Option<AmlAlertsStatus>,
    pub detected_at: Option<DateTime<FixedOffset>>,
    pub created_at: Option<DateTime<FixedOffset>>,
    pub updated_at: Option<DateTime<FixedOffset>>,

    pub rule_id: Option<i64>,
    pub rule_name: String,
    pub rule_description: Option<String>,
    pub rule_type: AmlRulesRuleType,
    pub condition_logic: Value,
    pub action_logic: AmlRulesActionOnTrigger,
    pub is_enabled: Option<bool>,

    pub rule_creator_id: Option<i64>,
    pub rule_creator_full_name: Option<String>,

    pub rule_updater_id: Option<i64>,
    pub rule_updater_full_name: Option<String>,

    pub case_id: Option<i64>,
    pub case_number: String,
    pub case_title: String,
    pub case_description: Option<String>,
    pub case_status: AmlCaseStatus,
    pub case_priority: AmlCasesPriority,

    pub customer_id: Option<i64>,
    pub customer_type: Option<CustomerType>,
    pub customer_number: Option<String>,
    pub customer_first_name: Option<String>,
    pub customer_last_name: Option<String>,

    pub account_id: Option<i64>,
    pub acccount_number: Option<String>,
    pub account_name: Option<String>,
    pub currency: Option<Value>,
    pub current_balance: Option<i64>,
    pub available_balance: Option<i64>,
    pub ledger_balance: Option<i64>,
    pub hold_balance: Option<i64>,

    pub transaction_id: Option<i64>,
    pub transaction_reference: Option<String>,
    pub transaction_group_id: uuid::Uuid,
    pub transaction_type: TransactionType,
    pub transaction_category: TransactionCategoryType,
    pub transaction_amount: i64,
    pub transaction_currency: Option<Value>,
    pub transaction_status: TransactionStatus,
    pub transaction_posted_at: Option<DateTime<FixedOffset>>,
    pub transaction_completed_at: Option<DateTime<FixedOffset>>,
    pub transaction_failed_at: Option<DateTime<FixedOffset>>,
}

impl From<AmlAlertFlat> for AmlAlertRow {
    fn from(flat: AmlAlertFlat) -> Self {
        Self {
            id: flat.id.to_string(),
            institution_id: flat.institution_id.to_string(),
            risk_level: flat.risk_level,
            alert_type: flat.alert_type,
            alert_details: flat.alert_details,
            risk_breakdown: flat.risk_breakdown,
            risk_score: flat.risk_score,
            status: flat.status,
            detected_at: flat.detected_at,
            created_at: flat.created_at,
            updated_at: flat.updated_at,

            rule_id: flat.rule_id.map(|id| AmlRuleSummary {
                id: id.to_string(),
                rule_name: flat.rule_name,
                rule_description: flat.rule_description,
                rule_type: flat.rule_type,
                condition_logic: flat.condition_logic,
                action_on_trigger: flat.action_logic,
                is_enabled: flat.is_enabled,
                creator: flat.rule_creator_id.map(|id| StaffSelectFields {
                    id: id.to_string(),
                    full_name: flat.rule_creator_full_name,
                }),
                updater: flat.rule_updater_id.map(|id| StaffSelectFields {
                    id: id.to_string(),
                    full_name: flat.rule_updater_full_name,
                }),
            }),

            case_id: flat.case_id.map(|id| AmlCaseSummary {
                id: id.to_string(),
                case_number: flat.case_number,
                title: flat.case_title,
                description: flat.case_description,
                priority: flat.case_priority,
                status: flat.case_status,
            }),

            customer: flat.customer_id.map(|id| CustomerSummary {
                id: id.to_string(),
                customer_type: flat.customer_type,
                customer_number: flat.customer_number,
                first_name: flat.customer_first_name,
                last_name: flat.customer_last_name,
            }),

            account_id: flat.account_id.map(|id| AccountSummary {
                id: id.to_string(),
                acccount_number: flat.acccount_number,
                account_name: flat.account_name,
                currency: flat.currency,
                current_balance: flat.current_balance,
                available_balance: flat.available_balance,
                ledger_balance: flat.ledger_balance,
                hold_balance: flat.hold_balance,
            }),

            transaction_id: flat.transaction_id.map(|id| TransactionSummary {
                id: id.to_string(),
                transaction_reference: flat.transaction_reference,
                transaction_group_id: flat.transaction_group_id,
                transaction_type: flat.transaction_type,
                transaction_category: flat.transaction_category,
                amount: flat.transaction_amount,
                currency: flat.transaction_currency,
                status: flat.transaction_status,
                posted_at: flat.transaction_posted_at,
                completed_at: flat.transaction_completed_at,
                failed_at: flat.transaction_failed_at,
            }),
        }
    }
}
