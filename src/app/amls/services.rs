use actix_web::web;
use sea_orm::{ActiveValue::Set, DbErr, EntityTrait, InsertResult};

use crate::{
    AppState,
    app::amls::models::{
        AmlActionsModel, AmlAlertsModel, AmlCaseNotesModel, AmlCasesModel, AmlRulesModel,
    },
    utils::gen_snow_ids,
};

pub async fn save_aml(
    model: &AmlRulesModel,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::aml_rules::ActiveModel>, DbErr> {
    use entity::aml_rules::{ActiveModel, Entity};

    let (snowflake, _) =
        gen_snow_ids::gen_snowflake_slug().map_err(|e| DbErr::Custom(e.to_string()))?;

    let data = model.clone();

    let rule = ActiveModel {
        id: Set(snowflake),
        institution_id: Set(data.institution_id),
        rule_name: Set(data.rule_name),
        rule_type: Set(data.rule_type),
        condition_logic: Set(data.condition_logic),
        action_on_trigger: Set(data.trigger_action),
        rule_description: Set(data.desc),
        priority: Set(data.priority),
        version: Set(data.version),
        effective_from: Set(data.effective_from),
        effective_to: Set(data.effective_to),
        ..Default::default()
    };

    Entity::insert(rule).exec(state.pgdb.get_ref()).await
}

pub async fn save_aml_alerts(
    model: &AmlAlertsModel,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::aml_alerts::ActiveModel>, DbErr> {
    use entity::aml_alerts::{ActiveModel, Entity};

    let (snowflake, _) =
        gen_snow_ids::gen_snowflake_slug().map_err(|e| DbErr::Custom(e.to_string()))?;

    let data = model.clone();

    let alert = ActiveModel {
        id: Set(snowflake),
        institution_id: Set(data.institution_id),
        rule_id: Set(Some(data.rule_id)),
        alert_type: Set(data.alert_type),
        risk_level: Set(Some(data.risk_level)),
        alert_details: Set(data.alert_details),
        risk_breakdown: Set(data.risk_breakdown),
        ..Default::default()
    };

    Entity::insert(alert).exec(state.pgdb.get_ref()).await
}

pub async fn save_aml_cases(
    model: &AmlCasesModel,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::aml_cases::ActiveModel>, DbErr> {
    use entity::aml_cases::{ActiveModel, Entity};

    let (snowflake, _) =
        gen_snow_ids::gen_snowflake_slug().map_err(|e| DbErr::Custom(e.to_string()))?;

    let data = model.clone();

    let case = ActiveModel {
        id: Set(snowflake),
        institution_id: Set(data.institution_id),
        case_number: Set(data.case_number),
        title: Set(data.title),
        priority: Set(data.priority),
        assigned_investigator: Set(data.investigator),
        description: Set(data.desc),
        ..Default::default()
    };

    Entity::insert(case).exec(state.pgdb.get_ref()).await
}

pub async fn save_aml_case_notes(
    model: &AmlCaseNotesModel,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::aml_case_notes::ActiveModel>, DbErr> {
    use entity::aml_case_notes::{ActiveModel, Entity};

    let (snowflake, _) =
        gen_snow_ids::gen_snowflake_slug().map_err(|e| DbErr::Custom(e.to_string()))?;

    let data = model.clone();

    let note = ActiveModel {
        id: Set(snowflake),
        case_id: Set(data.case_id),
        investigator_id: Set(data.investigator_id),
        note: Set(data.note),
        ..Default::default()
    };

    Entity::insert(note).exec(state.pgdb.get_ref()).await
}

pub async fn save_aml_action(
    model: &AmlActionsModel,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::aml_actions::ActiveModel>, DbErr> {
    use entity::aml_actions::{ActiveModel, Entity};

    let (snowflake, _) =
        gen_snow_ids::gen_snowflake_slug().map_err(|e| DbErr::Custom(e.to_string()))?;

    let data = model.clone();

    let action = ActiveModel {
        id: Set(snowflake),
        institution_id: Set(data.institution_id),
        case_id: Set(data.case_id),
        alert_id: Set(data.alert_id),
        action_type: Set(Some(data.action_type)),
        performedby: Set(Some(data.performedby)),
        metadata: Set(data.metadata),
        ..Default::default()
    };

    Entity::insert(action).exec(state.pgdb.get_ref()).await
}
