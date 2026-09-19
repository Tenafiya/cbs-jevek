use actix_web::web;
use chrono::NaiveDate;
use sea_orm::TransactionTrait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{AppState, app::accounts::services, utils::errors::ApiError};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum AccountDailyProcessorError {
    #[error("Invalid resource type")]
    InvalidResourceType,
    #[error("IO error")]
    IoError,
    #[error("Parser error")]
    ParserError,
    #[error("Method execution error")]
    MethodExecutionError,
}

#[derive(Debug, Clone)]
pub struct UpdateCustomerAccountModel {
    pub account_id: i64,
    pub customer_id: i64,
    pub amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordAccountDaily {
    pub amount: i64,
    pub bal_date: NaiveDate,
    pub account_id: i64,
}

pub async fn process_daily_balance_record(
    model: String,
    state: &web::Data<AppState>,
) -> Result<(), AccountDailyProcessorError> {
    let data: RecordAccountDaily = serde_json::from_str(&model).map_err(|e| {
        tracing::error!("Failed to parse created actions: {}", e);
        AccountDailyProcessorError::ParserError
    })?;

    services::record_account_dailys(data.bal_date, data.account_id, data.amount, state)
        .await
        .map_err(|e| {
            tracing::error!("Failed to record account daily balance: {}", e);
            AccountDailyProcessorError::MethodExecutionError
        })?;

    Ok(())
}

pub async fn update_acc_deposit_balances(
    model: &UpdateCustomerAccountModel,
    state: &web::Data<AppState>,
) -> Result<(), ApiError> {
    let trn = state.pgdb.get_ref().begin().await.map_err(|e| {
        tracing::error!(error = ?e, "Failed to start db transaction for deposit account balance updates");
        ApiError::InternalServerError
    })?;

    let data = model.clone();

    services::update_account_limit_value(data.account_id, data.amount, &trn)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to update account limit");
            ApiError::InternalServerError
        })?;

    services::update_main_account_bal_deposit(data.account_id, data.customer_id, data.amount, &trn)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to update account balances");
            ApiError::InternalServerError
        })?;

    trn.commit().await.map_err(|e| {
        tracing::error!(error = ?e, "Failed to commit db transaction for deposit account balance updates");
        ApiError::InternalServerError
    })?;

    let daily = RecordAccountDaily {
        account_id: data.account_id,
        amount: data.amount,
        bal_date: chrono::Utc::now().date_naive(),
    };

    let message = serde_json::to_string(&daily).map_err(|e| {
        tracing::error!("Failed to parse daily: {}", e);
        ApiError::InternalServerError
    })?;

    state
        .streamer
        .publish_to_stream("accounts.daily.new", message)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to publish account new daily balance");
            ApiError::InternalServerError
        })?;

    Ok(())
}
