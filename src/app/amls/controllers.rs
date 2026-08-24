use actix_web::{HttpRequest, HttpResponse, web};
use validator::Validate;

use crate::{
    AppState,
    app::{
        amls::{
            models::{
                AmlActionsModel, AmlCaseNotesModel, AmlRulesModel, ConditionField, ConditionParams,
                CreateAmlActionParams, CreateAmlCaseNodes, CreateAmlRulesParams, FIELD_DEFINITIONS,
                FieldDefinition,
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

pub fn get_field_definition(field: ConditionField) -> Option<&'static FieldDefinition> {
    FIELD_DEFINITIONS
        .iter()
        .find(|definition| definition.field == field)
}

pub fn validate_condition(conditions: &[ConditionParams]) -> Result<(), ApiError> {
    for condition in conditions {
        let definition = get_field_definition(condition.field).ok_or_else(|| {
            ApiError::BadRequest(format!("Unknown condition field: {:?}", condition.field))
        })?;

        if !definition.allowed_operators.contains(&condition.operator) {
            return Err(ApiError::BadRequest(format!(
                "Operator {:?} is not allowed for field {:?}",
                condition.operator, condition.field
            )));
        }
    }

    Ok(())
}

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

    for condition in &data.condition_logic {
        validate_condition(&condition.conditions)?;
    }

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
        execution_stage: data.execution_stage,
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

pub async fn fetch_aml_rules(
    _req: HttpRequest,
    state: web::Data<AppState>,
    staff: web::ReqData<StaffResponseModel>,
) -> Result<HttpResponse, ApiError> {
    match services::get_aml_rules(staff.institution_id, &state).await {
        Ok(res) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            res,
        ))),
        Err(e) => {
            tracing::error!(error = ?e, "Failed to fetch Aml Rules");
            Err(ApiError::InternalServerError)
        }
    }
}

pub async fn fetch_aml_cases(
    _req: HttpRequest,
    state: web::Data<AppState>,
    staff: web::ReqData<StaffResponseModel>,
) -> Result<HttpResponse, ApiError> {
    match services::get_aml_cases(staff.institution_id, &state).await {
        Ok(res) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            res,
        ))),
        Err(e) => {
            tracing::error!(error = ?e, "Failed to fetch Aml Cases");
            Err(ApiError::InternalServerError)
        }
    }
}

pub async fn fetch_aml_alerts(
    _req: HttpRequest,
    state: web::Data<AppState>,
    staff: web::ReqData<StaffResponseModel>,
) -> Result<HttpResponse, ApiError> {
    match services::get_aml_alerts(staff.institution_id, &state).await {
        Ok(res) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Successful",
            res,
        ))),
        Err(e) => {
            tracing::error!(error = ?e, "Failed to fetch Aml Alerts");
            Err(ApiError::InternalServerError)
        }
    }
}
