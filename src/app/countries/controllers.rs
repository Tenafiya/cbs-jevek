use actix_web::{HttpRequest, HttpResponse, web};

use crate::{
    AppState,
    app::countries::{
        models::{AddCountryModel, AddCountryParamsModel, OperationModel},
        services,
    },
    utils::{
        errors::{ApiCode, ApiError, ApiResponse},
        models::PathParamsModel,
    },
};

pub async fn add_country(
    _req: HttpRequest,
    payload: web::Json<AddCountryParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let data = payload.into_inner();
    let country = AddCountryModel {
        name: data.name,
        official_name: data.official_name,
        capital_city: data.capital_city,
        call_code: data.call_code,
        iso_code: data.iso_code,
        currency: data.currency,
        flag_url: data.flag_url,
        more_data: None,
    };

    match services::save_country(&country, &state).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Country Created",
            {},
            None,
        ))),
        Err(err) => Err(ApiError::BadRequest(err.to_string())),
    }
}

pub async fn get_countries(
    _req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    match services::get_all(&state).await {
        Ok(countries) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            countries,
            None,
        ))),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

pub async fn get_country(
    _req: HttpRequest,
    params: web::Path<PathParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let data = params.into_inner();

    match services::get_one(&data.id, &state).await {
        Ok(country) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            country,
            None,
        ))),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

pub async fn operate_country(
    _req: HttpRequest,
    params: web::Path<OperationModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let data = params.into_inner();

    match services::operate_country(&data.id, &data.operation, &state).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            {},
            None,
        ))),
        Err(err) => Err(ApiError::BadRequest(err.to_string())),
    }
}
