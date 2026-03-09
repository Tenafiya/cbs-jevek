use actix_web::{HttpRequest, HttpResponse, web};
use validator::Validate;

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
        gen_snow_ids::id_parser,
        models::{ListResponseModel, PathParamsModel, QueryModel, QueryParamsModel},
    },
};

pub async fn add_institution(
    _req: HttpRequest,
    payload: web::Json<AddInstitutionParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let data = payload.into_inner();

    let country_id = id_parser(&data.country_id, "Country Id").await?;

    let institution = AddInstitutionModel {
        name: data.name,
        code: utils::gen_snow_ids::get_code(6).await,
        country: country_id,
        license_num: data.license_num,
        regulation_num: data.regulation_num,
        city: data.city,
        zip_code: data.zip_code,
        state: data.state,
        date_format: data.date_format,
        date_time_format: data.date_time_format,
        address: data.address,
        postal_address: data.postal_address,
    };

    match services::save_institution(&institution, &state).await {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::success(
            ApiCode::ResourceCreated,
            "Institution Created",
            {},
        ))),
        Err(err) => Err(ApiError::BadRequest(err.to_string())),
    }
}

pub async fn get_institution(
    _req: HttpRequest,
    params: web::Path<PathParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    params
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let data = params.into_inner();

    let id = id_parser(&data.id, "Id").await?;

    match services::get_one(&id, &state).await {
        Ok(ins) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            ins,
        ))),
        Err(_) => Err(ApiError::NotFound),
    }
}

pub async fn get_institutions(
    _req: HttpRequest,
    query: web::Query<QueryParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    query
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let query = QueryModel {
        size: query.size,
        page: query.page,
    };

    match services::get_all(&query, &state).await {
        Ok(insts) => {
            let (items, meta) = insts;
            Ok(HttpResponse::Ok().json(ApiResponse::success(
                ApiCode::OperationSuccess,
                "Successful",
                ListResponseModel { items, meta },
            )))
        }
        Err(_) => Err(ApiError::NotFound),
    }
}

pub async fn update_institution(
    _req: HttpRequest,
    payload: web::Json<UpdateInstitutionParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let data = payload.into_inner();

    let id = id_parser(&data.id, "Id").await?;

    let update_model = UpdateInstitutionModel {
        name: data.name,
        timezone: data.timezone,
        license_num: data.license_num,
        regulation_num: data.regulation_num,
    };

    match services::update(&id, &update_model, &state).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            {},
        ))),
        Err(_) => Err(ApiError::InternalServerError),
    }
}
