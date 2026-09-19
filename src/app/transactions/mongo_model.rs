use entity::sea_orm_active_enums::{TransactionCategoryType, TransactionStatus, TransactionType};
use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TrigStatus {
    Hold,
    Reject,
    Auth,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DepositTransactionMongoModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub core_id: Option<i64>,

    pub institution_id: i64,
    pub trans_channel_id: i64,
    pub transaction_type: TransactionType, // 'DEBIT' | 'CREDIT'
    pub transaction_category: TransactionCategoryType, // 'CASH_DEPOSIT' | ...
    pub status: TransactionStatus,         // 'PENDING' | ...
    pub trig_status: TrigStatus,           // HOLD | REJECT | AUTH

    pub reference: String,
    pub transaction_group_id: uuid::Uuid,

    pub amount: i64,
    pub currency: mongodb::bson::Bson,
    pub fee_amount: Option<i64>,
    pub vat_amount: Option<i64>,
    pub total_amount: Option<i64>,

    pub ip_address: Option<String>,
    pub posted_at: DateTime,
    pub created_by: i64,
    pub approved_by: Option<i64>,
    pub requires_approval: bool,

    pub description: Option<String>,
    pub credit_account_id: i64,
    pub credit_customer_id: i64,
    pub drawer_id: i64,

    #[serde(default = "def_date_time", skip_serializing)]
    pub created_at: DateTime,
    #[serde(default = "def_date_time", skip_serializing)]
    pub updated_at: DateTime,
}

fn def_date_time() -> DateTime {
    DateTime::now()
}
