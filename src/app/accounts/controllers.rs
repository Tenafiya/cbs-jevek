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
        customers,
        staffs::models::StaffResponseModel,
    },
    utils::{
        errors::{ApiCode, ApiError, ApiResponse},
        gen_snow_ids,
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

    let StaffResponseModel { institution_id, .. } = staff.into_inner();

    let customer_id = gen_snow_ids::id_parser(&data.customer_id, "Customer ID")?;
    let acc_type_id = gen_snow_ids::id_parser(&data.account_type_id, "Account Type ID")?;

    let customer = customers::services::get_details(customer_id, &state)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    account_charts::services::get_account_type(acc_type_id, &state)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

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
        account_number: "".to_string(),
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
