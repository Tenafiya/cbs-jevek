use chrono::{DateTime, FixedOffset};
use entity::sea_orm_active_enums::{
    CustomerType, TransactionCategoryType, TransactionDisputeStatus, TransactionDisputeType,
    TransactionLimitsLimitType, TransactionPriority, TransactionReversalStatus,
    TransactionReversalTypes, TransactionStatus, TransactionType,
};
use serde_json::Value;

// ===========================================
// Models
// ===========================================
#[derive(Debug, Clone)]
pub struct CurrencyModel {
    pub name: String,
    pub symbol: String,
    pub precision: Option<i32>,
}

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
