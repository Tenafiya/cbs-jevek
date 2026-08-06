use chrono::{DateTime, FixedOffset};
use entity::sea_orm_active_enums::{CustomerType, TransactionLimitsLimitType};
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    pub metadata: Option<Value>
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
    pub effective_to: Option<DateTime<FixedOffset>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionCheckerRow {
    pub channel: TransactionChannelSummary,
    pub limit: Option<TransactionLimitSummary>
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