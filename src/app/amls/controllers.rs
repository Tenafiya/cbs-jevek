use actix_web::{HttpRequest, HttpResponse, web};
use validator::Validate;

use crate::{
    AppState,
    app::{
        amls::{
            models::{
                AmlActionsModel, AmlCaseNotesModel, AmlRulesModel, CreateAmlActionParams,
                CreateAmlCaseNodes, CreateAmlRulesParams,
            },
            services,
        },
        staffs::models::StaffResponseModel,
    },
    utils::{
        errors::{ApiCode, ApiError, ApiResponse},
        gen_snow_ids,
    },
};

pub async fn create_new_rule(
    _req: HttpRequest,
    state: web::Data<AppState>,
    staff: web::ReqData<StaffResponseModel>,
    payload: web::Json<CreateAmlRulesParams>,
) -> Result<HttpResponse, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let data = payload.into_inner();

    let logic = serde_json::to_value(&data.condition_logic)
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let rule = AmlRulesModel {
        institution_id: staff.institution_id,
        rule_name: data.rule_name,
        rule_type: data.rule_type,
        condition_logic: logic,
        trigger_action: data.trigger_action,
        desc: data.description,
        priority: data.priority,
        version: data.version,
        effective_from: data.effective_dates.as_ref().map(|d| d.effective_from),
        effective_to: data.effective_dates.as_ref().map(|d| d.effective_to),
    };

    match services::save_aml(&rule, &state).await {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::success(
            ApiCode::ResourceCreated,
            "Successful",
            {},
        ))),
        Err(e) => {
            tracing::error!(error = ?e, "Failed to create Aml Rule");
            Err(ApiError::InternalServerError)
        }
    }
}

pub async fn create_new_case_note(
    _req: HttpRequest,
    state: web::Data<AppState>,
    staff: web::ReqData<StaffResponseModel>,
    payload: web::Json<CreateAmlCaseNodes>,
) -> Result<HttpResponse, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let data = payload.into_inner();

    let note = AmlCaseNotesModel {
        case_id: gen_snow_ids::id_parser(&data.case_id, "Case ID")?,
        investigator_id: staff.id,
        note: data.notes,
    };

    match services::save_aml_case_notes(&note, &state).await {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::success(
            ApiCode::ResourceCreated,
            "Successful",
            {},
        ))),
        Err(e) => {
            tracing::error!(error = ?e, "Failed to create Aml Case Note");
            Err(ApiError::InternalServerError)
        }
    }
}

pub async fn create_aml_action(
    _req: HttpRequest,
    state: web::Data<AppState>,
    staff: web::ReqData<StaffResponseModel>,
    payload: web::Json<CreateAmlActionParams>,
) -> Result<HttpResponse, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let StaffResponseModel {
        id, institution_id, ..
    } = staff.into_inner();

    let data = payload.into_inner();

    let action = AmlActionsModel {
        case_id: gen_snow_ids::id_parser(&data.case_id, "Case ID")?,
        alert_id: gen_snow_ids::id_parser(&data.alert_id, "Alert ID")?,
        action_type: data.action_type,
        metadata: data.metadata,
        institution_id: institution_id,
        performed_by: id,
    };

    match services::save_aml_action(&action, &state).await {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse::success(
            ApiCode::ResourceCreated,
            "Successful",
            {},
        ))),
        Err(e) => {
            tracing::error!(error = ?e, "Failed to create Aml Action");
            Err(ApiError::InternalServerError)
        }
    }
}
