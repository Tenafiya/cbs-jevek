use actix_web::{HttpRequest, HttpResponse, web};
use validator::Validate;

use crate::{
    AppState,
    app::branches::{
        models::{AddBranchModel, AddBranchParams},
        services,
    },
    utils::{
        self,
        errors::{ApiCode, ApiError, ApiResponse},
        gen_snow_ids::id_parser,
        models::{ListResponseModel, PathParamsModel, QueryModel, QueryParamsModel},
    },
};

pub async fn add_branch(
    _req: HttpRequest,
    payload: web::Json<AddBranchParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let data = payload.into_inner();

    let institution_id = id_parser(&data.institution_id, "Institution Id").await?;

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
            ApiCode::ResourceCreated,
            "Successful",
            {},
        ))),
        Err(err) => Err(ApiError::BadRequest(err.to_string())),
    }
}

pub async fn get_branch_details(
    _req: HttpRequest,
    params: web::Path<PathParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    params
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let data = params.into_inner();

    let id = id_parser(&data.id, "Id").await?;

    match services::get_details(&id, &state).await {
        Ok(branch) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            branch,
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
    params
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let data = params.into_inner();

    let id = id_parser(&data.id, "Id").await?;

    query
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

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
            )))
        }
        Err(_) => Err(ApiError::NotFound),
    }
}
