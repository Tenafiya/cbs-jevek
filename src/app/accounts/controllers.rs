use actix_web::{HttpRequest, HttpResponse, web};
use validator::Validate;

use crate::{
    AppState,
    app::{
        account_charts,
        accounts::{
            models::{AddAccountModel, AddAccountParams},
            services,
        },
        branches, customers,
        staffs::models::StaffResponseModel,
    },
    utils::{
        errors::{ApiCode, ApiError, ApiResponse},
        gen_snow_ids,
        models::{CursorModel, CursorQueryParams, ListResponseModel, PathParamsModel},
    },
};

pub async fn add_customer_account(
    _req: HttpRequest,
    state: web::Data<AppState>,
    staff: web::ReqData<StaffResponseModel>,
    payload: web::Json<AddAccountParams>,
) -> Result<HttpResponse, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let data = payload.into_inner();

    let StaffResponseModel {
        institution_id,
        branch_id,
        ..
    } = staff.into_inner();

    let branch = branches::services::get_int_branch(branch_id, institution_id, &state)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    let customer_id = gen_snow_ids::id_parser(&data.customer_id, "Customer ID")?;
    let acc_type_id = gen_snow_ids::id_parser(&data.account_type_id, "Account Type ID")?;

    let customer = customers::services::get_details(customer_id, &state)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    account_charts::services::get_account_type(acc_type_id, &state)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    let code: i64 = branch
        .code
        .as_ref()
        .ok_or(ApiError::InternalServerError)?
        .parse::<i64>()
        .map_err(|_| ApiError::InternalServerError)?;

    if code == 0 {
        return Err(ApiError::InternalServerError);
    }

    let account_number = gen_snow_ids::generate_account_number(code, customer.id);

    let account_name = format!(
        "{} {}",
        customer.first_name.as_deref().unwrap_or("").to_uppercase(),
        customer.last_name.as_deref().unwrap_or("").to_uppercase()
    )
    .trim()
    .to_string();

    let acc_model = AddAccountModel {
        institution_id,
        customer_id: customer.id,
        account_type_id: acc_type_id,
        account_number,
        account_name,
    };

    match services::save_customer_acc(&acc_model, &state).await {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::success(
            ApiCode::ResourceCreated,
            "Successful",
            {},
        ))),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

pub async fn get_all_cus_accounts(
    _req: HttpRequest,
    state: web::Data<AppState>,
    staff: web::ReqData<StaffResponseModel>,
    query: web::Query<CursorQueryParams>,
) -> Result<HttpResponse, ApiError> {
    query
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let StaffResponseModel { institution_id, .. } = staff.into_inner();

    let query = query.into_inner();

    let cursor = query
        .cursor
        .map(|cur| gen_snow_ids::id_parser(&cur, "Cursor ID"))
        .transpose()?;

    let cursor = CursorModel {
        cursor,
        limit: query.limit,
    };

    match services::get_accounts(institution_id, &cursor, &state).await {
        Ok(res) => {
            let (items, meta) = res;

            Ok(HttpResponse::Ok().json(ApiResponse::success(
                ApiCode::OperationSuccess,
                "Successful",
                ListResponseModel { items, meta },
            )))
        }
        Err(_) => Err(ApiError::InternalServerError),
    }
}

pub async fn get_cus_account(
    _req: HttpRequest,
    state: web::Data<AppState>,
    params: web::Path<PathParamsModel>,
) -> Result<HttpResponse, ApiError> {
    params
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let path = params.into_inner();

    let id = gen_snow_ids::id_parser(&path.id, "Customer ID")?;

    match services::fetch_customer_acc(id, &state).await {
        Ok(res) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            res,
        ))),
        Err(_) => Err(ApiError::InternalServerError),
    }
}
