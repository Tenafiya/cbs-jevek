use actix_web::{HttpRequest, HttpResponse, web};
use serde_json::json;
use validator::Validate;

use crate::{
    AppState,
    app::{
        institutions::services::init_institution,
        staffs::{
            models::{
                AddInitializerParams, AddStaffModel, AddStaffParams, GetAuthModel, SetupStaff,
                SignInParams, UpdateStaffModel, UpdateStaffParams, UpdateStaffStatusModel,
                UpdateStaffStatusParams,
            },
            services,
        },
    },
    utils::{
        errors::{ApiCode, ApiError, ApiResponse},
        gen_snow_ids::{gen_string, id_parser},
        models::{ListResponseModel, PathParamsModel, QueryModel, QueryParamsModel},
        password::encrypt_password,
        tokens,
    },
};

pub async fn setup(
    _req: HttpRequest,
    payload: web::Json<AddInitializerParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

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
        salt,
    };

    match services::init_staff(&staff, &state).await {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::success(
            ApiCode::ResourceCreated,
            "Successful",
            json!({ "password": password }),
        ))),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

pub async fn add_staff(
    _req: HttpRequest,
    payload: web::Json<AddStaffParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let data = payload.into_inner();

    let salt = uuid::Uuid::new_v4();
    let password = gen_string(14).await;

    let staff = AddStaffModel {
        institution_id: id_parser(&data.institution_id, "Institution Id").await?,
        branch_id: Some(id_parser(&data.branch_id, "Branch Id").await?),
        first_name: data.first_name,
        last_name: data.last_name,
        phone_number: data.phone_number,
        email: data.email,
        gender: data.gender,
        nationality: data.nationality,
        job_title: data.job_title,
        hired_date: data.hired_date,
        password: encrypt_password(&password, &salt).await,
        salt,
    };

    match services::save_staff(&staff, &state).await {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::success(
            ApiCode::ResourceCreated,
            "Successful",
            {},
        ))),
        Err(err) => Err(ApiError::BadRequest(err.to_string())),
    }
}

pub async fn update_status(
    _req: HttpRequest,
    payload: web::Json<UpdateStaffStatusParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let data = payload.into_inner();

    let stat = UpdateStaffStatusModel {
        id: id_parser(&data.staff_id, "Staff Id").await?,
        employment_status: data.status,
    };

    match services::update_emp_status(&stat, &state).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            {},
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
    params
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let path = params.into_inner();

    let id = id_parser(&path.id, "Id").await?;

    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let data = payload.into_inner();

    let branch_id = match data.branch_id {
        Some(val) => Some(id_parser(&val, "Branch Id").await?),
        None => None,
    };

    let update = UpdateStaffModel {
        branch_id,
        phone_number: data.phone_number,
        date_of_birth: data.birth_date,
        department: data.department,
        job_title: data.job_title,
    };

    match services::update_staff(&id, &update, &state).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            {},
        ))),
        Err(err) => Err(ApiError::BadRequest(err.to_string())),
    }
}

pub async fn staff_details(
    _req: HttpRequest,
    params: web::Path<PathParamsModel>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    params
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let path = params.into_inner();

    let id = id_parser(&path.id, "Id").await?;

    match services::get_staff_details(&id, &state).await {
        Ok(res) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            res,
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
    params
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;
    query
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let query = query.into_inner();
    let path = params.into_inner();

    let id = id_parser(&path.id, "Id").await?;

    let query = QueryModel {
        size: query.size,
        page: query.page,
    };

    match services::get_staff_list(&id, &query, &state).await {
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

pub async fn signin(
    _req: HttpRequest,
    payload: web::Json<SignInParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let data = payload.into_inner();

    let signup = GetAuthModel {
        email: data.email,
        password: data.password,
    };

    match services::signin_auth(&signup, &state).await {
        Ok(user) => {
            let (token, exp) =
                tokens::create_jwt(&user.session.unwrap_or_default().into(), "NORMAL", &state)
                    .await;

            Ok(HttpResponse::Ok().json(ApiResponse::success(
                ApiCode::OperationSuccess,
                "Successful",
                json!({ "staff": user, "token": { "session": token, "expiry": exp } }),
            )))
        }
        Err(_) => {
            let email = signup.email;

            tokio::spawn(async move {
                if let Err(e) = services::update_failed_login(&email, &state).await {
                    tracing::error!(error = ?e, email = %email, "Failed to record login attempt");
                }
            });

            Err(ApiError::Unauthorized)
        }
    }
}
