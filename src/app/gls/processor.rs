use actix_web::web;
use entity::sea_orm_active_enums::TransactionType;
use sea_orm::TransactionTrait;

use crate::{
    AppState,
    app::gls::{
        models::{
            AddGlDailyBalance, AddGlPostings, AddGlRecordModel, AddLedgerEntry, GlAccountInfo,
        },
        services::{save_gl_posting, save_ledger_entry, upsert_gl_daily_balances},
    },
    utils::errors::ApiError,
};

pub async fn setup_gl_record(
    model: &AddGlRecordModel,
    state: &web::Data<AppState>,
) -> Result<(), ApiError> {
    let trn = state.pgdb.get_ref().begin().await.map_err(|e| {
        tracing::error!(error = ?e, "Failed to start db transaction for gl record setup");
        ApiError::InternalServerError
    })?;

    let now = chrono::Utc::now();

    let data = model.clone();

    // record ledger entry (Credit)
    let entry = AddLedgerEntry {
        institution_id: data.institution_id,
        transaction_id: data.transaction_id,
        account_id: Some(data.cus_account_id),
        entry_type: TransactionType::Credit,
        gl_account_id: None,
        currency_code: "GHS".to_string(),
        description: "Customer Deposit".to_string(),
        reference: data.reference.clone(),
        amount: data.amount,
    };

    save_ledger_entry(&entry, &trn).await.map_err(|e| {
        tracing::error!(error = ?e, "Failed to save ledger entry");
        ApiError::InternalServerError
    })?;

    let mut gl_infos = Vec::new();

    // GL double entry (for gl postins)
    // GL debit (credit amount = 0) & (debit account = asset account debited) & (credit account = asset account credited) both these accounts are gl accounts
    let gl_debit_posting = AddGlPostings {
        institution_id: data.institution_id,
        transaction_id: data.transaction_id,
        ref_number: data.reference.clone(),
        value_date: now.date_naive(),
        posting_date: now,
        debit_account_id: data.debit_account,
        debit_amount: data.amount,
        credit_account_id: data.credit_account,
        credit_amount: 0,
        narration: "Customer deposit - debit to customer asset".to_string(),
        is_reversed: false,
        posted_by: data.posted_by,
    };

    save_gl_posting(&gl_debit_posting, &trn)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to save debit gl posting");
            ApiError::InternalServerError
        })?;

    let gl_debit_info = GlAccountInfo {
        debit_delta: data.amount,
        credit_delta: 0,
        gl_account_id: data.debit_account,
    };

    gl_infos.push(gl_debit_info);

    // GL credit (debit amount = 0)
    let gl_credit_posting = AddGlPostings {
        institution_id: data.institution_id,
        transaction_id: data.transaction_id,
        ref_number: data.reference.clone(),
        value_date: now.date_naive(),
        posting_date: now,
        debit_account_id: data.debit_account,
        debit_amount: 0,
        credit_account_id: data.credit_account,
        credit_amount: data.amount,
        narration: "Customer deposit - credit to cash account".to_string(),
        is_reversed: false,
        posted_by: data.posted_by,
    };

    save_gl_posting(&gl_credit_posting, &trn)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to save credit gl posting");
            ApiError::InternalServerError
        })?;

    let gl_credit_info = GlAccountInfo {
        debit_delta: 0,
        credit_delta: data.amount,
        gl_account_id: data.credit_account,
    };

    gl_infos.push(gl_credit_info);

    let gl_daily = AddGlDailyBalance {
        institution_id: data.institution_id,
        gl_infos,
        balance_date: now.date_naive(),
        count_delta: 1,
    };

    upsert_gl_daily_balances(&gl_daily, &trn)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to save gl daily");
            ApiError::InternalServerError
        })?;

    trn.commit().await.map_err(|e| {
        tracing::error!(error = ?e, "Failed to commit db transaction for gl record setup");
        ApiError::InternalServerError
    })?;

    Ok(())
}
