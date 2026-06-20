use actix_web::{HttpRequest, HttpResponse, web};
use entity::sea_orm_active_enums::AccTypeStatus;
use validator::Validate;

use crate::{
    AppState,
    app::{
        account_charts::{
            models::{
                AddAccountCategoryModel, AddAccountCategoryParams, AddAccountChartModel,
                AddAccountChartParams, AddAccountTypeModel, AddAccountTypeParams,
            },
            services,
        },
        staffs::models::StaffResponseModel,
    },
    utils::{
        conversions,
        errors::{ApiCode, ApiError, ApiResponse},
        gen_snow_ids,
    },
};

pub async fn add_acc_chart(
    _req: HttpRequest,
    state: web::Data<AppState>,
    staff: web::ReqData<StaffResponseModel>,
    payload: web::Json<AddAccountChartParams>,
) -> Result<HttpResponse, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let data = payload.into_inner();

    let StaffResponseModel { institution_id, .. } = staff.into_inner();

    let parent_acc_id = data
        .parent_account
        .map(|par_id| gen_snow_ids::id_parser(&par_id, "Parent Account ID"))
        .transpose()?;

    let acc_chart = AddAccountChartModel {
        institution_id,
        acc_code: gen_snow_ids::gen_string(14).await,
        acc_name: data.acc_name,
        acc_type: data.acc_type,
        currency_code: data.currency_code,
        parent_acc_id,
        is_system_acc: data.is_system_acc,
    };

    match services::save_acc_chart(&acc_chart, &state).await {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::success(
            ApiCode::ResourceCreated,
            "Successful",
            {},
        ))),
        Err(e) => {
            tracing::error!(error = ?e, "failed to add account charts");
            Err(ApiError::InternalServerError)
        },
    }
}

pub async fn add_acc_cat(
    _req: HttpRequest,
    state: web::Data<AppState>,
    staff: web::ReqData<StaffResponseModel>,
    payload: web::Json<AddAccountCategoryParams>,
) -> Result<HttpResponse, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let data = payload.into_inner();

    let StaffResponseModel { institution_id, .. } = staff.into_inner();

    let acc_cat = AddAccountCategoryModel {
        institution_id,
        name: Some(data.name),
        category_type: Some(data.category_type),
        description: Some(data.description),
    };

    match services::save_account_category(&acc_cat, &state).await {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::success(
            ApiCode::ResourceCreated,
            "Successful",
            {},
        ))),
        Err(e) => {
            tracing::error!(error = ?e, "failed to add account category");
            Err(ApiError::InternalServerError)
        },
    }
}

pub async fn fetch_charts(
    _req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    match services::get_charts(&state).await {
        Ok(res) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            res,
        ))),
        Err(e) => {
            tracing::error!(error = ?e, "failed to fetch account charts");
            Err(ApiError::NotFound)
        },
    }
}

pub async fn fetch_categories(
    _req: HttpRequest,
    state: web::Data<AppState>,
    staff: web::ReqData<StaffResponseModel>,
) -> Result<HttpResponse, ApiError> {
    let StaffResponseModel { institution_id, .. } = staff.into_inner();

    match services::get_account_categories(institution_id, &state).await {
        Ok(res) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            res,
        ))),
        Err(e) => {
            tracing::error!(error = ?e, "failed to fetch account categories");
            Err(ApiError::NotFound)
        },
    }
}

pub async fn add_acc_types(
    _req: HttpRequest,
    state: web::Data<AppState>,
    staff: web::ReqData<StaffResponseModel>,
    payload: web::Json<AddAccountTypeParams>,
) -> Result<HttpResponse, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let data = payload.into_inner();

    let StaffResponseModel { institution_id, .. } = staff.into_inner();

    let maintenance_fee = data
        .maintenance_fee
        .map(|fee| conversions::minor_conversion(fee, "Ghs"));

    let withdrawal_fee = data
        .withdrawal_fee
        .map(|fee| conversions::minor_conversion(fee, "Ghs"));

    let acc_type = AddAccountTypeModel {
        institution_id,
        category_id: gen_snow_ids::id_parser(&data.category_id, "Category ID")?,
        name: Some(data.name),
        code: Some(gen_snow_ids::gen_string(13).await),
        description: Some(data.description),
        currency: gen_snow_ids::get_serde_value(&data.currency)?,
        min_balance: conversions::minor_conversion(data.min_balance, "GHs"),
        max_balance: conversions::minor_conversion(data.max_balance, "Ghs"),
        interest_rate: data.interest_rate,
        interest_rate_calc: data.interest_rate_calc,
        interest_payout_freq: data.interest_payout_freq,
        is_overdraft_allowable: data.is_overdraft_allowable,
        overdraft_limit: data.overdraft_limit,
        overdraft_interest_rate: data.overdraft_interest_rate,
        dormancy_period: data.dormancy_period,
        maintenance_fee,
        withdrawal_fee,
        status: AccTypeStatus::Active,
    };

    match services::save_acc_type(&acc_type, &state).await {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::success(
            ApiCode::ResourceCreated,
            "Successful",
            {},
        ))),
        Err(e) => {
            tracing::error!(error = ?e, "failed to add account types");
            Err(ApiError::InternalServerError)
        },
    }
}

pub async fn fetch_account_types(
    _req: HttpRequest,
    state: web::Data<AppState>,
    staff: web::ReqData<StaffResponseModel>,
) -> Result<HttpResponse, ApiError> {
    let StaffResponseModel { institution_id, .. } = staff.into_inner();

    match services::get_account_types(institution_id, &state).await {
        Ok(res) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            res,
        ))),
        Err(e) => {
            tracing::error!(error = ?e, "failed to fetch account types");
            Err(ApiError::NotFound)
        },
    }
}
