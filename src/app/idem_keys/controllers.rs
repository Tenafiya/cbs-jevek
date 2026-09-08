use actix_web::web;
use thiserror::Error;

use crate::{
    AppState,
    app::idem_keys::{
        models::{IdemModel, InitIdemModel},
        services,
    },
    utils::password::compute_hash,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum IdemError {
    #[error("Database Error : {0}")]
    DbError(String),
    #[error("Parser error")]
    ParserError,
    #[error("Rule fetch error")]
    RuleFetchError,
    #[error("Unsupported operator")]
    UnsupportedOperator,
}

pub async fn handle_idem_processing(
    idem: &String,
    state: &web::Data<AppState>,
) -> Result<(), IdemError> {
    let init_idem: InitIdemModel = serde_json::from_str(idem).map_err(|e| {
        tracing::error!(error = ?e, "Failed to parse initial idempotency model");
        IdemError::ParserError
    })?;

    let request_hash = compute_hash(idem.as_bytes());

    let idem_model = IdemModel {
        institution_id: init_idem.institution_id,
        idem_key: init_idem.idem_key,
        request_method: init_idem.request_method,
        request_path: init_idem.request_path,
        channel: Some(init_idem.channel),
        operation: init_idem.operation,
        request_hash,
        customer_id: None,
        account_id: None,
        transaction_id: None,
        transaction_group_id: None,
    };

    services::write_idem_key(&idem_model, &state)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to write idempotency to the db");
            IdemError::DbError(e.to_string())
        })?;

    Ok(())
}
