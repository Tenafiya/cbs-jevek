use actix_web::{HttpRequest, HttpResponse, web};
use entity::sea_orm_active_enums::{
    AccTypeStatus, AmlRulesExecutionStage, CustomerType, TransactionCategoryType,
    TransactionStatus, TransactionType,
};
use redis::AsyncCommands;
use validator::Validate;

use crate::{
    AppState,
    app::{
        account_charts,
        accounts::{self, processor::UpdateCustomerAccountModel},
        amls::{
            self,
            executor::{
                AccountAmlContext, AmlContext, CustomerAmlContext, DepositAmlContext,
                TransactionAmlContext,
            },
            models::AmlModel,
            mongo_model::AmlAction,
        },
        gls::{self, models::AddGlRecordModel},
        staffs::models::StaffResponseModel,
        tellers::mapper::TellerCashDrawerRow,
        transactions::{
            models::{
                AddDepositModel, AddDepositParams, AddTransChannelParams, AddTransLimitParams,
                AddTransactionChannelModel, AddTransactionLimitModel, CoreTransactionModel,
            },
            mongo_model::TrigStatus,
            services,
        },
    },
    utils::{
        conversions,
        errors::{ApiCode, ApiError, ApiResponse},
        gen_snow_ids,
    },
};

pub async fn check_aml_deposit_triggers(
    entity_ids: Vec<i64>,
    deposit: &AddDepositModel,
    state: &web::Data<AppState>,
) -> Result<(), ApiError> {
    let trigger_actions = amls::services::find_all_action_info(entity_ids, state)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to retrieve actions");
            ApiError::InternalServerError
        })?;

    for trigger_action in trigger_actions {
        match trigger_action.action {
            AmlAction::Freeze => {
                return Err(ApiError::Unprocessable(
                    "Suspicious Account Activity, Account Frozen".to_string(),
                ));
            }
            AmlAction::Hold => {
                services::add_mongo_deposit_transaction(&deposit, TrigStatus::Hold, state)
                    .await
                    .map_err(|e| {
                        tracing::error!(error = ?e, "Failed to record held transaction");
                        ApiError::InternalServerError
                    })?;

                return Err(ApiError::Unprocessable(
                    "Suspicious Transaction, Transaction On Hold".to_string(),
                ));
            }
            AmlAction::Reject => {
                services::add_mongo_deposit_transaction(&deposit, TrigStatus::Reject, state)
                    .await
                    .map_err(|e| {
                        tracing::error!(error = ?e, "Failed to record reject transaction");
                        ApiError::InternalServerError
                    })?;

                return Err(ApiError::Unprocessable(
                    "Suspicious Transaction, Transaction Rejected".to_string(),
                ));
            }
            AmlAction::Investigate => {
                services::add_mongo_deposit_transaction(&deposit, TrigStatus::Auth, state)
                    .await
                    .map_err(|e| {
                        tracing::error!(error = ?e, "Failed to record escalate transaction");
                        ApiError::InternalServerError
                    })?;

                return Err(ApiError::Unprocessable(
                    "Suspicious Transaction, Transaction Pending Investigation".to_string(),
                ));
            }
            _ => {}
        }
    }

    Ok(())
}

pub async fn set_trans_lock(
    cache_key: String,
    state: &web::Data<AppState>,
) -> Result<(), ApiError> {
    let mut conn = state.cache.get_ref().clone();

    let setting_key: bool = conn.set_nx(&cache_key, "1").await.map_err(|e| {
        tracing::error!(error = ?e, "Failed to set cache");
        ApiError::InternalServerError
    })?;

    if !setting_key {
        return Err(ApiError::Conflict(
            "Account already locked for processing".to_string(),
        ));
    }

    let _: bool = conn.expire(&cache_key, 600).await.map_err(|e| {
        tracing::error!(error = ?e, "deposit key expiration failed");
        ApiError::InternalServerError
    })?;

    Ok(())
}

pub async fn drop_trans_lock(
    cache_key: String,
    state: &web::Data<AppState>,
) -> Result<(), ApiError> {
    let mut conn = state.cache.get_ref().clone();

    let _: i64 = conn.del(&cache_key).await.map_err(|e| {
        tracing::error!(error = ?e, "Failed to delete deposit cache key");
        ApiError::InternalServerError
    })?;

    Ok(())
}

pub async fn create_trans_channel(
    _req: HttpRequest,
    state: web::Data<AppState>,
    staff: web::ReqData<StaffResponseModel>,
    payload: web::Json<AddTransChannelParams>,
) -> Result<HttpResponse, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let data = payload.into_inner();

    let code = gen_snow_ids::get_code(8);

    let channel = AddTransactionChannelModel {
        institution_id: staff.institution_id,
        channel_name: Some(data.channel_name),
        channel_code: Some(code),
        description: data.description,
        requires_maker_checker: data.requires_approval,
        metadata: data.metadata,
    };

    match services::add_trans_channel(&channel, &state).await {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::success(
            ApiCode::ResourceCreated,
            "Successful",
            {},
        ))),
        Err(e) => {
            tracing::error!(error = ?e, "Failed to save transaction channel");
            Err(ApiError::InternalServerError)
        }
    }
}

pub async fn create_trans_limit(
    _req: HttpRequest,
    state: web::Data<AppState>,
    staff: web::ReqData<StaffResponseModel>,
    payload: web::Json<AddTransLimitParams>,
) -> Result<HttpResponse, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let data = payload.into_inner();

    let currency = serde_json::to_value(data.currency.as_ref()).map_err(|e| {
        tracing::error!(error = ?e, "Failed to serialize currency");
        ApiError::InternalServerError
    })?;

    let max_amount = data
        .max_amount
        .as_ref()
        .map(|amt| conversions::minor_conversion(*amt, "GHS"));

    let limit = AddTransactionLimitModel {
        institution_id: staff.institution_id,
        trans_channel_id: gen_snow_ids::id_parser(
            &data.trans_channel_id,
            "Transaction Channel ID",
        )?,
        customer_type: data.customer_type,
        acc_category_id: gen_snow_ids::id_parser(&data.acc_category_id, "Account Category ID")?,
        limit_type: data.limit_type,
        max_amount,
        max_count: data.max_count,
        effective_from: data.effective_dates.as_ref().map(|d| d.effective_from),
        effective_to: data.effective_dates.as_ref().map(|d| d.effective_to),
        currency: Some(currency),
    };

    match services::add_trans_limit(&limit, &state).await {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::success(
            ApiCode::ResourceCreated,
            "Successful",
            {},
        ))),
        Err(e) => {
            tracing::error!(error = ?e, "Failed to add transaction limit");
            Err(ApiError::InternalServerError)
        }
    }
}

pub async fn get_trans_checkers(
    _req: HttpRequest,
    state: web::Data<AppState>,
    staff: web::ReqData<StaffResponseModel>,
) -> Result<HttpResponse, ApiError> {
    match services::fetch_checker_limits(staff.institution_id, &state).await {
        Ok(limits) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            limits,
        ))),
        Err(e) => {
            tracing::error!(error = ?e, "Failed to fetch checker limits");
            Err(ApiError::InternalServerError)
        }
    }
}

pub async fn get_trans_limits(
    _req: HttpRequest,
    state: web::Data<AppState>,
    staff: web::ReqData<StaffResponseModel>,
) -> Result<HttpResponse, ApiError> {
    match services::fetch_transaction_limits(staff.institution_id, &state).await {
        Ok(limits) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            limits,
        ))),
        Err(e) => {
            tracing::error!(error = ?e, "Failed to fetch transaction limits");
            Err(ApiError::InternalServerError)
        }
    }
}

pub async fn get_trans_channels(
    _req: HttpRequest,
    state: web::Data<AppState>,
    staff: web::ReqData<StaffResponseModel>,
) -> Result<HttpResponse, ApiError> {
    match services::fetch_transaction_channels(staff.institution_id, &state).await {
        Ok(channels) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            channels,
        ))),
        Err(e) => {
            tracing::error!(error = ?e, "Failed to fetch transaction channels");
            Err(ApiError::InternalServerError)
        }
    }
}

pub async fn process_deposit_trans(
    _req: HttpRequest,
    state: web::Data<AppState>,
    staff: web::ReqData<StaffResponseModel>,
    drawer: web::ReqData<TellerCashDrawerRow>,
    payload: web::Json<AddDepositParams>,
) -> Result<HttpResponse, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let (transaction_id, _) = gen_snow_ids::gen_snowflake_slug().map_err(|e| {
        tracing::error!(error = ?e, "Failed to generate transaction id");
        ApiError::InternalServerError
    })?;

    let data = payload.into_inner();
    let staff = staff.into_inner();
    let drawer = drawer.into_inner();

    // check if ledger is lock
    if let Ok(Some(_)) = gls::services::get_ledger_lock_period(staff.institution_id, &state).await {
        tracing::error!("Ledger is locked");
        return Err(ApiError::InternalServerError);
    };

    let customer_id = gen_snow_ids::id_parser(&data.customer_id, "Customer ID")?;
    let account_id = gen_snow_ids::id_parser(&data.account_id, "Account ID")?;

    let cus_acc = accounts::services::fetch_customer_acc_id(
        customer_id,
        account_id,
        AccTypeStatus::Active,
        &state,
    )
    .await
    .map_err(|e| {
        tracing::error!(error = ?e, "Fetch customer account error");
        ApiError::InternalServerError
    })?;

    let customer_account = cus_acc.ok_or_else(|| {
        tracing::error!("Customer account not found");
        ApiError::BadRequest("Customer account not found".to_string())
    })?;

    let asset_accounts =
        account_charts::controllers::get_deposit_charts(staff.institution_id, &state)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "Error getting asset accounts");
                ApiError::InternalServerError
            })?;

    let account_id = gen_snow_ids::id_parser(&customer_account.id, "Customer Account ID")?;
    let amount = conversions::minor_conversion(data.amount, "GHS");
    let group_id = uuid::Uuid::new_v4();
    let currency =
        serde_json::to_value(&data.currency).map_err(|e| ApiError::BadRequest(e.to_string()))?;
    let channel_id = gen_snow_ids::id_parser(&data.trans_channel_id, "Transaction Channel ID")?;

    // lock customer account till deposit processes completes or TTL is 10 mins
    let cus_acc_lock_key = format!("deposit-{}:{}", customer_id, account_id);
    set_trans_lock(cus_acc_lock_key.clone(), &state).await?;

    // check customer account limit
    let acc_check = accounts::services::get_account_credit_limit(account_id, amount, 1, &state)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to get account limit");
            ApiError::InternalServerError
        })?;

    if !acc_check.is_success() {
        return Err(ApiError::Unprocessable(
            "Customer has exceeded their limits".to_string(),
        ));
    }

    // check limits related to this transaction channel
    let transaction_check = services::fetch_checker_limit(staff.institution_id, channel_id, &state)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Transaction checker query fail: {}", e);
            ApiError::InternalServerError
        })?;

    let requires_approval = if transaction_check.channel.requires_maker_checker {
        true
    } else {
        false
    };

    // create aml context
    let aml_context = DepositAmlContext {
        transaction: TransactionAmlContext {
            id: transaction_id,
            amount: amount,
            transaction_type: TransactionType::Credit,
            group_id,
            channel_id,
            category: TransactionCategoryType::CashDeposit,
            currency_name: "GHS".to_string(),
            requires_approval,
        },
        account: AccountAmlContext {
            id: gen_snow_ids::id_parser(&customer_account.id, "Account ID")?,
            account_type_id: gen_snow_ids::id_parser(
                &customer_account.account_type.id,
                "Account Type ID",
            )?,
            balance: customer_account.current_balance,
        },
        customer: CustomerAmlContext {
            id: gen_snow_ids::id_parser(&customer_account.customer.id, "Customer ID")?,
            customer_type: customer_account
                .customer
                .customer_type
                .unwrap_or(CustomerType::default()),
            institution_id: staff.institution_id,
        },
    };

    let aml_model = AmlModel {
        institution_id: staff.institution_id,
        stage: AmlRulesExecutionStage::PreTransaction,
    };

    // start aml processing
    amls::executor::evaluate(&aml_model, &AmlContext::Deposit(aml_context), &state)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to start aml execution");
            ApiError::InternalServerError
        })?;

    // check transaction context
    let deposit = AddDepositModel {
        core: CoreTransactionModel {
            id: Some(transaction_id),
            institution_id: staff.institution_id,
            trans_channel_id: gen_snow_ids::id_parser(&transaction_check.channel.id, "Channel ID")?,
            transaction_type: TransactionType::Credit,
            transaction_category: TransactionCategoryType::CashDeposit,
            status: TransactionStatus::Pending,
            reference: gen_snow_ids::generate_reference_number("DEP"),
            transaction_group_id: group_id,
            amount,
            currency,
            created_by: staff.id,
            fee_amount: None,
            vat_amount: None,
            total_amount: Some(amount),
            ip_address: None,
            approved_at: None,
            approved_by: None,
            requires_approval,
        },
        description: Some("Deposit Transaction".to_string()),
        credit_account_id: account_id,
        credit_customer_id: gen_snow_ids::id_parser(&customer_account.customer.id, "Customer ID")?,
        drawer_id: gen_snow_ids::id_parser(&drawer.id, "Drawer ID")?,
    };

    // check aml triggers before recording transaction
    let entity_ids: Vec<i64> = vec![account_id, transaction_id, customer_id];
    check_aml_deposit_triggers(entity_ids, &deposit, &state).await?;

    // write transaction to db
    let transaction = services::add_deposit_transaction(&deposit, &state)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to save transaction");
            ApiError::InternalServerError
        })?;

    // update customer account balances
    let customer_acc_update = UpdateCustomerAccountModel {
        account_id,
        customer_id,
        amount: transaction.amount,
    };

    accounts::processor::update_acc_deposit_balances(&customer_acc_update, &state)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Update Customer account balances failed");
            ApiError::InternalServerError
        })?;

    // update gl records
    let (asset_gl, cash_gl) = asset_accounts;

    let gl_data = AddGlRecordModel {
        institution_id: staff.institution_id,
        transaction_id: transaction.id,
        cus_account_id: account_id,
        reference: transaction
            .transaction_reference
            .unwrap_or(format!("Transaction for :{}", transaction.id)),
        entry_type: TransactionType::Credit,
        amount: transaction.amount,
        debit_account: asset_gl.id,
        credit_account: cash_gl.id,
        posted_by: staff.id,
    };

    gls::processor::setup_gl_record(&gl_data, &state)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to record gl data");
            ApiError::InternalServerError
        })?;

    drop_trans_lock(cus_acc_lock_key, &state).await?;

    Ok(HttpResponse::Accepted().json(ApiResponse::success(
        ApiCode::RequestAccepted,
        "Processing",
        {},
    )))
}
