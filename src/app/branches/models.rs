use sea_orm::prelude::Decimal;
use serde::Deserialize;
use serde_json::Value;
use validator::Validate;

#[derive(Debug, Clone)]
pub struct AddBranchModel {
    pub name: String,
    pub code: String,
    pub institution: i64,
    pub address: Value,
    pub phone: String,
    pub email: String,
    pub location: Value,
    pub is_main: bool,
    pub cash_limit: Decimal,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct AddBranchParams {
    #[validate(length(min = 3, max = 120, message = "Name is invalid"))]
    pub name: String,
    #[serde(rename = "institutionId")]
    pub institution_id: String,
    pub address: Value,
    pub phone: String,
    pub email: String,
    pub location: Value,
    #[serde(rename = "cashLimit")]
    pub cash_limit: Decimal,
    #[serde(rename = "isMainBranch")]
    pub is_main_branch: bool,
}
