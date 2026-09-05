use actix_web::web;
use entity::sea_orm_active_enums::AccTypeStatus;

use crate::{
    AppState,
    app::{
        accounts,
        amls::executor::{AccountAmlContext, AmlContext, AmlError, get_account_value},
    },
};

pub async fn log_action(action: &entity::aml_actions::Model) -> Result<(), AmlError> {
    tracing::debug!("Executed action: {:?}", action.id);
    Ok(())
}

pub async fn escalate_action(
    _action: &entity::aml_actions::Model,
    _context: AmlContext,
    _state: &web::Data<AppState>,
) -> Result<(), AmlError> {
    Ok(())
}

pub async fn freeze_account(
    _action: &entity::aml_actions::Model,
    context: &AmlContext,
    state: &web::Data<AppState>,
) -> Result<(), AmlError> {
    let account_val = get_account_value(context)?;

    let account: AccountAmlContext = serde_json::from_value(account_val).map_err(|e| {
        tracing::error!("Failed to parse account: {}", e);
        AmlError::ParserError
    })?;

    accounts::services::toggle_account_status(account.id, AccTypeStatus::Frozen, state)
        .await
        .map_err(|e| {
            tracing::error!("Failed to toggle account status: {}", e);
            AmlError::MethodExecutionError
        })?;

    Ok(())
}
