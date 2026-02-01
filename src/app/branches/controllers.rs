use actix_web::{HttpRequest, HttpResponse, web};

use crate::{
    AppState,
    app::branches::{
        models::{AddBranchModel, AddBranchParams},
        services,
    },
    utils::{
        self,
        errors::{ApiCode, ApiError, ApiResponse},
        models::{PathParamsModel, QueryModel, QueryParamsModel},
    },
};

pub async fn add_branch(
    _req: HttpRequest,
    payload: web::Json<AddBranchParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let data = payload.into_inner();

    let branch = AddBranchModel {
        name: data.name,
        code: utils::gen_snow_ids::get_code(6).await,
        institution: data.institution_id.parse().unwrap_or(0),
        address: data.address,
        phone: data.phone,
        email: data.email,
        location: data.location,
        is_main: data.is_main_branch,
        cash_limit: data.cash_limit,
    };

    match services::save_branch(&branch, &state).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            {},
            None,
        ))),
        Err(err) => Err(ApiError::BadRequest(err.to_string())),
    }
}

pub async fn get_branch_details(
    _req: HttpRequest,
    params: web::Path<PathParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let data = params.into_inner();

    match services::get_details(&data.id.parse().unwrap_or(0), &state).await {
        Ok(branch) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            branch,
            None,
        ))),
        Err(_) => Err(ApiError::NotFound),
    }
}

pub async fn get_branches(
    _req: HttpRequest,
    query: web::Query<QueryParamsModel>,
    params: web::Path<PathParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let data = params.into_inner();

    let query = QueryModel {
        size: query.size,
        page: query.page,
    };

    match services::get_all(&data.id.parse().unwrap_or(0), &query, &state).await {
        Ok(branches) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            branches,
            None,
        ))),
        Err(_) => Err(ApiError::NotFound),
    }
}
