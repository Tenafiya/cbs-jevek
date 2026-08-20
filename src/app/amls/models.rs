use chrono::{DateTime, FixedOffset, NaiveDate};
use entity::sea_orm_active_enums::{
    AmlAlertsAlertType, AmlCasesPriority, AmlEntityType, AmlRiskLevelEnum, AmlRuleActions,
    AmlRulesActionOnTrigger, AmlRulesExecutionStage, AmlRulesRuleType, AmlWatchlistsListType,
};
use migration::prelude::rust_decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

use crate::utils::{models::DateStruct, validators::validate_snowflake};

//=================================================================
// Models
//=================================================================
#[derive(Debug, Clone)]
pub struct AmlRulesModel {
    pub institution_id: i64,
    pub rule_name: String,
    pub rule_type: AmlRulesRuleType,
    pub condition_logic: Value,
    pub trigger_action: AmlRulesActionOnTrigger,
    pub execution_stage: AmlRulesExecutionStage,
    pub desc: Option<String>,
    pub priority: Option<i32>,
    pub version: Option<i32>,
    pub effective_from: Option<DateTime<FixedOffset>>,
    pub effective_to: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone)]
pub struct AmlExecutionModel {
    pub institution_id: i64,
    pub rule_id: i64,
}

#[derive(Debug, Clone)]
pub struct AmlAlertsModel {
    pub institution_id: i64,
    pub rule_id: i64,
    pub alert_type: AmlAlertsAlertType,
    pub risk_level: AmlRiskLevelEnum,
    pub alert_details: Value,
    pub risk_breakdown: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct AmlCasesModel {
    pub institution_id: i64,
    pub case_number: String,
    pub title: String,
    pub priority: AmlCasesPriority,
    pub investigator: Option<i64>,
    pub desc: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AmlCaseNotesModel {
    pub case_id: i64,
    pub investigator_id: i64,
    pub note: String,
}

#[derive(Debug, Clone)]
pub struct AmlActionsModel {
    pub institution_id: i64,
    pub case_id: i64,
    pub alert_id: i64,
    pub action_type: AmlRuleActions,
    pub performed_by: i64,
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct AmlWatchListsModel {
    pub institution_id: i64,
    pub list_type: AmlWatchlistsListType,
    pub external_references: Option<String>,
    pub full_name: String,
    pub country: Option<String>,
    pub date_of_birth: NaiveDate,
    pub metadata: Value,
}

#[derive(Debug, Clone)]
pub struct AmlWhiteListModel {
    pub institution_id: i64,
    pub entity_type: AmlEntityType,
    pub entity_id: Option<i64>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AmlBlackListModel {
    pub institution_id: i64,
    pub entity_type: AmlEntityType,
    pub entity_id: Option<i64>,
    pub reason: Option<String>,
    pub severity: AmlRiskLevelEnum,
}

//=================================================================
// Params
//=================================================================
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ConditionValue {
    String(String),
    Integer(i64),
    Decimal(rust_decimal::Decimal),
    Boolean(bool),
    Strings(Vec<String>),
    Integers(Vec<i64>),
}

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct ConditionParams {
    pub field: String,
    pub operator: String,
    pub value: ConditionValue,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct ConditionGroup {
    pub operator: String,
    pub conditions: Vec<ConditionParams>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct CreateAmlRulesParams {
    #[validate(length(min = 1, max = 255))]
    #[serde(rename = "ruleName")]
    pub rule_name: String,

    #[serde(rename = "ruleType")]
    pub rule_type: AmlRulesRuleType,

    #[serde(rename = "conditionLogic")]
    pub condition_logic: Vec<ConditionGroup>,

    #[serde(rename = "executionStage")]
    pub execution_stage: AmlRulesExecutionStage,

    #[serde(rename = "triggerAction")]
    pub trigger_action: AmlRulesActionOnTrigger,

    #[validate(length(max = 255))]
    pub description: Option<String>,

    #[validate(range(min = 1, max = 10))]
    pub priority: Option<i32>,

    #[validate(range(min = 1))]
    pub version: Option<i32>,

    #[validate(nested)]
    #[serde(rename = "effectiveDates")]
    pub effective_dates: Option<DateStruct>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct CreateAmlCaseNodes {
    #[validate(custom(function = "validate_snowflake"))]
    #[serde(rename = "caseId")]
    pub case_id: String,

    #[validate(length(min = 1, max = 255))]
    pub notes: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct CreateAmlActionParams {
    #[validate(custom(function = "validate_snowflake"))]
    #[serde(rename = "caseId")]
    pub case_id: String,

    #[validate(custom(function = "validate_snowflake"))]
    #[serde(rename = "alertId")]
    pub alert_id: String,

    #[serde(rename = "actionType")]
    pub action_type: AmlRuleActions,

    pub metadata: Option<Value>,
}
