use crate::{
    AppState,
    app::amls::{
        action_processes::{freeze_account, log_action},
        mapper::AmlRule,
        models::{
            AmlActionsModel, AmlAlertsModel, AmlCasesModel, AmlExecutionModel, AmlModel,
            ConditionGroup, ConditionParams,
        },
        services,
        util::{
            ConditionField, ConditionFieldClassify, ConditionOperator, ConditionValue,
            LogicalOperator,
        },
    },
};
use actix_web::web;
use chrono::{DateTime, Utc};
use entity::sea_orm_active_enums::{
    AmlAlertsAlertType, AmlCasesPriority, AmlRiskLevelEnum, AmlRuleActions,
    AmlRulesActionOnTrigger, AmlRulesPriority, AmlRulesRuleType, CustomerType,
    TransactionCategoryType, TransactionType,
};
use rust_decimal::Decimal;
use sea_orm::{InsertResult, TransactionTrait};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Instant;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum AmlError {
    #[error("Invalid resource type")]
    InvalidResourceType,
    #[error("IO error")]
    IoError,
    #[error("Parser error")]
    ParserError,
    #[error("Method execution error")]
    MethodExecutionError,
    #[error("Rule fetch error")]
    RuleFetchError,
    #[error("Unsupported operator")]
    UnsupportedOperator,
}

#[derive(Clone, Debug)]
pub struct CheckerParams {
    pub params: Value,
    pub field: ConditionField,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveActionMeta {
    pub rule_action_type: AmlRulesActionOnTrigger,
    pub context: AmlContext,
    pub timestamp: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuleEvaluation {
    pub rule_id: i64,
    pub is_passed: bool,
    pub evaluated_at: DateTime<Utc>,
    pub context: AmlContext,
}

#[derive(Debug, Clone)]
pub struct AmlEvaluationResult {
    pub matched_rules: Vec<RuleEvaluation>,
    pub alerts_created: Vec<i64>,
    pub actions_created: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmlRiskScoreContext {
    pub rule_priority: AmlRulesPriority,
    pub rule_type: AmlRulesRuleType,
    pub factors: AmlRiskFactors,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmlRiskFactors {
    pub amount: i64,
    pub customer_type: String,
    pub risk_country: bool,
}

pub enum AmlAccounts<'a> {
    Single(&'a AccountAmlContext),
    Pair(&'a AccountAmlContext, &'a AccountAmlContext),
}

pub enum AmlContacts<'a> {
    Single(&'a CustomerAmlContext),
    Pair(&'a CustomerAmlContext, &'a CustomerAmlContext),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionAmlContext {
    pub id: i64,
    pub amount: i64,
    pub transaction_type: TransactionType,
    pub group_id: uuid::Uuid,
    pub channel_id: i64,
    pub category: TransactionCategoryType,
    pub currency_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAmlContext {
    pub id: i64,
    pub customer_type: CustomerType,
    pub institution_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountAmlContext {
    pub id: i64,
    pub account_type_id: i64,
    pub balance: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositAmlContext {
    pub transaction: TransactionAmlContext,
    pub account: AccountAmlContext,
    pub customer: CustomerAmlContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AmlContext {
    Deposit(DepositAmlContext),
}

impl AmlContext {
    pub fn type_name(&self) -> &'static str {
        match self {
            AmlContext::Deposit(_) => "deposit",
        }
    }
}

pub async fn evaluate(
    model: &AmlModel,
    context: &AmlContext,
    state: &web::Data<AppState>,
) -> Result<AmlEvaluationResult, AmlError> {
    let model = model.clone();
    let mut matched_rules: Vec<RuleEvaluation> = Vec::new();
    let mut created_alerts: Vec<i64> = Vec::new();
    let mut created_actions: Vec<i64> = Vec::new();
    let mut risk_score = Decimal::ZERO;

    // 1. Load rules
    let rule_models = services::fetch_execution_rules(model.institution_id, model.stage, state)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch execution rules: {:?}", e);
            AmlError::RuleFetchError
        })?;

    // 2. Evaluate each rule
    for rule_model in rule_models {
        let start = Instant::now();

        let rule_condition = rule_model.condition_logic.clone();

        let condition_groups: Vec<ConditionGroup> = serde_json::from_value(rule_condition)
            .map_err(|e| {
                tracing::error!("Failed to parse from rule model: {}", e);
                AmlError::InvalidResourceType
            })?;

        for condition_group in condition_groups {
            let result = evaluate_group(&condition_group, context)?;

            let evaluation = RuleEvaluation {
                rule_id: rule_model.id,
                is_passed: result,
                evaluated_at: chrono::Utc::now(),
                context: context.clone(),
            };

            matched_rules.push(evaluation.clone());

            if !result {
                let (alert, score) =
                    create_aml_alert(state, model.institution_id, result, &rule_model, &context)
                        .await?;

                risk_score = score;

                let action = create_aml_actions(
                    state,
                    model.institution_id,
                    &rule_model,
                    alert.last_insert_id,
                    context,
                    risk_score,
                )
                .await?;

                created_alerts.push(alert.last_insert_id);
                created_actions.push(action.last_insert_id);
            };

            // 3. Record execution
            // Write key to nats jetstream to save this async
            let execution = AmlExecutionModel {
                institution_id: model.institution_id,
                rule_id: rule_model.id,
                is_matched: result,
                risk_score,
                evaluation: serde_json::to_value(evaluation).map_err(|e| {
                    tracing::error!("Failed to parse evaluation: {}", e);
                    AmlError::ParserError
                })?,
                execution_ms: start.elapsed().as_millis() as i32,
            };

            // use nats here
            let message = serde_json::to_string(&execution).map_err(|e| {
                tracing::error!("Failed to parse evaluation: {}", e);
                AmlError::ParserError
            })?;

            state
                .streamer
                .publish_to_stream("amls.execution.new", message)
                .await
                .map_err(|e| {
                    tracing::error!("Failed to parse evaluation: {}", e);
                    AmlError::MethodExecutionError
                })?;
        }
    }

    // 4. Write to nats jetstream to execute actions
    let action_messages = serde_json::to_string(&created_actions).map_err(|e| {
        tracing::error!("Failed to parse created actions: {}", e);
        AmlError::ParserError
    })?;

    state
        .streamer
        .publish_to_stream("amls.actions.new", action_messages)
        .await
        .map_err(|e| {
            tracing::error!("Failed to parse evaluation: {}", e);
            AmlError::MethodExecutionError
        })?;

    // 5. Return result
    Ok(AmlEvaluationResult {
        matched_rules: matched_rules,
        alerts_created: created_alerts,
        actions_created: created_actions,
    })
}

fn evaluate_group(rule_group: &ConditionGroup, context: &AmlContext) -> Result<bool, AmlError> {
    match rule_group.operator {
        LogicalOperator::And => {
            for condition in &rule_group.conditions {
                if !evaluate_condition_params_condition(&condition, context)? {
                    return Ok(false);
                };
            }

            Ok(true)
        }
        LogicalOperator::Or => {
            for condition in &rule_group.conditions {
                if evaluate_condition_params_condition(&condition, context)? {
                    return Ok(true);
                }
            }

            Ok(false)
        }
    }
}

fn evaluate_condition_params_condition(
    aml_rule_condition: &ConditionParams,
    context: &AmlContext,
) -> Result<bool, AmlError> {
    // Extract the field value from the context
    let field_value = extract_field_value(&aml_rule_condition.field, context)?;

    let (cond, val) = field_value;

    let checker_params = CheckerParams {
        params: val,
        field: cond,
    };

    // Evaluate based on operator
    match aml_rule_condition.operator {
        ConditionOperator::Eq => evaluate_eq_condition(aml_rule_condition, &checker_params),
        ConditionOperator::Ne => {
            let result = evaluate_eq_condition(aml_rule_condition, &checker_params)?;
            Ok(!result)
        }
        ConditionOperator::Gt => evaluate_gt_condition(aml_rule_condition, &checker_params),
        ConditionOperator::Gte => evaluate_gte_condition(aml_rule_condition, &checker_params),
        ConditionOperator::Lt => evaluate_lt_condition(aml_rule_condition, &checker_params),
        ConditionOperator::Lte => evaluate_lte_condition(aml_rule_condition, &checker_params),
        // ConditionOperator::Contains => contains_value(&field_value, &params.value),
        // ConditionOperator::StartsWith => starts_with_value(&field_value, &params.value),
        ConditionOperator::In => evaluate_in_condition(aml_rule_condition, &checker_params),
        ConditionOperator::NotIn => {
            let result = evaluate_in_condition(aml_rule_condition, &checker_params)?;
            Ok(!result)
        }
        _ => {
            tracing::error!("Unsupported operator: {:?}", aml_rule_condition.operator);
            Err(AmlError::UnsupportedOperator)
        }
    }
}

fn extract_field_value(
    field: &ConditionField,
    context: &AmlContext,
) -> Result<(ConditionField, Value), AmlError> {
    let current_value = match field.classify() {
        ConditionFieldClassify::Customer => (*field, get_customer_value(context)?),
        ConditionFieldClassify::Transaction => (*field, get_transaction_value(context)?),
        ConditionFieldClassify::Account => (*field, get_account_value(context)?),
        _ => return Err(AmlError::InvalidResourceType),
    };

    Ok(current_value)
}

pub fn get_customer_value(context: &AmlContext) -> Result<Value, AmlError> {
    match context {
        AmlContext::Deposit(ctx) => serde_json::to_value(&ctx.customer),
    }
    .map_err(|e| {
        tracing::error!("Failed to serialize customer value: {}", e);
        AmlError::IoError
    })
}

pub fn get_transaction_value(context: &AmlContext) -> Result<Value, AmlError> {
    match context {
        AmlContext::Deposit(ctx) => serde_json::to_value(&ctx.transaction),
    }
    .map_err(|e| {
        tracing::error!("Failed to serialize transaction value: {}", e);
        AmlError::IoError
    })
}

pub fn get_account_value(context: &AmlContext) -> Result<Value, AmlError> {
    match context {
        AmlContext::Deposit(ctx) => serde_json::to_value(&ctx.account),
    }
    .map_err(|e| {
        tracing::error!("Failed to serialize account value: {}", e);
        AmlError::IoError
    })
}

fn calculate_risk_score(rule: &AmlRule, context: &AmlContext) -> Decimal {
    let base_score = match rule.priority {
        AmlRulesPriority::Critical => Decimal::new(90, 2), //0.90
        AmlRulesPriority::High => Decimal::new(70, 2),
        AmlRulesPriority::Medium => Decimal::new(50, 2),
        AmlRulesPriority::Low => Decimal::new(30, 2),
    };

    let adjustment = match context {
        AmlContext::Deposit(ctx) => {
            if ctx.transaction.amount > 10000 {
                Decimal::new(5, 2) //0.05
            } else {
                Decimal::ZERO
            }
        } // _ => Decimal::ZERO,
    };

    (base_score + adjustment).min(Decimal::ONE)
}

fn calculate_risk_breakdown(rule: &AmlRule, context: &AmlContext) -> AmlRiskScoreContext {
    let rule = rule.clone();
    AmlRiskScoreContext {
        rule_priority: rule.priority,
        rule_type: rule.rule_type,

        factors: AmlRiskFactors {
            amount: match context {
                AmlContext::Deposit(ctx) => ctx.transaction.amount,
            },

            customer_type: match context {
                AmlContext::Deposit(ctx) => ctx.customer.customer_type.to_string(),
            },

            risk_country: false,
        },

        timestamp: Utc::now(),
    }
}

fn map_priority_to_risk_level(priority: &AmlRulesPriority) -> AmlRiskLevelEnum {
    match priority {
        AmlRulesPriority::Critical => AmlRiskLevelEnum::Critical,
        AmlRulesPriority::High => AmlRiskLevelEnum::High,
        AmlRulesPriority::Medium => AmlRiskLevelEnum::Medium,
        AmlRulesPriority::Low => AmlRiskLevelEnum::Low,
    }
}

fn determine_case_priority(
    rule_priority: &AmlRulesPriority,
    risk_score: Decimal,
) -> AmlCasesPriority {
    if risk_score >= Decimal::new(60, 2) && risk_score < Decimal::new(70, 2) {
        return AmlCasesPriority::Urgent;
    }

    match rule_priority {
        AmlRulesPriority::Critical => AmlCasesPriority::Critical,
        AmlRulesPriority::High => AmlCasesPriority::High,
        AmlRulesPriority::Medium => AmlCasesPriority::Normal,
        AmlRulesPriority::Low => AmlCasesPriority::Low,
    }
}

fn set_alert_type(rule: &AmlRule, context: &AmlContext) -> AmlAlertsAlertType {
    match &rule.rule_type {
        AmlRulesRuleType::TransactionAmount => AmlAlertsAlertType::LargeTransaction,

        AmlRulesRuleType::TransactionVelocity => AmlAlertsAlertType::RapidMovementOfFunds,

        AmlRulesRuleType::TransactionPattern => AmlAlertsAlertType::UnusualTransactionPattern,

        AmlRulesRuleType::Structuring => AmlAlertsAlertType::Structuring,

        AmlRulesRuleType::GeographicRisk => AmlAlertsAlertType::HighRiskCountry,

        AmlRulesRuleType::SanctionsScreening => AmlAlertsAlertType::SanctionsMatch,

        AmlRulesRuleType::PepScreening => AmlAlertsAlertType::PepMatch,

        AmlRulesRuleType::AdverseMediaScreening => AmlAlertsAlertType::AdverseMedia,

        AmlRulesRuleType::CustomerRisk => AmlAlertsAlertType::SuspiciousAccount,

        AmlRulesRuleType::AccountActivity => AmlAlertsAlertType::UnusualTransaction,

        AmlRulesRuleType::BeneficiaryRisk => AmlAlertsAlertType::SuspiciousBeneficiary,

        AmlRulesRuleType::DeviceRisk => AmlAlertsAlertType::FraudSuspected,

        AmlRulesRuleType::ImpossibleTravel => AmlAlertsAlertType::FraudSuspected,

        AmlRulesRuleType::BehaviouralAnomaly => AmlAlertsAlertType::UnusualTransactionPattern,

        AmlRulesRuleType::DormantAccountActivity => AmlAlertsAlertType::DormantAccountActivity,

        AmlRulesRuleType::CashActivity => AmlAlertsAlertType::UnusualCashActivity,

        AmlRulesRuleType::AccountTakeover => AmlAlertsAlertType::AccountTakeover,

        AmlRulesRuleType::MuleAccount => AmlAlertsAlertType::MuleAccount,

        AmlRulesRuleType::FundsCycling => AmlAlertsAlertType::FundsCycling,

        AmlRulesRuleType::RoundTripping => AmlAlertsAlertType::RoundTripping,

        AmlRulesRuleType::CustomRule => match context {
            AmlContext::Deposit(_) => AmlAlertsAlertType::UnusualDeposit,
        },
    }
}

fn map_action_type(action_type: &AmlRulesActionOnTrigger) -> AmlRuleActions {
    match action_type {
        AmlRulesActionOnTrigger::Alert => AmlRuleActions::GenerateAlert,
        AmlRulesActionOnTrigger::BlockTransaction => AmlRuleActions::RejectTransaction,
        AmlRulesActionOnTrigger::Flag => AmlRuleActions::EscalateToInvestigator,
        AmlRulesActionOnTrigger::FreezeAccount => AmlRuleActions::FreezeAccount,
    }
}

async fn create_aml_alert(
    state: &web::Data<AppState>,
    institution_id: i64,
    rule_pass: bool,
    rule: &AmlRule,
    context: &AmlContext,
) -> Result<(InsertResult<entity::aml_alerts::ActiveModel>, Decimal), AmlError> {
    let risk_score = calculate_risk_score(rule, context);
    let risk_breakdown = calculate_risk_breakdown(rule, context);
    let alert_type = set_alert_type(rule, context);

    let customer_value = get_customer_value(context)?;
    let transaction_value = get_transaction_value(context)?;

    let alert_details = serde_json::json!({
        "rule_id": rule.id,
        "rule_name": rule.rule_name,
        "rule_description": rule.rule_description,
        "condition_passed": rule_pass,
        "risk_score": risk_score,
        "trigger_reason": if rule_pass {
            "Suspicious pattern detected"
        } else {
            "Compliance requirement failed"
        },
        "evaluation_details": {
            "timestamp": Utc::now().to_rfc3339(),
            "stage": rule.execution_stage,
        },
        "transaction_details": transaction_value,
        "customer_details": customer_value,
    });

    let breakdown = serde_json::to_value(risk_breakdown).map_err(|e| {
        tracing::error!("Failed to parse breakdown: {}", e);
        AmlError::ParserError
    })?;

    let alert = AmlAlertsModel {
        institution_id,
        rule_id: rule.id,
        alert_type,
        risk_level: map_priority_to_risk_level(&rule.priority),
        alert_details,
        risk_breakdown: Some(breakdown),
    };

    let result = services::save_aml_alerts(&alert, state)
        .await
        .map_err(|e| {
            tracing::error!("Failed to save alert: {}", e);
            AmlError::MethodExecutionError
        })?;

    Ok((result, risk_score))
}

async fn create_aml_actions(
    state: &web::Data<AppState>,
    institution_id: i64,
    rule: &AmlRule,
    alert_id: i64,
    context: &AmlContext,
    risk_score: Decimal,
) -> Result<InsertResult<entity::aml_actions::ActiveModel>, AmlError> {
    let trn = state.pgdb.get_ref().begin().await.map_err(|e| {
        tracing::error!("Failed to start db transaction: {}", e);
        AmlError::MethodExecutionError
    })?;

    let rule = rule.clone();

    let case_model = AmlCasesModel {
        institution_id,
        title: rule.rule_name,
        priority: determine_case_priority(&rule.priority, risk_score),
        investigator: None,
        desc: Some(format!("Aml Cases created for Alert : {}", alert_id)),
    };

    let case = services::save_aml_cases(&case_model, &trn)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create case: {}", e);
            AmlError::MethodExecutionError
        })?;

    let action_meta = SaveActionMeta {
        rule_action_type: rule.action_on_trigger.clone(),
        context: context.clone(),
        timestamp: Utc::now().to_rfc3339(),
    };

    let action = AmlActionsModel {
        institution_id,
        case_id: case.last_insert_id,
        alert_id,
        action_type: map_action_type(&rule.action_on_trigger),
        performed_by: None,
        metadata: Some(serde_json::to_value(action_meta).map_err(|e| {
            tracing::error!("Failed to create case: {}", e);
            AmlError::ParserError
        })?),
    };

    let action = services::save_aml_action(&action, &trn)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create action: {}", e);
            AmlError::MethodExecutionError
        })?;

    trn.commit().await.map_err(|e| {
        tracing::error!("Failed to commit db transaction: {}", e);
        AmlError::MethodExecutionError
    })?;

    Ok(action)
}

pub fn evaluate_eq_condition(
    rule_params: &ConditionParams,
    context_params: &CheckerParams,
) -> Result<bool, AmlError> {
    let context = context_params.clone();

    let result: bool = match context.field {
        ConditionField::CustomerCustomerType => {
            let customer_value: CustomerAmlContext = serde_json::from_value(context.params)
                .map_err(|e| {
                    tracing::error!("Failed to parse customer value: {}", e);
                    AmlError::IoError
                })?;

            match &rule_params.value {
                ConditionValue::String(value) => value == &customer_value.customer_type.to_string(),
                _ => false,
            }
        }
        ConditionField::TransactionAmount => {
            let transaction_value: TransactionAmlContext = serde_json::from_value(context.params)
                .map_err(|e| {
                tracing::error!("Failed to parse transaction value: {}", e);
                AmlError::IoError
            })?;

            match &rule_params.value {
                ConditionValue::Integer(value) => value == &transaction_value.amount,
                _ => false,
            }
        }
        ConditionField::TransactionCurrency => {
            let transaction_value: TransactionAmlContext = serde_json::from_value(context.params)
                .map_err(|e| {
                tracing::error!("Failed to parse transaction value: {}", e);
                AmlError::IoError
            })?;

            match &rule_params.value {
                ConditionValue::String(value) => value == &transaction_value.currency_name,
                _ => false,
            }
        }
        ConditionField::TransactionType => {
            let transaction_value: TransactionAmlContext = serde_json::from_value(context.params)
                .map_err(|e| {
                tracing::error!("Failed to parse transaction value: {}", e);
                AmlError::IoError
            })?;

            match &rule_params.value {
                ConditionValue::String(value) => {
                    value == &transaction_value.transaction_type.to_string()
                }
                _ => false,
            }
        }
        ConditionField::TransactionChannel => {
            let transaction_value: TransactionAmlContext = serde_json::from_value(context.params)
                .map_err(|e| {
                tracing::error!("Failed to parse transaction value: {}", e);
                AmlError::IoError
            })?;

            match &rule_params.value {
                ConditionValue::String(value) => {
                    let val = value.parse::<i64>().map_err(|e| {
                        tracing::error!("Parse Error: {}", e);
                        AmlError::ParserError
                    })?;

                    val == transaction_value.channel_id
                }
                _ => false,
            }
        }
        _ => {
            tracing::error!("Failed to handle Eq for {:?} condition", context.field);
            return Err(AmlError::MethodExecutionError);
        }
    };

    Ok(result)
}

pub fn evaluate_gt_condition(
    rule_params: &ConditionParams,
    context_params: &CheckerParams,
) -> Result<bool, AmlError> {
    let context = context_params.clone();

    let result: bool = match context.field {
        ConditionField::TransactionAmount => {
            let transaction_value: TransactionAmlContext = serde_json::from_value(context.params)
                .map_err(|e| {
                tracing::error!("Failed to parse transaction value: {}", e);
                AmlError::IoError
            })?;

            match &rule_params.value {
                ConditionValue::Integer(value) => value > &transaction_value.amount,
                _ => false,
            }
        }
        _ => {
            tracing::error!("Failed to handle Gt for {:?} condition", context.field);
            return Err(AmlError::MethodExecutionError);
        }
    };

    Ok(result)
}

pub fn evaluate_gte_condition(
    rule_params: &ConditionParams,
    context_params: &CheckerParams,
) -> Result<bool, AmlError> {
    let context = context_params.clone();

    let result: bool = match context.field {
        ConditionField::TransactionAmount => {
            let transaction_value: TransactionAmlContext = serde_json::from_value(context.params)
                .map_err(|e| {
                tracing::error!("Failed to parse transaction value: {}", e);
                AmlError::IoError
            })?;

            match &rule_params.value {
                ConditionValue::Integer(value) => value >= &transaction_value.amount,
                _ => false,
            }
        }
        _ => {
            tracing::error!("Failed to handle Gte for {:?} condition", context.field);
            return Err(AmlError::MethodExecutionError);
        }
    };

    Ok(result)
}

pub fn evaluate_lt_condition(
    rule_params: &ConditionParams,
    context_params: &CheckerParams,
) -> Result<bool, AmlError> {
    let context = context_params.clone();

    let result: bool = match context.field {
        ConditionField::TransactionAmount => {
            let transaction_value: TransactionAmlContext = serde_json::from_value(context.params)
                .map_err(|e| {
                tracing::error!("Failed to parse transaction value: {}", e);
                AmlError::IoError
            })?;

            match &rule_params.value {
                ConditionValue::Integer(value) => value < &transaction_value.amount,
                _ => false,
            }
        }
        _ => {
            tracing::error!("Failed to handle Lt for {:?} condition", context.field);
            return Err(AmlError::MethodExecutionError);
        }
    };

    Ok(result)
}

pub fn evaluate_lte_condition(
    rule_params: &ConditionParams,
    context_params: &CheckerParams,
) -> Result<bool, AmlError> {
    let context = context_params.clone();

    let result: bool = match context.field {
        ConditionField::TransactionAmount => {
            let transaction_value: TransactionAmlContext = serde_json::from_value(context.params)
                .map_err(|e| {
                tracing::error!("Failed to parse transaction value: {}", e);
                AmlError::IoError
            })?;

            match &rule_params.value {
                ConditionValue::Integer(value) => value <= &transaction_value.amount,
                _ => false,
            }
        }
        _ => {
            tracing::error!("Failed to handle Lte for {:?} condition", context.field);
            return Err(AmlError::MethodExecutionError);
        }
    };

    Ok(result)
}

pub fn evaluate_in_condition(
    rule_params: &ConditionParams,
    context_params: &CheckerParams,
) -> Result<bool, AmlError> {
    let context = context_params.clone();

    let result: bool = match context.field {
        ConditionField::TransactionType => {
            let transaction_value: TransactionAmlContext = serde_json::from_value(context.params)
                .map_err(|e| {
                tracing::error!("Failed to parse transaction value: {}", e);
                AmlError::IoError
            })?;

            match &rule_params.value {
                ConditionValue::Strings(values) => {
                    values.contains(&transaction_value.transaction_type.to_string())
                }
                _ => false,
            }
        }
        _ => {
            tracing::error!("Failed to handle Lte for {:?} condition", context.field);
            return Err(AmlError::MethodExecutionError);
        }
    };

    Ok(result)
}

pub async fn process_aml_actions(
    created_actions: String,
    state: &web::Data<AppState>,
) -> Result<(), AmlError> {
    let action_ids: Vec<i64> = serde_json::from_str(&created_actions).map_err(|e| {
        tracing::error!("Failed to parse created actions: {}", e);
        AmlError::ParserError
    })?;

    let actions = services::get_action_list(action_ids, &state)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch list of actions: {}", e);
            AmlError::MethodExecutionError
        })?;

    for action in actions {
        let action_meta: SaveActionMeta =
            serde_json::from_value(action.metadata.clone().ok_or_else(|| {
                tracing::error!("Metadata is missing");
                AmlError::ParserError
            })?)
            .map_err(|e| {
                tracing::error!("Failed to parse evaluation: {}", e);
                AmlError::ParserError
            })?;

        let context: AmlContext = action_meta.context;

        match action.action_type {
            None => {
                tracing::warn!("No action type specified for action: {}", action.id);
            }
            Some(AmlRuleActions::LogOnly) => log_action(&action).await?,
            Some(AmlRuleActions::EscalateToInvestigator) => {}
            Some(AmlRuleActions::FileSarAutomatically) => {}
            Some(AmlRuleActions::FreezeAccount) => freeze_account(&action, &context, state).await?,
            Some(AmlRuleActions::GenerateAlert) => {}
            Some(AmlRuleActions::HoldTransaction) => {}
            Some(AmlRuleActions::RejectTransaction) => {}
            Some(AmlRuleActions::RequireAdditionalAuthentication) => {}
        }
    }

    Ok(())
}
