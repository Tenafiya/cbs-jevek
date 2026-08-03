use actix_web::{HttpRequest, HttpResponse, web};
use validator::Validate;

use crate::{
    AppState,
    app::{
        account_charts,
        accounts::{
            models::{
                AddAccountLimitModel, AddAccountLimitParams, AddAccountLinkModel,
                AddAccountLinkParams, AddAccountModel, AddAccountParams,
            },
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
        .map_err(|e| {
            tracing::error!(error = ?e, "failed to get branch");
            ApiError::InternalServerError
        })?;

    let customer_id = gen_snow_ids::id_parser(&data.customer_id, "Customer ID")?;
    let acc_type_id = gen_snow_ids::id_parser(&data.account_type_id, "Account Type ID")?;

    let customer = customers::services::get_details(customer_id, &state)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "failed to get customer");
            ApiError::InternalServerError
        })?;

    account_charts::services::get_account_type(acc_type_id, &state)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "account type error");
            ApiError::InternalServerError
        })?;

    match services::is_customer_subscribed(customer_id, acc_type_id, &state).await {
        Ok(Some(_)) => {
            return Err(ApiError::Conflict(
                "Customer already has a subscription for this account type".into(),
            ));
        }
        Ok(None) => {}
        Err(e) => {
            tracing::error!(
                error = ?e,
                customer_id = %customer_id,
                account_type_id = %acc_type_id,
                "Failed to check customer subscription"
            );

            return Err(ApiError::InternalServerError);
        }
    }

    let code: i64 = branch
        .code
        .as_ref()
        .ok_or({
            tracing::error!("failed to flatten code");
            ApiError::InternalServerError
        })?
        .parse::<i64>()
        .map_err(|e| {
            tracing::error!(error = ?e, "failed to parse code");
            ApiError::InternalServerError
        })?;

    if code == 0 {
        tracing::error!("account code is 0");
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
        Err(e) => {
            tracing::error!(error = ?e, "failed to add customer account");
            Err(ApiError::InternalServerError)
        }
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
        Err(e) => {
            tracing::error!(error = ?e, "failed to get account list");
            Err(ApiError::NotFound)
        }
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
        Err(e) => {
            tracing::error!(error = ?e, "failed to customer account");
            Err(ApiError::NotFound)
        }
    }
}

pub async fn add_account_links(
    _req: HttpRequest,
    state: web::Data<AppState>,
    staff: web::ReqData<StaffResponseModel>,
    payload: web::Json<AddAccountLinkParams>,
) -> Result<HttpResponse, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let data = payload.into_inner();

    let StaffResponseModel { institution_id, .. } = staff.into_inner();

    let acc_link = AddAccountLinkModel {
        institution_id,
        prim_account_id: gen_snow_ids::id_parser(&data.prim_account_id, "Primary Account Id")?,
        link_account_id: gen_snow_ids::id_parser(&data.link_account_id, "Link Account Id")?,
        link_type: data.link_type,
        relationship: data.relationship,
        authorized_limit: data.authorized_limit,
    };

    match services::add_acc_links(&acc_link, &state).await {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::success(
            ApiCode::ResourceCreated,
            "Successful",
            {},
        ))),
        Err(e) => {
            tracing::error!(error = ?e, "Failed to link accounts");
            Err(ApiError::InternalServerError)
        }
    }
}

pub async fn add_account_limit(
    _req: HttpRequest,
    state: web::Data<AppState>,
    payload: web::Json<AddAccountLimitParams>,
) -> Result<HttpResponse, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let data = payload.into_inner();

    let acc_limit = AddAccountLimitModel {
        account_id: gen_snow_ids::id_parser(&data.account_id, "Account Id")?,
        limit_type: data.limit_type,
        limit_unit: data.limit_unit,
        limit_value: data.limit_value,
        current_value: data.current_value,
        effective_from: data.effective_from.to_utc(),
        effective_to: data.effective_to.to_utc(),
    };

    match services::add_acc_limits(&acc_limit, &state).await {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::success(
            ApiCode::ResourceCreated,
            "Successful",
            {},
        ))),
        Err(e) => {
            tracing::error!(error = ?e, "Failed to add account limit");
            Err(ApiError::InternalServerError)
        }
    }
}
