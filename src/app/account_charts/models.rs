use chrono::{DateTime, FixedOffset};
use entity::sea_orm_active_enums::{AccTypeIntCalc, AccTypeIntPayoutFreq, AccTypeStatus};
use sea_orm::{DerivePartialModel, FromQueryResult, prelude::Decimal};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{DisplayFromStr, serde_as};
use validator::Validate;

use crate::utils::{models::AccountCategorySummary, validators::{validate_acc_cat_type, validate_income, validate_snowflake}};

#[serde_as]
#[derive(Debug, Clone, Serialize, FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "entity::chart_of_accounts::Entity")]
pub struct ChartOfAccountResponseModel {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "_id")]
    #[sea_orm(from_col = "id")]
    pub id: i64,

    #[sea_orm(from_col = "account_code")]
    pub account_code: Option<String>,

    #[sea_orm(from_col = "account_name")]
    pub account_name: Option<String>,

    #[sea_orm(from_col = "account_type")]
    pub account_type: Option<String>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[sea_orm(from_col = "parent_account_id")]
    pub parent_account_id: Option<i64>,

    #[sea_orm(from_col = "is_active")]
    pub is_active: Option<bool>,

    #[sea_orm(from_col = "is_system_account")]
    pub is_system_account: Option<bool>,

     #[sea_orm(from_col = "currency_code")]
    pub currency_code: Option<String>,

    #[sea_orm(from_col = "created_at")]
    pub created_at: Option<DateTime<FixedOffset>>,

    #[sea_orm(from_col = "updated_at")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "entity::account_categories::Entity")]
pub struct AccountCategoryResponseModel {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "_id")]
    #[sea_orm(from_col = "id")]
    pub id: i64,

    #[serde_as(as = "DisplayFromStr")]
    #[sea_orm(from_col = "institution_id")]
    pub institution_id: i64,

    #[sea_orm(from_col = "name")]
    pub name: Option<String>,

    #[sea_orm(from_col = "category_type")]
    pub category_type: Option<String>,

    #[sea_orm(from_col = "is_active")]
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountTypeRow {
    #[serde(rename = "_id")]
    pub id: String,
    pub institution_id: String,
    pub name: Option<String>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub currency: Option<Value>,
    pub minimum_balance: Option<i64>,
    pub maximum_balance: Option<i64>,
    pub interest_rate: Option<Decimal>,
    pub interest_rate_calc_method: Option<String>,
    pub kyc_tier: Option<Value>,
    pub interest_payout_frequency: Option<String>,
    pub is_overdraft_allowable: Option<bool>,
    pub overdraft_limit: Option<i64>,
    pub overdraft_interest_rate: Option<Decimal>,
    pub dormancy_period_days: Option<i32>,
    pub maintenance_fee: Option<i64>,
    pub withdrawal_fee: Option<i64>,
    pub status: Option<AccTypeStatus>,
    pub custom_fields: Option<Value>,
    pub created_at: Option<DateTime<FixedOffset>>,
    pub updated_at: Option<DateTime<FixedOffset>>,

    pub category_id: AccountCategorySummary
}

#[derive(FromQueryResult, Debug, Clone)]
pub struct AccountTypeFlat {
    pub id: i64,
    pub institution_id: i64,
    pub name: Option<String>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub currency: Option<Value>,
    pub minimum_balance: Option<i64>,
    pub maximum_balance: Option<i64>,
    pub interest_rate: Option<Decimal>,
    pub interest_rate_calc_method: Option<String>,
    pub kyc_tier: Option<Value>,
    pub interest_payout_frequency: Option<String>,
    pub is_overdraft_allowable: Option<bool>,
    pub overdraft_limit: Option<i64>,
    pub overdraft_interest_rate: Option<Decimal>,
    pub dormancy_period_days: Option<i32>,
    pub maintenance_fee: Option<i64>,
    pub withdrawal_fee: Option<i64>,
    pub status: Option<AccTypeStatus>,
    pub custom_fields: Option<Value>,
    pub created_at: Option<DateTime<FixedOffset>>,
    pub updated_at: Option<DateTime<FixedOffset>>,

    pub category_id: i64,
    pub category_name: Option<String>,
    pub category_category_type: Option<String>,
    pub category_description: Option<String>,
    pub category_is_active: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct AddAccountChartModel {
    pub institution_id: i64,
    pub acc_code: String,
    pub acc_name: String,
    pub acc_type: String,
    pub currency_code: String,
    pub parent_acc_id: Option<i64>,
    pub is_system_acc: bool
}

#[derive(Debug, Clone)]
pub struct AddAccountTypeModel {
    pub institution_id: i64,
    pub category_id: i64,
    pub name: Option<String>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub currency: Option<Value>,
    pub min_balance: i64,
    pub max_balance: i64,
    pub interest_rate: Decimal,
    pub interest_rate_calc: AccTypeIntCalc,
    pub interest_payout_freq: AccTypeIntPayoutFreq,
    pub is_overdraft_allowable: bool,
    pub overdraft_limit: Option<i64>,
    pub overdraft_interest_rate: Decimal,
    pub dormancy_period: Option<i32>,
    pub maintenance_fee: Option<i64>,
    pub withdrawal_fee: Option<i64>,
    pub status: AccTypeStatus
}

#[derive(Debug, Clone)]
pub struct AddAccountCategoryModel {
    pub institution_id: i64,
    pub name: Option<String>,
    pub category_type: Option<String>,  //'SAVINGS', 'CURRENT', 'FIXED_DEPOSIT', 'LOAN', 'WALLET', 'AGENT_FLOAT', 'SUSU'
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct AddAccountCategoryParams {
    #[validate(length(min = 2, max = 50, message = "Invalid name"))]
    pub name: String,

    #[validate(custom(function = "validate_acc_cat_type"))]
    #[serde(rename = "categoryType")]
    pub category_type: String,

    #[validate(length(min = 2, max = 150, message = "Description is invalid"))]
    pub description: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct AddAccountChartParams {
    #[validate(length(min = 2, max = 50, message = "Invalid account name"))]
    #[serde(rename = "accountName")]
    pub acc_name: String,

    #[validate(length(min = 2, max = 50, message = "Invalid account type"))]
    #[serde(rename = "accountType")]
    pub acc_type: String,

    #[validate(length(min = 2, max = 10, message = "Invalid currency code"))]
    #[serde(rename = "currencyCode")]
    pub currency_code: String,

    #[validate(custom(function = "validate_snowflake"))]
    #[serde(rename = "parentAccountId")]
    pub parent_account: Option<String>,

    #[serde(rename = "isSystemAccount")]
    pub is_system_acc: bool
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct AddAccountTypeParams {
    #[validate(custom(function = "validate_snowflake"))]
    #[serde(rename = "categoryId")]
    pub category_id: String,

    #[validate(length(min = 2, max = 50, message = "Name is invalid"))]
    pub name: String,

    #[validate(length(min = 2, max = 150, message = "Description is invalid"))]
    pub description: String,

    #[validate(custom(function = "validate_income"))]
    #[serde(rename = "minBalance")]
    pub min_balance: Decimal,

    #[validate(custom(function = "validate_income"))]
    #[serde(rename = "maxBalance")]
    pub max_balance: Decimal,

    #[validate(custom(function = "validate_income"))]
    #[serde(rename = "interestRate")]
    pub interest_rate: Decimal,

    #[serde(rename = "interestPayoutFreq")]
    pub interest_payout_freq: AccTypeIntPayoutFreq,

    #[serde(rename = "interestRateCalc")]
    pub interest_rate_calc: AccTypeIntCalc,

    #[serde(rename = "isOverdraftAllowed")]
    pub is_overdraft_allowable: bool,

    #[validate(range(min = 0, max = 10000))]
    #[serde(rename = "overdraftLimit")]
    pub overdraft_limit: Option<i64>,

    #[validate(custom(function = "validate_income"))]
    #[serde(rename = "overdraftInterestRate")]
    pub overdraft_interest_rate: Decimal,

    #[validate(range(min = 0, max = 10000))]
    #[serde(rename = "dormancyPeriod")]
    pub dormancy_period: Option<i32>,

     #[validate(custom(function = "validate_income"))]
    #[serde(rename = "maintenanceFee")]
    pub maintenance_fee: Option<Decimal>,

    #[validate(custom(function = "validate_income"))]
    #[serde(rename = "withdrawalFee")]
    pub withdrawal_fee: Option<Decimal>,

    #[validate(nested)]
    pub currency: Option<CurrencyLayout>
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CurrencyLayout {
    #[validate(length(min = 2, max = 50, message = "Currency name is invalid"))]
    pub name: String,

    #[validate(length(min = 2, max = 50, message = "Currency code is invalid"))]
    pub code: String,

    #[validate(length(min = 2, max = 50, message = "Currency symbol is invalid"))]
    pub symbol: String,
}

impl From<AccountTypeFlat> for AccountTypeRow {
    fn from(flat: AccountTypeFlat) -> Self {
        Self {
            id: flat.id.to_string(),
            institution_id: flat.institution_id.to_string(),
            name: flat.name,
            code: flat.code,
            description: flat.description,
            currency: flat.currency,
            minimum_balance: flat.minimum_balance,
            maximum_balance: flat.maximum_balance,
            interest_rate: flat.interest_rate,
            interest_rate_calc_method: flat.interest_rate_calc_method,
            kyc_tier: flat.kyc_tier,
            interest_payout_frequency: flat.interest_payout_frequency,
            is_overdraft_allowable: flat.is_overdraft_allowable,
            overdraft_limit: flat.overdraft_limit,
            overdraft_interest_rate: flat.overdraft_interest_rate,
            dormancy_period_days: flat.dormancy_period_days,
            maintenance_fee: flat.maintenance_fee,
            withdrawal_fee: flat.withdrawal_fee,
            status: flat.status,
            custom_fields: flat.custom_fields,
            created_at: flat.created_at,
            updated_at: flat.updated_at,

            category_id: AccountCategorySummary {
                id: flat.category_id.to_string(),
                name: flat.category_name,
                category_type: flat.category_category_type,
                description: flat.category_description,
                is_active: flat.category_is_active,
            },
        }
    }
}