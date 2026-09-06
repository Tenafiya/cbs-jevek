use actix_web::{HttpRequest, HttpResponse, web};
use entity::sea_orm_active_enums::{
    AccTypeStatus, AmlRulesExecutionStage, CustomerType, TransactionCategoryType,
    TransactionStatus, TransactionType,
};
use validator::Validate;

use crate::{
    AppState,
    app::{
        accounts,
        amls::{
            self,
            executor::{
                AccountAmlContext, AmlContext, CustomerAmlContext, DepositAmlContext,
                TransactionAmlContext,
            },
            models::AmlModel,
        },
        staffs::models::StaffResponseModel,
        tellers::mapper::TellerCashDrawerRow,
        transactions::{
            models::{
                AddDepositModel, AddDepositParams, AddTransChannelParams, AddTransLimitParams,
                AddTransactionChannelModel, AddTransactionLimitModel, CoreTransactionModel,
            },
            services,
        },
    },
    utils::{
        conversions,
        errors::{ApiCode, ApiError, ApiResponse},
        gen_snow_ids,
    },
};

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

    let data = payload.into_inner();
    let staff = staff.into_inner();
    let drawer = drawer.into_inner();

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

    let amount = conversions::minor_conversion(data.amount, "GHS");
    let group_id = uuid::Uuid::new_v4();
    let currency =
        serde_json::to_value(&data.currency).map_err(|e| ApiError::BadRequest(e.to_string()))?;

    // confirm and implement maker checker, and also increment customer balances and write to ledger
    let deposit = AddDepositModel {
        core: CoreTransactionModel {
            institution_id: staff.institution_id,
            trans_channel_id: gen_snow_ids::id_parser(
                &data.trans_channel_id,
                "Transaction Channel ID",
            )?,
            transaction_type: TransactionType::Credit,
            transaction_category: TransactionCategoryType::CashDeposit,
            status: TransactionStatus::Pending,
            reference: gen_snow_ids::generate_reference_number("DEP"),
            transaction_group_id: group_id,
            amount,
            currency,
            created_by: staff.id,
            total_amount: Some(amount),
            ip_address: None,
            approved_at: None,
            approved_by: None,
        },
        description: Some("Deposit Transaction".to_string()),
        credit_account_id: gen_snow_ids::id_parser(&customer_account.id, "Customer Account ID")?,
        credit_customer_id: gen_snow_ids::id_parser(&customer_account.customer.id, "Customer ID")?,
        drawer_id: gen_snow_ids::id_parser(&drawer.id, "Drawer ID")?,
    };

    let transaction = services::add_deposit_transaction(&deposit, &state)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to save transaction");
            ApiError::InternalServerError
        })?;

    let aml_context = DepositAmlContext {
        transaction: TransactionAmlContext {
            id: transaction.id,
            amount: transaction.amount,
            transaction_type: transaction.transaction_type,
            group_id: transaction.transaction_group_id,
            channel_id: transaction.transaction_channel_id,
            category: transaction.transaction_category,
            currency_name: "GHS".to_string(),
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

    amls::executor::evaluate(&aml_model, &AmlContext::Deposit(aml_context), &state)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to start aml execution");
            ApiError::InternalServerError
        })?;

    Ok(HttpResponse::Accepted().json(ApiResponse::success(
        ApiCode::RequestAccepted,
        "Processing",
        {},
    )))
}
