use crate::app::amls::{
    models::{AmlModel, ConditionGroup, ConditionParams},
    services,
    util::{
        ConditionField, ConditionFieldClassify, ConditionOperator, ConditionValue, LogicalOperator,
    },
};
use chrono::{DateTime, Utc};
use entity::sea_orm_active_enums::{CustomerType, TransactionCategoryType, TransactionType};
use sea_orm::{DatabaseTransaction, prelude::Decimal};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AmlError {
    InvalidOpcode(u8),
    InvalidLength,
    InvalidResourceType,
    InvalidArgumentCount,
    NamespaceLookupFailed(String),
    InvalidDataType,
    IoError,
    ParserError(usize),
    MethodExecutionError,
    RuleExecutionError,
    RuleFetchError,
    UnsupportedOperator,
}

#[derive(Clone, Debug)]
pub struct CheckerParams {
    pub params: Value,
    pub field: ConditionField,
}

#[derive(Clone, Debug)]
pub struct ContextMetadata {
    pub ctx_name: &'static str,
    pub evaluated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct AmlEvaluationResult {
    pub context_names: Vec<String>,
    pub matched_rules: Vec<i64>,
    pub alerts_created: Vec<i64>,
    pub should_block: bool,
    pub should_hold: bool,
    pub risk_score: Decimal,
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
    pub balance: i64,
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
    trn: &DatabaseTransaction,
    context: &AmlContext,
    model: &AmlModel,
) -> Result<AmlEvaluationResult, AmlError> {
    let model = model.clone();
    let mut ctx_names: Vec<String> = Vec::new();

    // 1. Load rules (using transaction, customer, accounts as needed)
    let rule_models = services::fetch_execution_rules(model.institution_id, model.stage, trn)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch execution rules: {:?}", e);
            AmlError::RuleFetchError
        })?;

    // 2. Evaluate each rule
    for rule_model in rule_models {
        let condition_groups: Vec<ConditionGroup> =
            serde_json::from_value(rule_model.condition_logic).map_err(|e| {
                tracing::error!("Failed to parse from rule model: {}", e);
                AmlError::InvalidResourceType
            })?;

        for condition_group in condition_groups {
            let result = evaluate_group(&condition_group, context)?;
        }
    }

    // 3. Record execution

    // 4. Create alerts/actions

    // 5. Return result
    Ok(AmlEvaluationResult {
        context_names: vec![],
        matched_rules: vec![],
        alerts_created: vec![],
        should_block: false,
        should_hold: false,
        risk_score: Decimal::new(0, 0),
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

fn get_context_metadata_value(context: &AmlContext) -> Result<ContextMetadata, AmlError> {
    Ok(ContextMetadata {
        ctx_name: context.type_name(),
        evaluated_at: chrono::Utc::now(),
    })
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

fn get_customer_value(context: &AmlContext) -> Result<Value, AmlError> {
    match context {
        AmlContext::Deposit(ctx) => serde_json::to_value(&ctx.customer),
    }
    .map_err(|e| {
        tracing::error!("Failed to serialize customer value: {}", e);
        AmlError::IoError
    })
}

fn get_transaction_value(context: &AmlContext) -> Result<Value, AmlError> {
    match context {
        AmlContext::Deposit(ctx) => serde_json::to_value(&ctx.transaction),
    }
    .map_err(|e| {
        tracing::error!("Failed to serialize transaction value: {}", e);
        AmlError::IoError
    })
}

fn get_account_value(context: &AmlContext) -> Result<Value, AmlError> {
    match context {
        AmlContext::Deposit(ctx) => serde_json::to_value(&ctx.account),
    }
    .map_err(|e| {
        tracing::error!("Failed to serialize account value: {}", e);
        AmlError::IoError
    })
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
                        AmlError::ParserError(0)
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
