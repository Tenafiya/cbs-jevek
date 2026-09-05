use actix_web::{HttpRequest, HttpResponse, web};
use validator::Validate;

use crate::{
    AppState,
    app::{
        staffs::models::StaffResponseModel,
        transactions::{
            models::{
                AddDepositParams, AddTransChannelParams, AddTransLimitParams,
                AddTransactionChannelModel, AddTransactionLimitModel,
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

// pub async fn process_deposit_trans(_req: HttpRequest, state: web::Data<AppState>, staff: web::ReqData<StaffResponseModel>, payload: web::Json<AddDepositParams>) -> Result<HttpResponse, ApiError> {
//     payload.validate().map_err(|e| {
//         ApiError::BadRequest(e.to_string())
//     })?;

//     let data = payload.into_inner();

// }
