use actix_web::web;
use entity::sea_orm_active_enums::AmlRulesExecutionStage;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseBackend, DatabaseTransaction, DbErr,
    EntityTrait, FromQueryResult, InsertResult, QueryFilter, Statement,
};

use crate::{
    AppState,
    app::amls::{
        mapper::{
            AmlAlertFlat, AmlAlertRow, AmlCaseFlat, AmlCaseRow, AmlRule, AmlRuleFlat, AmlRuleRow,
        },
        models::{
            AmlActionsModel, AmlAlertsModel, AmlCaseNotesModel, AmlCasesModel, AmlExecutionModel,
            AmlRulesModel,
        },
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
        version: Set(data.version),
        effective_from: Set(data.effective_from),
        effective_to: Set(data.effective_to),
        execution_stage: Set(data.execution_stage),
        priority: Set(data.priority),
        is_enabled: Set(Some(true)),
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
    trn: &DatabaseTransaction,
) -> Result<InsertResult<entity::aml_cases::ActiveModel>, DbErr> {
    use entity::aml_cases::{ActiveModel, Entity};

    let (snowflake, slug) =
        gen_snow_ids::gen_snowflake_slug().map_err(|e| DbErr::Custom(e.to_string()))?;

    let data = model.clone();

    let case = ActiveModel {
        id: Set(snowflake),
        institution_id: Set(data.institution_id),
        case_number: Set(slug),
        title: Set(data.title),
        priority: Set(data.priority),
        assigned_investigator: Set(data.investigator),
        description: Set(data.desc),
        ..Default::default()
    };

    Entity::insert(case).exec(trn).await
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
    trn: &DatabaseTransaction,
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
        performedby: Set(data.performed_by),
        metadata: Set(data.metadata),
        ..Default::default()
    };

    Entity::insert(action).exec(trn).await
}

pub async fn save_aml_rule_execution(
    model: &AmlExecutionModel,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::aml_rule_executions::ActiveModel>, DbErr> {
    use entity::aml_rule_executions::{ActiveModel, Entity};

    let data = model.clone();

    let (snowflake, _) =
        gen_snow_ids::gen_snowflake_slug().map_err(|e| DbErr::Custom(e.to_string()))?;

    let exection = ActiveModel {
        id: Set(snowflake),
        institution_id: Set(data.institution_id),
        is_matched: Set(data.is_matched),
        risk_score: Set(data.risk_score),
        evaluation_details: Set(data.evaluation),
        execution_time_ms: Set(data.execution_ms),
        executed_at: Set(chrono::Utc::now().into()),
        ..Default::default()
    };

    Entity::insert(exection).exec(state.pgdb.get_ref()).await
}

pub async fn get_aml_rules(
    institution_id: i64,
    state: &web::Data<AppState>,
) -> Result<Vec<AmlRuleRow>, DbErr> {
    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        r#"
        SELECT
            ar.id,
            ar.institution_id,
            ar.rule_name,
            ar.rule_description,
            ar.rule_type::TEXT,
            ar.condition_logic,
            ar.action_on_trigger::TEXT,
            ar.execution_stage::TEXT,
            ar.is_enabled,
            ar.priority::TEXT,
            ar.stop_processing,
            ar.version,
            ar.effective_from,
            ar.effective_to,
            ar.created_at,
            ar.updated_at,

            creator.id AS creator_id,
            creator.employee_number AS creator_employee_number,
            creator.full_name AS creator_full_name,
            creator.first_name AS creator_first_name,
            creator.last_name AS creator_last_name,
            creator.phone_number AS creator_phone_number,
            creator.email_address AS creator_email_address,
            creator.job_title AS creator_job_title,
            creator.department AS creator_department,
            creator.employment_status::TEXT AS creator_employment_status,

            updater.id AS updater_id,
            updater.employee_number AS updater_employee_number,
            updater.full_name AS updater_full_name,
            updater.first_name AS updater_first_name,
            updater.last_name AS updater_last_name,
            updater.phone_number AS updater_phone_number,
            updater.email_address AS updater_email_address,
            updater.job_title AS updater_job_title,
            updater.department AS updater_department,
            updater.employment_status::TEXT AS updater_employment_status

        FROM aml_rules ar

        LEFT JOIN staff creator
            ON creator.id = ar.created_by
            AND creator.institution_id = ar.institution_id

        LEFT JOIN staff updater
            ON updater.id = ar.updated_by
            AND updater.institution_id = ar.institution_id

        WHERE ar.institution_id = $1
        AND ar.created_at >= NOW() - INTERVAL '24 hours';
        "#,
        vec![institution_id.into()],
    );

    AmlRuleFlat::find_by_statement(stmt)
        .all(state.pgdb.get_ref())
        .await
        .map(|rows| rows.into_iter().map(Into::into).collect())
}

pub async fn get_aml_cases(
    institution_id: i64,
    state: &web::Data<AppState>,
) -> Result<Vec<AmlCaseRow>, DbErr> {
    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        r#"
        SELECT
            ac.id,
            ac.institution_id,
            ac.case_number,
            ac.title,
            ac.description,
            ac.priority::TEXT,
            ac.status::TEXT,
            ac.resolution,
            ac.resolved_at,
            ac.created_at,
            ac.updated_at,

            -- Assigned investigator
            investigator.id AS assigned_investigator,
            investigator.employee_number AS investigator_employee_number,
            investigator.full_name AS investigator_full_name,
            investigator.first_name AS investigator_first_name,
            investigator.last_name AS investigator_last_name,
            investigator.phone_number AS investigator_phone_number,
            investigator.email_address AS investigator_email_address,
            investigator.job_title AS investigator_job_title,
            investigator.department AS investigator_department,
            investigator.employment_status::TEXT AS investigator_employment_status,

            -- Resolved by
            resolver.id AS resolved_by,
            resolver.employee_number AS resolved_by_employee_number,
            resolver.full_name AS resolved_by_full_name,
            resolver.first_name AS resolved_by_first_name,
            resolver.last_name AS resolved_by_last_name,
            resolver.phone_number AS resolved_by_phone_number,
            resolver.email_address AS resolved_by_email_address,
            resolver.job_title AS resolved_by_job_title,
            resolver.department AS resolved_by_department,
            resolver.employment_status::TEXT AS resolved_by_employment_status

        FROM aml_cases ac

        LEFT JOIN staff investigator
            ON investigator.id = ac.assigned_investigator
            AND investigator.institution_id = ac.institution_id

        LEFT JOIN staff resolver
            ON resolver.id = ac.resolved_by
            AND resolver.institution_id = ac.institution_id

        WHERE ac.institution_id = $1
        AND ac.created_at >= NOW() - INTERVAL '24 hours';
        "#,
        vec![institution_id.into()],
    );

    AmlCaseFlat::find_by_statement(stmt)
        .all(state.pgdb.get_ref())
        .await
        .map(|rows| rows.into_iter().map(Into::into).collect())
}

pub async fn get_aml_alerts(
    institution_id: i64,
    state: &web::Data<AppState>,
) -> Result<Vec<AmlAlertRow>, DbErr> {
    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        r#"
        SELECT
            aa.id,
            aa.institution_id,
            aa.risk_level::TEXT,
            aa.alert_type::TEXT,
            aa.alert_details,
            aa.risk_breakdown,
            aa.risk_score,
            aa.status::TEXT,
            aa.detected_at,
            aa.created_at,
            aa.updated_at,

            ar.id AS rule_id,
            ar.rule_name,
            ar.rule_description,
            ar.rule_type::TEXT,
            ar.condition_logic,
            ar.action_on_trigger::TEXT AS action_logic,
            ar.is_enabled,

            rule_creator.id AS rule_creator_id,
            rule_creator.full_name AS rule_creator_full_name,

            rule_updater.id AS rule_updater_id,
            rule_updater.full_name AS rule_updater_full_name,

            ac.id AS case_id,
            ac.case_number,
            ac.title AS case_title,
            ac.description AS case_description,
            ac.priority::TEXT AS case_priority,
            ac.status::TEXT AS case_status,

            c.id AS customer_id,
            c.customer_type::TEXT,
            c.customer_number,
            c.first_name AS customer_first_name,
            c.last_name AS customer_last_name,

            a.id AS account_id,
            a.account_number AS acccount_number,
            a.account_name,
            a.currency,
            a.current_balance,
            a.available_balance,
            a.ledger_balance,
            a.hold_balance,

            t.id AS transaction_id,
            t.transaction_reference,
            t.transaction_group_id,
            t.transaction_type::TEXT,
            t.transaction_category::TEXT,
            t.amount AS transaction_amount,
            t.currency AS transaction_currency,
            t.status::TEXT AS transaction_status,
            t.posted_at AS transaction_posted_at,
            t.completed_at AS transaction_completed_at,
            t.failed_at AS transaction_failed_at

        FROM aml_alerts aa

        LEFT JOIN aml_rules ar
            ON ar.id = aa.rule_id
            AND ar.institution_id = aa.institution_id

        LEFT JOIN staff rule_creator
            ON rule_creator.id = ar.creator_id
            AND rule_creator.institution_id = ar.institution_id

        LEFT JOIN staff rule_updater
            ON rule_updater.id = ar.updater_id
            AND rule_updater.institution_id = ar.institution_id

        LEFT JOIN aml_cases ac
            ON ac.id = aa.case_id
            AND ac.institution_id = aa.institution_id

        LEFT JOIN customers c
            ON c.id = aa.customer_id
            AND c.institution_id = aa.institution_id

        LEFT JOIN accounts a
            ON a.id = aa.account_id
            AND a.institution_id = aa.institution_id

        LEFT JOIN transactions t
            ON t.id = aa.transaction_id
            AND t.institution_id = aa.institution_id

        WHERE aa.institution_id = $1
        AND aa.created_at >= NOW() - INTERVAL '24 hours';
        "#,
        vec![institution_id.into()],
    );

    let rows = AmlAlertFlat::find_by_statement(stmt)
        .all(state.pgdb.get_ref())
        .await?;

    rows.into_iter()
        .map(AmlAlertRow::try_from)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| DbErr::Custom("Data parsing failed".into()))
}

pub async fn fetch_execution_rules(
    institution_id: i64,
    stage: AmlRulesExecutionStage,
    state: &web::Data<AppState>,
) -> Result<Vec<AmlRule>, DbErr> {
    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        r#"
        SELECT
            id,
            institution_id,
            rule_name,
            rule_description,
            rule_type::TEXT,
            execution_stage::TEXT,
            condition_logic,
            action_on_trigger::TEXT,
            is_enabled,
            priority::TEXT,
            stop_processing,
            version,
            effective_from,
            effective_to
        FROM aml_rules
        WHERE institution_id = $1
          AND is_enabled = TRUE
          AND execution_stage = $2::aml_rules_execution_stage
          AND (effective_from IS NULL OR effective_from <= NOW())
          AND (effective_to IS NULL OR effective_to > NOW())
        ORDER BY priority ASC, id ASC;
        "#,
        vec![institution_id.into(), stage.into()],
    );

    AmlRule::find_by_statement(stmt)
        .all(state.pgdb.get_ref())
        .await
}

pub async fn toggle_aml_rule(
    institution_id: i64,
    rule_id: i64,
    state: &web::Data<AppState>,
) -> Result<(), DbErr> {
    use entity::aml_rules::{ActiveModel, Column, Entity};

    let rule = Entity::find_by_id(rule_id)
        .filter(Column::InstitutionId.eq(institution_id))
        .one(state.pgdb.get_ref())
        .await?
        .ok_or_else(|| DbErr::Custom("Rule not found".to_string()))?;

    let is_enabled = rule.is_enabled.unwrap_or(false);

    let mut active_rule: ActiveModel = rule.into();

    active_rule.is_enabled = Set(Some(!is_enabled));

    active_rule.updated_at = Set(Some(chrono::Utc::now().into()));

    ActiveModelTrait::update(active_rule, state.pgdb.get_ref()).await?;

    Ok(())
}

pub async fn get_action_list(
    created: Vec<i64>,
    state: &web::Data<AppState>,
) -> Result<Vec<entity::aml_actions::Model>, DbErr> {
    use entity::aml_actions::{Column, Entity};

    Entity::find()
        .filter(Column::Id.is_in(created))
        .all(state.pgdb.get_ref())
        .await
}
