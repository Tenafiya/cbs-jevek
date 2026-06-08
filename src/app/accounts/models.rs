use serde::Serialize;
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

// ==============================================
// Params
// ==============================================
#[derive(Debug, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct AddAccountParams {
    #[validate(custom(function = "validate_snowflake"))]
    #[serde(rename = "customerId")]
    pub customer_id: String,

    #[validate(custom(function = "validate_snowflake"))]
    #[serde(rename = "accountTypeId")]
    pub account_type_id: String,
}