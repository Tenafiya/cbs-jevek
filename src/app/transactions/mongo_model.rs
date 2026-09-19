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

impl From<TrigStatus> for mongodb::bson::Bson {
    fn from(status: TrigStatus) -> Self {
        mongodb::bson::Bson::String(match status {
            TrigStatus::Hold => "HOLD".to_string(),
            TrigStatus::Reject => "REJECT".to_string(),
            TrigStatus::Auth => "AUTH".to_string(),
        })
    }
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MongoDepositTransaction {
    pub _id: ObjectId,
    pub core_id: Option<i64>,
    pub institution_id: i64,
    pub trans_channel_id: i64,
    pub transaction_type: TransactionType,
    pub transaction_category: TransactionCategoryType,
    pub status: TransactionStatus,
    pub trig_status: TrigStatus,
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
}

impl From<DepositTransactionMongoModel> for MongoDepositTransaction {
    fn from(model: DepositTransactionMongoModel) -> Self {
        Self {
            _id: model.id.unwrap_or(ObjectId::new()),
            core_id: model.core_id,
            institution_id: model.institution_id,
            trans_channel_id: model.trans_channel_id,
            transaction_type: model.transaction_type,
            transaction_category: model.transaction_category,
            status: model.status,
            trig_status: model.trig_status,
            reference: model.reference,
            transaction_group_id: model.transaction_group_id,
            amount: model.amount,
            currency: model.currency,
            fee_amount: model.fee_amount,
            vat_amount: model.vat_amount,
            total_amount: model.total_amount,
            ip_address: model.ip_address,
            posted_at: model.posted_at,
            created_by: model.created_by,
            approved_by: model.approved_by,
            requires_approval: model.requires_approval,
            description: model.description,
            credit_account_id: model.credit_account_id,
            credit_customer_id: model.credit_customer_id,
            drawer_id: model.drawer_id,
        }
    }
}
