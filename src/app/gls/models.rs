use chrono::{DateTime, NaiveDate, Utc};
use entity::sea_orm_active_enums::{LedgerLockType, TransactionType};

#[derive(Debug, Clone)]
pub struct AddGlRecordModel {
    pub institution_id: i64,
    pub transaction_id: i64,
    pub cus_account_id: i64,
    pub reference: String,
    pub entry_type: TransactionType,
    pub amount: i64,
    pub debit_account: i64,
    pub credit_account: i64,
    pub posted_by: i64,
}

#[derive(Debug, Clone)]
pub struct GlAccountInfo {
    pub debit_delta: i64,
    pub credit_delta: i64,
    pub gl_account_id: i64,
}

#[derive(Debug, Clone)]
pub struct AddGlDailyBalance {
    pub institution_id: i64,
    pub gl_infos: Vec<GlAccountInfo>, //GL Account ID
    pub balance_date: NaiveDate,
    pub count_delta: i32,
}

#[derive(Debug, Clone)]
pub struct CloseGlDailyBalance {
    pub institution_id: i64,
    pub gl_account_id: i64, //GL Account ID
    pub balance_date: NaiveDate,
    pub closing_balance: i64,
}

#[derive(Debug, Clone)]
pub struct AddLedgerEntry {
    pub institution_id: i64,
    pub transaction_id: i64,
    pub account_id: Option<i64>,
    pub gl_account_id: Option<i64>,
    pub entry_type: TransactionType,
    pub amount: i64,
    pub currency_code: String,
    pub description: String,
    pub reference: String,
}

#[derive(Debug, Clone)]
pub struct AddGlPostings {
    pub institution_id: i64,
    pub transaction_id: i64,
    pub ref_number: String,
    pub value_date: NaiveDate,
    pub posting_date: DateTime<Utc>,
    pub debit_account_id: i64,
    pub debit_amount: i64,
    pub credit_account_id: i64,
    pub credit_amount: i64,
    pub narration: String,
    pub is_reversed: bool,
    pub posted_by: i64,
}

#[derive(Debug, Clone)]
pub struct CreateLedgerLockPeriod {
    pub institution_id: i64,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub lock_type: LedgerLockType,
    pub locked_by: i64,
}
