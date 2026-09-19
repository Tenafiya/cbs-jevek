use actix_web::web;
use entity::sea_orm_active_enums::AccTypeStatus;

use crate::{
    AppState,
    app::{
        accounts,
        amls::{
            executor::{
                AccountAmlContext, AmlContext, AmlError, TransactionAmlContext, get_account_value,
                get_transaction_value,
            },
            mongo_model::{AmlAction, AmlTrigModel, EntityType},
            services,
        },
    },
};

pub async fn log_action(action: &entity::aml_actions::Model) -> Result<(), AmlError> {
    tracing::debug!("Executed action: {:?}", action.id);
    Ok(())
}

pub async fn escalate_action(
    action: &entity::aml_actions::Model,
    _context: &AmlContext,
    state: &web::Data<AppState>,
) -> Result<(), AmlError> {
    let trig_writer = AmlTrigModel {
        action_id: action.id,
        entity: EntityType::Action,
        entity_id: action.id,
        action: AmlAction::Escalate,
    };

    services::write_action_info(&trig_writer, state)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to write action info to mongo");
            AmlError::MethodExecutionError
        })?;

    Ok(())
}

pub async fn freeze_account(
    action: &entity::aml_actions::Model,
    context: &AmlContext,
    state: &web::Data<AppState>,
) -> Result<(), AmlError> {
    let account_val = get_account_value(context)?;

    let account: AccountAmlContext = serde_json::from_value(account_val).map_err(|e| {
        tracing::error!("Failed to parse account: {}", e);
        AmlError::ParserError
    })?;

    let trig_writer = AmlTrigModel {
        action_id: action.id,
        entity: EntityType::Account,
        entity_id: account.id,
        action: AmlAction::Freeze,
    };

    services::write_action_info(&trig_writer, state)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to write action info to mongo");
            AmlError::MethodExecutionError
        })?;

    accounts::services::toggle_account_status(account.id, AccTypeStatus::Frozen, state)
        .await
        .map_err(|e| {
            tracing::error!("Failed to toggle account status: {}", e);
            AmlError::MethodExecutionError
        })?;

    Ok(())
}

pub async fn file_sar(
    _action: &entity::aml_actions::Model,
    _context: &AmlContext,
    _state: &web::Data<AppState>,
) -> Result<(), AmlError> {
    Ok(())
}

pub async fn generate_alert(
    _action: &entity::aml_actions::Model,
    _context: &AmlContext,
    _state: &web::Data<AppState>,
) -> Result<(), AmlError> {
    Ok(())
}

pub async fn transaction_hold(
    action: &entity::aml_actions::Model,
    context: &AmlContext,
    state: &web::Data<AppState>,
) -> Result<(), AmlError> {
    let trans_val = get_transaction_value(context)?;

    let transaction: TransactionAmlContext = serde_json::from_value(trans_val).map_err(|e| {
        tracing::error!("Failed to parse account: {}", e);
        AmlError::ParserError
    })?;

    let trig_writer = AmlTrigModel {
        action_id: action.id,
        entity: EntityType::Transaction,
        entity_id: transaction.id,
        action: AmlAction::Hold,
    };

    services::write_action_info(&trig_writer, state)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to write action info to mongo");
            AmlError::MethodExecutionError
        })?;

    Ok(())
}

pub async fn transaction_reject(
    action: &entity::aml_actions::Model,
    context: &AmlContext,
    state: &web::Data<AppState>,
) -> Result<(), AmlError> {
    let trans_val = get_transaction_value(context)?;

    let transaction: TransactionAmlContext = serde_json::from_value(trans_val).map_err(|e| {
        tracing::error!("Failed to parse account: {}", e);
        AmlError::ParserError
    })?;

    let trig_writer = AmlTrigModel {
        action_id: action.id,
        entity: EntityType::Transaction,
        entity_id: transaction.id,
        action: AmlAction::Reject,
    };

    services::write_action_info(&trig_writer, state)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to write action info to mongo");
            AmlError::MethodExecutionError
        })?;

    Ok(())
}

pub async fn additional_auth(
    action: &entity::aml_actions::Model,
    context: &AmlContext,
    state: &web::Data<AppState>,
) -> Result<(), AmlError> {
    let trans_val = get_transaction_value(context)?;

    let transaction: TransactionAmlContext = serde_json::from_value(trans_val).map_err(|e| {
        tracing::error!("Failed to parse account: {}", e);
        AmlError::ParserError
    })?;

    let trig_writer = AmlTrigModel {
        action_id: action.id,
        entity: EntityType::Transaction,
        entity_id: transaction.id,
        action: AmlAction::Investigate,
    };

    services::write_action_info(&trig_writer, state)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to write action info to mongo");
            AmlError::MethodExecutionError
        })?;

    Ok(())
}
