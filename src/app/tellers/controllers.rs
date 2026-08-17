use actix_web::{HttpRequest, HttpResponse, web};
use entity::sea_orm_active_enums::TellerReconType;
use sea_orm::TransactionTrait;
use validator::Validate;

use crate::{
    AppState,
    app::{
        staffs::{self, models::StaffResponseModel},
        tellers::{
            models::{
                AddDrawerModel, AddDrawerParams, AddTellerModel, AddTellerParams,
                AddTellerReconModel, AddTellerReconParams,
            },
            services,
        },
    },
    utils::{
        errors::{ApiCode, ApiError, ApiResponse},
        gen_snow_ids,
        models::{ListResponseModel, PathParamsModel, QueryModel, QueryParamsModel},
    },
};

pub async fn create_teller(
    _req: HttpRequest,
    state: web::Data<AppState>,
    staff: web::ReqData<StaffResponseModel>,
    payload: web::Json<AddTellerParams>,
) -> Result<HttpResponse, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let data = payload.into_inner();

    let staff_id = gen_snow_ids::id_parser(&data.staff_id, "Staff ID")?;

    let teller_staff = staffs::services::get_staff_details(staff_id, &state)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Staff not found");
            ApiError::InternalServerError
        })?;

    let teller_name = format!("{} {}", staff.first_name, staff.last_name);

    let num = gen_snow_ids::get_code(3);

    let teller = AddTellerModel {
        institution_id: teller_staff.institution_id,
        branch_id: gen_snow_ids::id_parser(&data.branch_id, "Branch ID")?,
        teller_name,
        teller_number: num,
        staff_id: teller_staff.id,
    };

    match services::add_teller(&teller, &state).await {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::success(
            ApiCode::ResourceCreated,
            "Successful",
            {},
        ))),
        Err(e) => {
            tracing::error!(error = ?e, "Failed to add teller");
            Err(ApiError::InternalServerError)
        }
    }
}

pub async fn create_daily_recon(
    _req: HttpRequest,
    state: web::Data<AppState>,
    _staff: web::ReqData<StaffResponseModel>,
    payload: web::Json<AddTellerReconParams>,
) -> Result<HttpResponse, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let trn = state.pgdb.get_ref().begin().await.map_err(|e| {
        tracing::error!(error = ?e, "Failed to begin transaction");
        ApiError::InternalServerError
    })?;

    let data = payload.into_inner();

    // find cash drawer
    let recon = AddTellerReconModel {
        cash_drawer_id: gen_snow_ids::id_parser(&data.cash_drawer_id, "Cash drawer ID")?,
        recon_type: Some(TellerReconType::Daily),
        notes: data.notes,
        supervisor_id: None,
    };

    services::add_recon(&recon, &trn).await.map_err(|e| {
        tracing::error!(error = ?e, "Failed to add recon");
        ApiError::InternalServerError
    })?;

    trn.commit().await.map_err(|e| {
        tracing::error!(error = ?e, "Failed to commit transaction");
        ApiError::InternalServerError
    })?;

    Ok(HttpResponse::Created().json(ApiResponse::success(
        ApiCode::ResourceCreated,
        "Successful",
        {},
    )))
}

pub async fn start_drawer_session(
    _req: HttpRequest,
    state: web::Data<AppState>,
    staff: web::ReqData<StaffResponseModel>,
    payload: web::Json<AddDrawerParams>,
) -> Result<HttpResponse, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let data = payload.into_inner();

    let teller_id = gen_snow_ids::id_parser(&data.teller_id, "Teller ID")?;

    let teller = services::get_teller(teller_id, staff.institution_id, &state)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to get teller");
            ApiError::InternalServerError
        })?;

    let cash = serde_json::to_value(&data.opening_cash).map_err(|e| {
        tracing::error!(error = ?e, "Failed to serialize opening cash");
        ApiError::InternalServerError
    })?;

    let drawer = AddDrawerModel {
        teller_id: gen_snow_ids::id_parser(&teller.id, "Teller ID")?,
        opening_cash_amount: data.opening_cash_amount,
        opening_cash: cash,
    };

    match services::open_drawer(&drawer, &state).await {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::success(
            ApiCode::ResourceCreated,
            "Successful",
            {},
        ))),
        Err(e) => {
            tracing::error!(error = ?e, "Failed to open drawer");
            Err(ApiError::InternalServerError)
        }
    }
}

pub async fn fetch_teller_details(
    _req: HttpRequest,
    state: web::Data<AppState>,
    staff: web::ReqData<StaffResponseModel>,
    params: web::Path<PathParamsModel>,
) -> Result<HttpResponse, ApiError> {
    params
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let path = params.into_inner();

    let teller_id = gen_snow_ids::id_parser(&path.id, "Teller ID")?;

    let teller = services::get_teller(teller_id, staff.institution_id, &state)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to get teller");
            ApiError::InternalServerError
        })?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(
        ApiCode::OperationSuccess,
        "Successful",
        teller,
    )))
}

pub async fn fetch_teller_list(
    _req: HttpRequest,
    state: web::Data<AppState>,
    staff: web::ReqData<StaffResponseModel>,
    query: web::Query<QueryParamsModel>,
) -> Result<HttpResponse, ApiError> {
    query
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let query = query.into_inner();

    let q_model = QueryModel {
        size: query.size,
        page: query.page,
    };

    let (items, meta) = services::get_teller_list(staff.institution_id, q_model, &state)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to get teller list");
            ApiError::InternalServerError
        })?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(
        ApiCode::OperationSuccess,
        "Successful",
        ListResponseModel { items, meta },
    )))
}
