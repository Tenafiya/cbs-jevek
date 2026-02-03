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
        models::{ListResponseModel, PathParamsModel, QueryModel, QueryParamsModel},
    },
};

pub async fn add_branch(
    _req: HttpRequest,
    payload: web::Json<AddBranchParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let data = payload.into_inner();

    let institution_id = data
        .institution_id
        .parse::<i64>()
        .map_err(|_| ApiError::BadRequest("Invalid Institution ID format".to_string()))?;

    let branch = AddBranchModel {
        name: data.name,
        code: utils::gen_snow_ids::get_code(6).await,
        institution: institution_id,
        address: data.address,
        phone: data.phone,
        email: data.email,
        location: data.location,
        is_main: data.is_main_branch,
        cash_limit: data.cash_limit,
    };

    match services::save_branch(&branch, &state).await {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::success(
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

    let id = data
        .id
        .parse::<i64>()
        .map_err(|_| ApiError::BadRequest("Invalid ID format".to_string()))?;

    match services::get_details(&id, &state).await {
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

    let id = data
        .id
        .parse::<i64>()
        .map_err(|_| ApiError::BadRequest("Invalid ID format".to_string()))?;

    let query = QueryModel {
        size: query.size,
        page: query.page,
    };

    match services::get_all(&id, &query, &state).await {
        Ok(res) => {
            let (items, meta) = res;
            Ok(HttpResponse::Ok().json(ApiResponse::success(
                ApiCode::OperationSuccess,
                "Successful",
                ListResponseModel { items, meta },
                None,
            )))
        }
        Err(_) => Err(ApiError::NotFound),
    }
}
