use chrono::{DateTime, FixedOffset, NaiveDate};
use entity::sea_orm_active_enums::{
    AmlAlertsAlertType, AmlCasesPriority, AmlEntityType, AmlRiskLevelEnum, AmlRuleActions,
    AmlRulesActionOnTrigger, AmlRulesRuleType, AmlWatchlistsListType,
};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct AmlRulesModel {
    pub institution_id: i64,
    pub rule_name: String,
    pub rule_type: AmlRulesRuleType,
    pub condition_logic: Value,
    pub trigger_action: AmlRulesActionOnTrigger,
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
    pub performedby: i64,
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
