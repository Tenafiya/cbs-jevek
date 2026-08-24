use chrono::{DateTime, FixedOffset, Utc};
use entity::sea_orm_active_enums::{
    AmlCaseStatus, AmlCasesPriority, AmlRulesActionOnTrigger, AmlRulesRuleType, CustomerType,
    StaffEmploymentEnum, TransactionCategoryType, TransactionStatus, TransactionType,
};
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

use crate::utils::validators::{validate_cash_type, validate_date_range, validate_income};

#[derive(Debug, Clone)]
pub struct CurrencyModel {
    pub name: String,
    pub symbol: String,
    pub precision: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
#[validate(schema(function = "validate_date_range"))]
pub struct DateStruct {
    #[serde(rename = "effectiveFrom")]
    pub effective_from: DateTime<FixedOffset>,
    #[serde(rename = "effectiveTo")]
    pub effective_to: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffSelectFields {
    #[serde(rename = "_id")]
    pub id: String,
    pub full_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountCategorySummary {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: Option<String>,
    pub category_type: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountTypeSummary {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: Option<String>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub minimum_balance: Option<i64>,
    pub maximum_balance: Option<i64>,
    pub interest_rate: Option<Decimal>,
    pub maintenance_fee: Option<i64>,
    pub withdrawal_fee: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerSummary {
    #[serde(rename = "_id")]
    pub id: String,
    pub customer_type: Option<CustomerType>,
    pub customer_number: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSummary {
    #[serde(rename = "_id")]
    pub id: String,
    pub acccount_number: Option<String>,
    pub account_name: Option<String>,
    pub currency: Option<Value>,
    pub current_balance: Option<i64>,
    pub available_balance: Option<i64>,
    pub ledger_balance: Option<i64>,
    pub hold_balance: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffSummary {
    #[serde(rename = "_id")]
    pub id: String,
    pub employee_number: String,
    pub full_name: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub email_address: String,
    pub job_title: Option<String>,
    pub department: Option<String>,
    pub employment_status: Option<StaffEmploymentEnum>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TellerSummary {
    #[serde(rename = "_id")]
    pub id: String,
    pub teller_name: String,
    pub teller_number: String,
    pub branch_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionSummary {
    #[serde(rename = "_id")]
    pub id: String,
    pub transaction_reference: Option<String>,
    pub transaction_group_id: uuid::Uuid,
    pub transaction_type: TransactionType,
    pub transaction_category: TransactionCategoryType,
    pub amount: i64,
    pub currency: Option<Value>,
    pub status: TransactionStatus,
    pub posted_at: Option<DateTime<FixedOffset>>,
    pub completed_at: Option<DateTime<FixedOffset>>,
    pub failed_at: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmlRuleSummary {
    #[serde(rename = "_id")]
    pub id: String,
    pub rule_name: String,
    pub rule_description: Option<String>,
    pub rule_type: AmlRulesRuleType,
    pub condition_logic: Value,
    pub action_on_trigger: AmlRulesActionOnTrigger,
    pub is_enabled: Option<bool>,

    pub creator: Option<StaffSelectFields>,
    pub updater: Option<StaffSelectFields>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmlCaseSummary {
    #[serde(rename = "_id")]
    pub id: String,
    pub case_number: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: AmlCasesPriority,
    pub status: AmlCaseStatus,
}

#[derive(Debug, Validate, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PathParamsModel {
    pub id: String,
}

#[derive(Debug, Validate, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QueryParamsModel {
    #[serde(default = "default_per_page")]
    pub size: u64,
    #[serde(default = "default_page")]
    pub page: u64,
}

#[derive(Debug, Validate, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CursorQueryParams {
    pub cursor: Option<String>,
    #[serde(default = "default_limit")]
    pub limit: u64,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct CurrencyParams {
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    #[validate(length(min = 1, max = 10))]
    pub symbol: String,

    #[validate(length(equal = 3))]
    pub code: String,

    #[validate(range(min = 0, max = 18))]
    pub precision: i32,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct CashParams {
    #[validate(custom(function = "validate_income"))]
    pub denomination: Decimal,

    #[validate(range(min = 0, max = 100000))]
    pub quantity: i32,

    #[validate(custom(function = "validate_cash_type"))]
    #[serde(rename = "cashType")]
    pub cash_type: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct ChequeParams {
    #[serde(rename = "chequeNumber")]
    pub cheque_number: String,

    #[serde(rename = "bankName")]
    pub bank_name: String,

    #[serde(rename = "branchName")]
    pub branch_name: Option<String>,

    #[serde(rename = "accountNumber")]
    pub account_number: Option<String>,

    #[validate(custom(function = "validate_income"))]
    pub amount: Decimal,

    #[serde(rename = "currency")]
    pub currency: String,

    #[serde(rename = "issueDate")]
    pub issue_date: Option<chrono::NaiveDate>,

    #[serde(rename = "drawerName")]
    pub drawer_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MetaModel {
    pub total_items: u64,
    pub total_pages: u64,
    pub page: u64,
    pub per_page: u64,
}

#[derive(Debug, Serialize)]
pub struct ListResponseModel<T, F> {
    pub items: T,
    pub meta: F,
}

#[derive(Debug, Clone)]
pub struct QueryModel {
    pub size: u64,
    pub page: u64,
}

#[derive(Debug, Clone)]
pub struct CursorModel {
    pub cursor: Option<i64>,
    pub limit: u64,
}

#[derive(Debug, Serialize)]
pub struct CursorMetaModel {
    pub next_cursor: Option<String>,
    pub has_next: bool,
    pub limit: u64,
}

#[derive(Debug, Deserialize)]
pub struct DateQuery {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}

fn default_page() -> u64 {
    1
}

fn default_per_page() -> u64 {
    10
}

pub fn default_decimal() -> Decimal {
    Decimal::ZERO
}

fn default_limit() -> u64 {
    20
}
