use actix_web::{HttpRequest, HttpResponse, web};

use crate::{
    AppState,
    app::institutions::{
        models::{
            AddInstitutionModel, AddInstitutionParams, UpdateInstitutionModel,
            UpdateInstitutionParams,
        },
        services,
    },
    utils::{
        self,
        errors::{ApiCode, ApiError, ApiResponse},
        models::{PathParamsModel, QueryModel, QueryParamsModel},
    },
};

pub async fn add_institution(
    _req: HttpRequest,
    payload: web::Json<AddInstitutionParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let data = payload.into_inner();

    let institution = AddInstitutionModel {
        name: data.name,
        code: utils::gen_snow_ids::get_code(6).await,
        country: data.country_id.parse().unwrap_or(0),
        license_num: data.license_num,
        regulation_num: data.regulation_num,
    };

    match services::save_institution(&institution, &state).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Institution Created",
            {},
            None,
        ))),
        Err(err) => Err(ApiError::BadRequest(err.to_string())),
    }
}

pub async fn get_institution(
    _req: HttpRequest,
    params: web::Path<PathParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let data = params.into_inner();

    match services::get_one(&data.id.parse().unwrap_or(0), &state).await {
        Ok(ins) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            ins,
            None,
        ))),
        Err(_) => Err(ApiError::NotFound),
    }
}

pub async fn get_institutions(
    _req: HttpRequest,
    query: web::Query<QueryParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let query = QueryModel {
        size: query.size,
        page: query.page,
    };

    match services::get_all(&query, &state).await {
        Ok(insts) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            insts,
            None,
        ))),
        Err(_) => Err(ApiError::NotFound),
    }
}

pub async fn update_institution(
    _req: HttpRequest,
    payload: web::Json<UpdateInstitutionParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let data = payload.into_inner();

    let update_model = UpdateInstitutionModel {
        name: data.name,
        timezone: data.timezone,
        license_num: data.license_num,
        regulation_num: data.regulation_num,
    };

    match services::update(&data.id.parse().unwrap_or(0), &update_model, &state).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            {},
            None,
        ))),
        Err(_) => Err(ApiError::InternalServerError),
    }
}
