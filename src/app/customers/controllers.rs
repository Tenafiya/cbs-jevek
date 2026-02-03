use actix_web::{HttpRequest, HttpResponse, web};

use crate::{
    AppState,
    app::customers::{
        models::{
            AddAddressModel, AddAddressParams, AddCustomerModel, AddCustomerParams, AddNextModel,
            AddOccupationModel, AddOccupationParams, NextOfKinParams,
        },
        services,
    },
    utils::{
        errors::{ApiCode, ApiError, ApiResponse},
        models::{ListResponseModel, PathParamsModel, QueryModel, QueryParamsModel},
    },
};

pub async fn add_customer(
    _req: HttpRequest,
    payload: web::Json<AddCustomerParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let data = payload.into_inner();

    let customer = AddCustomerModel {
        institution_id: 049430940300,
        customer_type: data.customer_type,
        customer_num: "ueu2389208".into(),
        first_name: data.first_name,
        last_name: data.last_name,
        middle_name: data.middle_name,
        birth_date: data.birth_date,
        gender: data.gender,
        nationality: data.nationality,
        phone_country_code: data.phone_country_code,
        phone_number: data.phone_number,
        email: data.email,
        created_by: 9849898989898,
    };

    match services::save_customer(&customer, &state).await {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            {},
            None,
        ))),
        Err(err) => Err(ApiError::BadRequest(err.to_string())),
    }
}

pub async fn save_address(
    _req: HttpRequest,
    params: web::Path<PathParamsModel>,
    payload: web::Json<AddAddressParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let data = payload.into_inner();
    let path = params.into_inner();

    let id = path
        .id
        .parse::<i64>()
        .map_err(|_| ApiError::BadRequest("Invalid ID format".to_string()))?;

    let address = AddAddressModel {
        residential_address: Some(data.residential_address),
        postal_address: Some(data.postal_address),
        city: Some(data.city),
        state: Some(data.state),
        country_id: Some(data.country_id.parse().unwrap_or(0)),
    };

    match services::add_address(&id, &address, &state).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            {},
            None,
        ))),
        Err(err) => Err(ApiError::BadRequest(err.to_string())),
    }
}

pub async fn save_occupation(
    _req: HttpRequest,
    params: web::Path<PathParamsModel>,
    payload: web::Json<AddOccupationParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let data = payload.into_inner();
    let path = params.into_inner();

    let id = path
        .id
        .parse::<i64>()
        .map_err(|_| ApiError::BadRequest("Invalid ID format".to_string()))?;

    let occupation = AddOccupationModel {
        occupation: data.occupation,
        employer_name: data.employer_name,
        income_source: data.income_source,
        monthly_income: data.monthly_income,
    };

    match services::add_occupation(&id, &occupation, &state).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            {},
            None,
        ))),
        Err(err) => Err(ApiError::BadRequest(err.to_string())),
    }
}

pub async fn save_kin(
    _req: HttpRequest,
    params: web::Path<PathParamsModel>,
    payload: web::Json<NextOfKinParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let data = payload.into_inner();
    let path = params.into_inner();

    let id = path
        .id
        .parse::<i64>()
        .map_err(|_| ApiError::BadRequest("Invalid ID format".to_string()))?;

    let next_details = AddNextModel {
        next_of_kin: data.next_of_kin,
        pep_details: data.pep_details,
    };

    match services::add_next_details(&id, &next_details, &state).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            {},
            None,
        ))),
        Err(err) => Err(ApiError::BadRequest(err.to_string())),
    }
}

pub async fn email_verification(
    _req: HttpRequest,
    params: web::Path<PathParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let path = params.into_inner();

    let id = path
        .id
        .parse::<i64>()
        .map_err(|_| ApiError::BadRequest("Invalid ID format".to_string()))?;

    match services::verify_email(&id, &state).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            {},
            None,
        ))),
        Err(err) => Err(ApiError::BadRequest(err.to_string())),
    }
}

pub async fn sms_verification(
    _req: HttpRequest,
    params: web::Path<PathParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let path = params.into_inner();

    let id = path
        .id
        .parse::<i64>()
        .map_err(|_| ApiError::BadRequest("Invalid ID format".to_string()))?;

    match services::verify_phone(&id, &state).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            {},
            None,
        ))),
        Err(err) => Err(ApiError::BadRequest(err.to_string())),
    }
}

pub async fn customer_details(
    _req: HttpRequest,
    params: web::Path<PathParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let path = params.into_inner();

    let id = path
        .id
        .parse::<i64>()
        .map_err(|_| ApiError::BadRequest("Invalid ID format".to_string()))?;

    match services::get_details(&id, &state).await {
        Ok(details) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            details,
            None,
        ))),
        Err(_) => Err(ApiError::NotFound),
    }
}

pub async fn all_customers(
    _req: HttpRequest,
    query: web::Query<QueryParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let query_data = query.into_inner();

    // this id would come from token
    let id = 93023989384989389;

    let query = QueryModel {
        size: query_data.size,
        page: query_data.page,
    };

    match services::get_customers(&id, &query, &state).await {
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

pub async fn update_sanctions(
    _req: HttpRequest,
    params: web::Path<PathParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let path = params.into_inner();

    let id = path
        .id
        .parse::<i64>()
        .map_err(|_| ApiError::BadRequest("Invalid ID format".to_string()))?;

    match services::update_sanctions(&id, &"BOG".to_string(), &state).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            {},
            None,
        ))),
        Err(err) => Err(ApiError::BadRequest(err.to_string())),
    }
}

pub async fn verify_customer(
    _req: HttpRequest,
    params: web::Path<PathParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let path = params.into_inner();

    let id = path
        .id
        .parse::<i64>()
        .map_err(|_| ApiError::BadRequest("Invalid ID format".to_string()))?;

    match services::customer_verify(&id, &898989898989, &state).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            {},
            None,
        ))),
        Err(err) => Err(ApiError::BadRequest(err.to_string())),
    }
}

pub async fn delete_customer(
    _req: HttpRequest,
    params: web::Path<PathParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let path = params.into_inner();

    let id = path
        .id
        .parse::<i64>()
        .map_err(|_| ApiError::BadRequest("Invalid ID format".to_string()))?;

    match services::customer_delete(&id, &state).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            {},
            None,
        ))),
        Err(err) => Err(ApiError::BadRequest(err.to_string())),
    }
}
