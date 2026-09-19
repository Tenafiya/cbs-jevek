use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AmlAction {
    Log,
    Freeze,
    Escalate,
    Hold,
    Reject,
    Alert,
    Investigate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EntityType {
    Account,
    Transaction,
    Customer,
    SendingAccount,
    ReceivingAccount,
    Action,
}

#[derive(Debug, Clone)]
pub struct AmlTrigModel {
    pub action_id: i64,
    pub entity: EntityType,
    pub entity_id: i64,
    pub action: AmlAction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmlActionTriggerModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub action_id: i64,

    pub entity: EntityType,
    pub entity_id: i64,

    pub action: AmlAction,

    #[serde(default = "def_date_time", skip_serializing)]
    pub created_at: DateTime,
    #[serde(default = "def_date_time", skip_serializing)]
    pub updated_at: DateTime,
}

fn def_date_time() -> DateTime {
    DateTime::now()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AmlTrigger {
    pub _id: ObjectId,
    pub action_id: i64,
    pub entity: EntityType,
    pub entity_id: i64,
    pub action: AmlAction,
}

impl From<AmlActionTriggerModel> for AmlTrigger {
    fn from(model: AmlActionTriggerModel) -> Self {
        Self {
            _id: model.id.unwrap_or(ObjectId::new()),
            action_id: model.action_id,
            entity: model.entity,
            entity_id: model.entity_id,
            action: model.action,
        }
    }
}
