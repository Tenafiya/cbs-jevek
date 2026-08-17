use entity::sea_orm_active_enums::TellerReconType;
use sea_orm::prelude::Decimal;
use serde::Deserialize;
use serde_json::Value;
use validator::Validate;

use crate::utils::{models::CashParams, validators::validate_snowflake};

#[derive(Debug, Clone)]
pub struct AddTellerModel {
    pub institution_id: i64,
    pub branch_id: i64,
    pub teller_name: String,
    pub teller_number: String,
    pub staff_id: i64,
}

#[derive(Debug, Clone)]
pub struct AddTellerReconModel {
    pub cash_drawer_id: i64,
    pub recon_type: Option<TellerReconType>,
    pub notes: Option<String>,
    pub supervisor_id: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct AddDrawerModel {
    pub teller_id: i64,
    pub opening_cash_amount: i64,
    pub opening_cash: Value,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct AddTellerParams {
    #[validate(custom(function = "validate_snowflake"))]
    #[serde(rename = "staffId")]
    pub staff_id: String,

    #[validate(custom(function = "validate_snowflake"))]
    #[serde(rename = "branchId")]
    pub branch_id: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct AddTellerReconParams {
    #[validate(custom(function = "validate_snowflake"))]
    #[serde(rename = "cashDrawerId")]
    pub cash_drawer_id: String,

    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct AddDrawerParams {
    #[validate(custom(function = "validate_snowflake"))]
    #[serde(rename = "tellerId")]
    pub teller_id: String,

    #[serde(rename = "openingCashAmount")]
    pub opening_cash_amount: Decimal,

    #[validate(nested)]
    #[serde(rename = "openingCash")]
    pub opening_cash: CashParams,
}
