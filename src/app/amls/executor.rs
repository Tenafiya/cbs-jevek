use crate::app::amls::{
    models::{AmlModel, ConditionParams},
    services,
    util::{ConditionField, ConditionFieldClassify},
};
use entity::sea_orm_active_enums::{CustomerType, TransactionCategoryType, TransactionType};
use sea_orm::{DatabaseTransaction, prelude::Decimal};
use serde::Serialize;
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

#[derive(Debug, Clone)]
pub struct AmlEvaluationResult {
    pub context_name: String,
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

#[derive(Debug, Clone, Serialize)]
pub struct TransactionAmlContext {
    pub id: i64,
    pub amount: i64,
    pub transaction_type: TransactionType,
    pub group_id: uuid::Uuid,
    pub channel_id: i64,
    pub category: TransactionCategoryType,
}

#[derive(Debug, Clone, Serialize)]
pub struct CustomerAmlContext {
    pub id: i64,
    pub customer_type: CustomerType,
}

#[derive(Debug, Clone, Serialize)]
pub struct AccountAmlContext {
    pub id: i64,
    pub account_type_id: i64,
    pub balance: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct DepositAmlContext {
    pub transaction: TransactionAmlContext,
    pub account: AccountAmlContext,
    pub customer: CustomerAmlContext,
}

#[derive(Debug, Clone, Serialize)]
pub enum AmlContext {
    Deposit(DepositAmlContext),
}

pub async fn evaluate(
    trn: &DatabaseTransaction,
    context: &AmlContext,
    model: &AmlModel,
) -> Result<AmlEvaluationResult, AmlError> {
    let model = model.clone();

    let (transaction, customers, accounts, ctx_name) = match context {
        AmlContext::Deposit(deposit) => (
            &deposit.transaction,
            AmlContacts::Single(&deposit.customer),
            AmlAccounts::Single(&deposit.account),
            "deposit",
        ),
    };

    // 1. Load rules (using transaction, customer, accounts as needed)
    let rule_models = services::fetch_execution_rules(model.institution_id, model.stage, trn)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch execution rules: {:?}", e);
            AmlError::RuleFetchError
        })?;

    // 2. Evaluate each rule

    // 3. Record execution

    // 4. Create alerts/actions

    // 5. Return result
    Ok(AmlEvaluationResult {
        context_name: ctx_name.to_string(),
        matched_rules: vec![],
        alerts_created: vec![],
        should_block: false,
        should_hold: false,
        risk_score: Decimal::new(0, 0),
    })
}

fn evaluate_condition_params_condition(
    params: &ConditionParams,
    context: &AmlContext,
) -> Result<bool, AmlError> {
    // Extract the field value from the context
    let field_value = extract_field_value(&params.field, context)?;

    // Evaluate based on operator
    match params.operator {
        // ConditionOperator::Eq => Ok(field_value == params.value),
        // ConditionOperator::Ne => Ok(field_value != params.value),
        // ConditionOperator::Gt => compare_greater_than(&field_value, &params.value),
        // ConditionOperator::Gte => compare_greater_than_or_equal(&field_value, &params.value),
        // ConditionOperator::Lt => compare_less_than(&field_value, &params.value),
        // ConditionOperator::Lte => compare_less_than_or_equal(&field_value, &params.value),
        // ConditionOperator::Contains => contains_value(&field_value, &params.value),
        // ConditionOperator::StartsWith => starts_with_value(&field_value, &params.value),
        // ConditionOperator::In => value_in_list(&field_value, &params.value),
        // ConditionOperator::NotIn => Ok(!value_in_list(&field_value, &params.value)?),
        _ => {
            tracing::error!("Unsupported operator: {:?}", params.operator);
            Err(AmlError::UnsupportedOperator)
        }
    }
}

fn extract_field_value(field: &ConditionField, context: &AmlContext) -> Result<Value, AmlError> {
    let current_value = match field.classify() {
        ConditionFieldClassify::Customer => get_customer_value(context)?,
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
