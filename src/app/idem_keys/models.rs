use entity::sea_orm_active_enums::{IdempotencyChannel, IdempotencyOperation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitIdemModel {
    pub institution_id: i64,
    pub idem_key: String,
    pub request_method: String,
    pub request_path: String,
    pub channel: IdempotencyChannel,
    pub operation: IdempotencyOperation,
}

#[derive(Debug, Clone)]
pub struct IdemModel {
    pub institution_id: i64,
    pub customer_id: Option<i64>,
    pub account_id: Option<i64>,
    pub transaction_id: Option<i64>,
    pub transaction_group_id: Option<uuid::Uuid>,
    pub idem_key: String,
    pub operation: IdempotencyOperation,
    pub request_method: String,
    pub request_path: String,
    pub request_hash: String,
    pub channel: Option<IdempotencyChannel>,
}
