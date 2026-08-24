use chrono::{DateTime, FixedOffset};
use entity::sea_orm_active_enums::{
    CustomerType, TransactionCategoryType, TransactionDisputeStatus, TransactionDisputeType,
    TransactionLimitsLimitType, TransactionPriority, TransactionReversalStatus,
    TransactionReversalTypes, TransactionStatus, TransactionType,
};
use sea_orm::prelude::Decimal;
use serde::Deserialize;
use serde_json::Value;
use validator::Validate;

use crate::utils::{
    models::{CashParams, ChequeParams, CurrencyParams, DateStruct},
    validators::{validate_income, validate_snowflake},
};

// ===========================================
// Models
// ===========================================

#[derive(Debug, Clone)]
pub struct AddTransactionLimitModel {
    pub institution_id: i64,
    pub trans_channel_id: i64,
    pub customer_type: CustomerType,
    pub acc_category_id: i64,
    pub limit_type: TransactionLimitsLimitType,
    pub max_amount: Option<i64>,
    pub max_count: Option<i32>,
    pub effective_from: Option<DateTime<FixedOffset>>,
    pub effective_to: Option<DateTime<FixedOffset>>,
    pub currency: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct AddTransactionChannelModel {
    pub institution_id: i64,
    pub channel_name: Option<String>,
    pub channel_code: Option<String>,
    pub description: Option<String>,
    pub requires_maker_checker: bool,
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct CoreTransactionModel {
    pub institution_id: i64,
    pub trans_channel_id: i64,
    pub transaction_type: TransactionType, //'DEBIT', 'CREDIT'
    pub transaction_category: TransactionCategoryType, //'CASH_DEPOSIT', 'CASH_WITHDRAWAL', 'TRANSFER', 'LOAN_DISBURSEMENT', 'LOAN_REPAYMENT'
    pub status: TransactionStatus, //'PENDING', 'COMPLETED', 'FAILED', 'REVERSED', 'DISPUTED', 'CANCELLED'
    pub reference: String,
    pub transaction_group_id: uuid::Uuid,
    pub amount: i64,
    pub currency: Value,
    pub total_amount: Option<i64>,
    pub ip_address: Option<String>,
    pub approved_at: Option<DateTime<FixedOffset>>,
    pub created_by: i64,
    pub approved_by: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct AddDepositModel {
    pub core: CoreTransactionModel,
    pub description: Option<String>,
    pub credit_account_id: i64,
    pub credit_customer_id: i64,
}

#[derive(Debug, Clone)]
pub struct AddWithdrawModel {
    pub core: CoreTransactionModel,
    pub description: Option<String>,
    pub debit_account_id: i64,
    pub debit_customer_id: i64,
}

#[derive(Debug, Clone)]
pub struct AddTransferModel {
    pub core: CoreTransactionModel,
    pub debit_account_id: i64,
    pub credit_account_id: i64,
    pub debit_customer_id: i64,
    pub credit_customer_id: i64,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct AddReversalModel {
    pub parent_transaction_id: i64,
    pub reversal_reason: String,
    pub core: CoreTransactionModel,
}

#[derive(Debug, Clone)]
pub struct AddReversalDetailsModel {
    pub original_transaction_id: i64,
    pub reversal_transaction_id: Option<i64>,
    pub institution_id: i64,
    pub reversal_type: TransactionReversalTypes,
    pub reason: String,
    pub amount: i64,
    pub status: TransactionReversalStatus,
    pub requested_at: DateTime<FixedOffset>,
    pub approved_at: DateTime<FixedOffset>,
    pub checker_workflow_id: Option<i64>,
    pub requested_by: i64,
    pub approved_by: i64,
}

#[derive(Debug, Clone)]
pub struct AddTransactionDisputeModel {
    pub institution_id: i64,
    pub transaction_id: i64,
    pub customer_id: i64,
    pub dispute_type: TransactionDisputeType,
    pub description: String,
    pub amount_disputed: i64,
    pub status: TransactionDisputeStatus,
    pub priority: TransactionPriority,
    pub resolution: String,
    pub refund_amount: i64,
}

//=================================================================
// Params
//=================================================================
#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct AddTransChannelParams {
    #[validate(length(min = 1, max = 255))]
    #[serde(rename = "channelName")]
    pub channel_name: String,

    #[validate(length(min = 1, max = 255))]
    pub description: Option<String>,

    #[serde(rename = "requiresApproval")]
    pub requires_approval: bool,

    pub metadata: Option<Value>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct AddTransLimitParams {
    #[validate(custom(function = "validate_snowflake"))]
    #[serde(rename = "transactionChannelId")]
    pub trans_channel_id: String,

    #[serde(rename = "customerType")]
    pub customer_type: CustomerType,

    #[validate(nested)]
    pub currency: Option<CurrencyParams>,

    #[validate(custom(function = "validate_snowflake"))]
    #[serde(rename = "accountCategoryId")]
    pub acc_category_id: String,

    #[serde(rename = "limitType")]
    pub limit_type: TransactionLimitsLimitType,

    #[serde(rename = "maxAmount")]
    pub max_amount: Option<Decimal>,

    #[serde(rename = "maxCount")]
    pub max_count: Option<i32>,

    #[validate(nested)]
    #[serde(rename = "effectiveDates")]
    pub effective_dates: Option<DateStruct>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct AddDepositParams {
    #[validate(custom(function = "validate_snowflake"))]
    #[serde(rename = "accountId")]
    pub account_id: String,

    #[validate(custom(function = "validate_snowflake"))]
    #[serde(rename = "transactionChannelId")]
    pub trans_channel_id: String,

    #[validate(custom(function = "validate_snowflake"))]
    #[serde(rename = "customerId")]
    pub customer_id: String,

    #[validate(custom(function = "validate_income"))]
    amount: Decimal,

    #[validate(nested)]
    pub currency: CurrencyParams,

    #[validate(nested)]
    #[serde(rename = "cashBreakdown")]
    pub cash_breakdown: Option<Vec<CashParams>>,

    #[validate(nested)]
    pub cheques: Option<Vec<ChequeParams>>,

    #[validate(length(min = 1, max = 255))]
    pub narration: Option<String>,
}
