use chrono::{DateTime, FixedOffset};
use entity::sea_orm_active_enums::{CustomerType, TransactionLimitsLimitType};
use sea_orm::DerivePartialModel;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{DisplayFromStr, serde_as};

use crate::utils::models::AccountCategorySummary;

// ===========================================
// Transaction Checker
// ===========================================
#[serde_as]
#[derive(Debug, Clone, Serialize, DerivePartialModel)]
#[sea_orm(entity = "entity::transaction_channels::Entity")]
pub struct TransactionChannelResponseModel {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "_id")]
    #[sea_orm(from_col = "id")]
    pub id: i64,
    #[serde_as(as = "DisplayFromStr")]
    #[sea_orm(from_col = "institution_id")]
    pub institution_id: i64,
    #[sea_orm(from_col = "channel_name")]
    pub channel_name: Option<String>,
    #[sea_orm(from_col = "channel_code")]
    pub channel_code: Option<String>,
    #[sea_orm(from_col = "description")]
    pub description: Option<String>,
    #[sea_orm(from_col = "is_active")]
    pub is_active: Option<bool>,
    #[sea_orm(from_col = "requires_maker_checker")]
    pub requires_maker_checker: bool,
    #[sea_orm(from_col = "metadata")]
    pub metadata: Option<Value>,
    #[sea_orm(from_col = "created_at")]
    pub created_at: Option<DateTime<FixedOffset>>,
    #[sea_orm(from_col = "updated_at")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

// ===========================================
// Transaction Checker
// ===========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionChannelSummary {
    #[serde(rename = "_id")]
    pub id: String,
    pub institution_id: String,
    pub channel_name: Option<String>,
    pub channel_code: Option<String>,
    pub requires_maker_checker: Option<bool>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionLimitSummary {
    #[serde(rename = "_id")]
    pub id: String,
    pub institution_id: String,
    pub transaction_channel_id: String,
    pub account_category_id: String,
    pub customer_type: CustomerType,
    pub limit_type: TransactionLimitsLimitType,
    pub max_amount: Option<i64>,
    pub max_count: Option<i64>,
    pub currency: Option<Value>,
    pub effective_from: Option<DateTime<FixedOffset>>,
    pub effective_to: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionCheckerRow {
    pub channel: TransactionChannelSummary,
    pub limit: Option<TransactionLimitSummary>,
}

#[derive(FromQueryResult, Debug, Clone)]
pub struct TransactionCheckerFlat {
    pub id: i64,
    pub institution_id: i64,
    pub channel_name: Option<String>,
    pub channel_code: Option<String>,
    pub requires_maker_checker: Option<bool>,
    pub metadata: Option<Value>,

    pub limit_id: Option<i64>,
    pub limit_institution_id: i64,
    pub limit_transaction_channel_id: i64,
    pub limit_account_category_id: i64,
    pub limit_customer_type: CustomerType,
    pub limit_limit_type: TransactionLimitsLimitType,
    pub limit_max_amount: Option<i64>,
    pub limit_max_count: Option<i64>,
    pub limit_currency: Option<Value>,
    pub limit_effective_from: Option<DateTime<FixedOffset>>,
    pub limit_effective_to: Option<DateTime<FixedOffset>>,
}

impl From<TransactionCheckerFlat> for TransactionCheckerRow {
    fn from(value: TransactionCheckerFlat) -> Self {
        Self {
            channel: TransactionChannelSummary {
                id: value.id.to_string(),
                institution_id: value.institution_id.to_string(),
                channel_name: value.channel_name,
                channel_code: value.channel_code,
                requires_maker_checker: value.requires_maker_checker,
                metadata: value.metadata,
            },

            limit: value.limit_id.and_then(|id| {
                Some(TransactionLimitSummary {
                    id: id.to_string(),
                    institution_id: value.limit_institution_id.to_string(),
                    transaction_channel_id: value.limit_transaction_channel_id.to_string(),
                    account_category_id: value.limit_account_category_id.to_string(),
                    customer_type: value.limit_customer_type,
                    limit_type: value.limit_limit_type,
                    max_amount: value.limit_max_amount,
                    max_count: value.limit_max_count,
                    currency: value.limit_currency,
                    effective_from: value.limit_effective_from,
                    effective_to: value.limit_effective_to,
                })
            }),
        }
    }
}

// ===========================================
// End Transaction Checker
// ===========================================
//
// ===========================================
// Transaction Limit
// ===========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionLimitRow {
    #[serde(rename = "_id")]
    pub id: String,
    pub institution_id: String,
    pub customer_type: CustomerType,
    pub limit_type: TransactionLimitsLimitType,
    pub max_amount: Option<i64>,
    pub max_count: Option<i32>,
    pub currency: Option<Value>,
    pub is_active: Option<bool>,
    pub effective_from: Option<DateTime<FixedOffset>>,
    pub effective_to: Option<DateTime<FixedOffset>>,
    pub kyc_tier: Option<Value>,
    pub created_at: Option<DateTime<FixedOffset>>,
    pub updated_at: Option<DateTime<FixedOffset>>,

    pub transaction_channel: TransactionChannelSummary,
    pub account_category: AccountCategorySummary,
}

#[derive(Debug, FromQueryResult, Clone)]
pub struct TransactionLimitFlat {
    pub id: i64,
    pub institution_id: i64,
    pub customer_type: CustomerType,
    pub limit_type: TransactionLimitsLimitType,
    pub max_amount: Option<i64>,
    pub max_count: Option<i32>,
    pub currency: Option<Value>,
    pub is_active: Option<bool>,
    pub effective_from: Option<DateTime<FixedOffset>>,
    pub effective_to: Option<DateTime<FixedOffset>>,
    pub kyc_tier: Option<Value>,
    pub created_at: Option<DateTime<FixedOffset>>,
    pub updated_at: Option<DateTime<FixedOffset>>,

    pub transaction_channel_id: i64,
    pub channel_institution_id: i64,
    pub channel_name: Option<String>,
    pub channel_code: Option<String>,
    pub requires_maker_checker: Option<bool>,
    pub metadata: Option<Value>,

    pub account_category_id: i64,
    pub category_name: Option<String>,
    pub category_type: Option<String>,
    pub category_description: Option<String>,
    pub category_is_active: Option<bool>,
}

impl From<TransactionLimitFlat> for TransactionLimitRow {
    fn from(flat: TransactionLimitFlat) -> Self {
        Self {
            id: flat.id.to_string(),
            institution_id: flat.institution_id.to_string(),
            customer_type: flat.customer_type,
            limit_type: flat.limit_type,
            max_amount: flat.max_amount,
            max_count: flat.max_count,
            currency: flat.currency,
            is_active: flat.is_active,
            effective_from: flat.effective_from,
            effective_to: flat.effective_to,
            kyc_tier: flat.kyc_tier,
            created_at: flat.created_at,
            updated_at: flat.updated_at,

            transaction_channel: TransactionChannelSummary {
                id: flat.transaction_channel_id.to_string(),
                institution_id: flat.channel_institution_id.to_string(),
                channel_name: flat.channel_name,
                channel_code: flat.channel_code,
                requires_maker_checker: flat.requires_maker_checker,
                metadata: flat.metadata,
            },

            account_category: AccountCategorySummary {
                id: flat.account_category_id.to_string(),
                name: flat.category_name,
                category_type: flat.category_type,
                description: flat.category_description,
                is_active: flat.category_is_active,
            },
        }
    }
}
