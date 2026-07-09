use chrono::{DateTime, NaiveDate, Utc};
use entity::sea_orm_active_enums::{AccLimitType, AccLimitUnit, AccLinkType};
use serde::Deserialize;
use validator::Validate;

use crate::utils::validators::validate_snowflake;

// ===========================================
// Models
// ===========================================

#[derive(Debug, Clone)]
pub struct AddAccountModel {
    pub institution_id: i64,
    pub customer_id: i64,
    pub account_type_id: i64,
    pub account_number: String,
    pub account_name: String,
}

#[derive(Debug, Clone)]
pub struct AddAccountBalanceModel {
    pub account_id: i64,
    pub balance_date: Option<NaiveDate>,
    pub opening_balance: i64,
}

#[derive(Debug, Clone)]
pub struct AddAccountLinkModel {
    pub institution_id: i64,
    pub prim_account_id: i64,
    pub link_account_id: i64,
    pub link_type: AccLinkType,
    pub relationship: Option<String>,
    pub authorized_limit: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct AddAccountLimitModel {
    pub account_id: i64,
    pub limit_type: AccLimitType,
    pub limit_unit: AccLimitUnit,
    pub limit_value: i64,
    pub current_value: i64,
    pub effective_from: DateTime<Utc>,
    pub effective_to: DateTime<Utc>
}

// ==============================================
// Params
// ==============================================
#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct AddAccountParams {
    #[validate(custom(function = "validate_snowflake"))]
    #[serde(rename = "customerId")]
    pub customer_id: String,

    #[validate(custom(function = "validate_snowflake"))]
    #[serde(rename = "accountTypeId")]
    pub account_type_id: String,
}
