use actix_web::{HttpRequest, HttpResponse, web};

use crate::{
    AppState,
    app::{
        institutions::services::init_institution,
        staffs::{
            models::{
                AddInitializerParams, AddStaffModel, AddStaffParams, SetupStaff, UpdateStaffModel,
                UpdateStaffParams, UpdateStaffStatusModel, UpdateStaffStatusParams,
            },
            services,
        },
    },
    utils::{
        errors::{ApiCode, ApiError, ApiResponse},
        gen_snow_ids::gen_string,
        models::{ListResponseModel, PathParamsModel, QueryModel, QueryParamsModel}, password::encrypt_password,
    },
};

pub async fn setup(
    _req: HttpRequest,
    payload: web::Json<AddInitializerParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let data = payload.into_inner();

    let institution_name = &data.institution_name;

    let code = gen_string(institution_name.len()).await;

    let institution = match init_institution(&data.institution_name, &code, &state).await {
        Ok(inst) => inst,
        Err(err) => return Err(ApiError::Unprocessable(err.to_string())),
    };

    let salt = uuid::Uuid::new_v4();
    let password = gen_string(14).await;

    let staff = SetupStaff {
        institution_id: institution.last_insert_id,
        first_name: data.first_name,
        last_name: data.last_name,
        phone_number: data.phone_number,
        email: data.email,
        password: encrypt_password(&password, &salt).await,
    };

    match services::init_staff(&staff, &state).await {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            {},
            None,
        ))),
        Err(err) => Err(ApiError::BadRequest(err.to_string())),
    }
}

pub async fn add_staff(
    _req: HttpRequest,
    payload: web::Json<AddStaffParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let data = payload.into_inner();

    let salt = uuid::Uuid::new_v4();
    let password = gen_string(14).await;

    let staff = AddStaffModel {
        institution_id: 84897473979,
        branch_id: Some(74737438737),
        first_name: data.first_name,
        last_name: data.last_name,
        phone_number: data.phone_number,
        email: data.email,
        gender: data.gender,
        nationality: data.nationality,
        job_title: data.job_title,
        hired_date: data.hired_date,
        password: encrypt_password(&password, &salt).await,
    };

    match services::save_staff(&staff, &state).await {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            {},
            None,
        ))),
        Err(err) => Err(ApiError::BadRequest(err.to_string())),
    }
}

pub async fn update_status(
    _req: HttpRequest,
    payload: web::Json<UpdateStaffStatusParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let data = payload.into_inner();

    let stat = UpdateStaffStatusModel {
        id: data
            .staff_id
            .parse::<i64>()
            .map_err(|_| ApiError::BadRequest("Invalid ID format".to_string()))?,
        employment_status: data.status,
    };

    match services::update_emp_status(&stat, &state).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            {},
            None,
        ))),
        Err(err) => Err(ApiError::BadRequest(err.to_string())),
    }
}

pub async fn staff_update(
    _req: HttpRequest,
    params: web::Path<PathParamsModel>,
    payload: web::Json<UpdateStaffParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let data = payload.into_inner();
    let path = params.into_inner();

    let branch_id = match data.branch_id {
        Some(val) => Some(
            val.parse::<i64>()
                .map_err(|_| ApiError::BadRequest("Invalid branch ID format".to_string()))?,
        ),
        None => None,
    };

    let update = UpdateStaffModel {
        branch_id,
        phone_number: data.phone_number,
        data_of_birth: data.birth_date,
        department: data.department,
        job_title: data.job_title,
    };

    let id = path
        .id
        .parse::<i64>()
        .map_err(|_| ApiError::BadRequest("Invalid staff ID format".to_string()))?;

    match services::update_staff(&id, &update, &state).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            {},
            None,
        ))),
        Err(err) => Err(ApiError::BadRequest(err.to_string())),
    }
}

pub async fn staff_details(
    _req: HttpRequest,
    params: web::Path<PathParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let path = params.into_inner();

    let id = path
        .id
        .parse::<i64>()
        .map_err(|_| ApiError::BadRequest("Invalid ID format".to_string()))?;

    match services::get_staff_details(&id, &state).await {
        Ok(res) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            res,
            None,
        ))),
        Err(_) => Err(ApiError::NotFound),
    }
}

pub async fn get_staffs(
    _req: HttpRequest,
    params: web::Path<PathParamsModel>,
    query: web::Query<QueryParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let query = query.into_inner();
    let path = params.into_inner();

    let query = QueryModel {
        size: query.size,
        page: query.page,
    };

    let id = path
        .id
        .parse::<i64>()
        .map_err(|_| ApiError::BadRequest("Invalid ID format".to_string()))?;

    match services::get_staff_list(&id, &query, &state).await {
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
