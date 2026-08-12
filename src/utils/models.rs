use chrono::{DateTime, FixedOffset};
use entity::sea_orm_active_enums::CustomerType;
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

use crate::utils::validators::validate_date_range;

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
    pub institution_id: String,
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

#[derive(Debug, Deserialize, Validate)]
#[validate(schema(function = "validate_date_range"))]
pub struct DateStruct {
    pub effective_from: DateTime<FixedOffset>,
    pub effective_to: DateTime<FixedOffset>,
}
