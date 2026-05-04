use actix_web::web;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, Condition, DatabaseBackend, DbErr, EntityTrait, FromQueryResult,
    InsertResult, QueryFilter, Statement,
};

use crate::{
    AppState,
    app::account_charts::models::{
        AccountCategoryResponseModel, AccountTypeFlat, AccountTypeRow, AddAccountCategoryModel,
        AddAccountChartModel, AddAccountTypeModel, ChartOfAccountResponseModel,
    },
    utils::gen_snow_ids,
};

pub async fn save_acc_chart(
    model: &AddAccountChartModel,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::chart_of_accounts::ActiveModel>, DbErr> {
    let (snowflake, _) = gen_snow_ids::gen_snowflake_slug()
        .map_err(|_| DbErr::Custom("Failed to generate ID's".to_string()))?;

    let data = model.clone();

    let chart = entity::chart_of_accounts::ActiveModel {
        id: Set(snowflake),
        account_code: Set(Some(gen_snow_ids::gen_string(13).await)),
        account_name: Set(Some(data.acc_name)),
        account_type: Set(Some(data.acc_code)),
        parent_account_id: Set(data.parent_acc_id),
        is_system_account: Set(Some(data.is_system_acc)),
        ..Default::default()
    };

    entity::chart_of_accounts::Entity::insert(chart)
        .exec(state.pgdb.get_ref())
        .await
}

pub async fn get_charts(
    state: &web::Data<AppState>,
) -> Result<Vec<ChartOfAccountResponseModel>, DbErr> {
    let charts = entity::chart_of_accounts::Entity::find()
        .into_model::<ChartOfAccountResponseModel>()
        .all(state.pgdb.get_ref())
        .await;

    charts
}

pub async fn save_account_category(
    model: &AddAccountCategoryModel,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::account_categories::ActiveModel>, DbErr> {
    let (snowflake, _) = gen_snow_ids::gen_snowflake_slug()
        .map_err(|_| DbErr::Custom("Failed to generate ID's".to_string()))?;

    let data = model.clone();

    let cat = entity::account_categories::ActiveModel {
        id: Set(snowflake),
        institution_id: Set(data.institution_id),
        category_type: Set(data.category_type),
        description: Set(data.description),
        name: Set(data.name),
        ..Default::default()
    };

    entity::account_categories::Entity::insert(cat)
        .exec(state.pgdb.get_ref())
        .await
}

pub async fn get_account_categories(
    id: i64,
    state: &web::Data<AppState>,
) -> Result<Vec<AccountCategoryResponseModel>, DbErr> {
    let cats = entity::account_categories::Entity::find()
        .filter(
            Condition::all()
                .add(entity::account_categories::Column::InstitutionId.eq(id))
                .add(entity::account_categories::Column::IsActive.eq(true)),
        )
        .into_model::<AccountCategoryResponseModel>()
        .all(state.pgdb.get_ref())
        .await;

    cats
}

pub async fn save_acc_type(
    model: &AddAccountTypeModel,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::account_types::ActiveModel>, DbErr> {
    let (snowflake, _) = gen_snow_ids::gen_snowflake_slug()
        .map_err(|_| DbErr::Custom("Failed to generate ID's".to_string()))?;

    let data = model.clone();

    let acc_type = entity::account_types::ActiveModel {
        id: Set(snowflake),
        institution_id: Set(data.institution_id),
        category_id: Set(data.category_id),
        name: Set(data.name),
        code: Set(data.code),
        description: Set(data.description),
        currency: Set(data.currency),
        minimum_balance: Set(Some(data.min_balance)),
        maximum_balance: Set(Some(data.max_balance)),
        interest_rate: Set(Some(data.interest_rate)),
        interest_rate_calc_method: Set(Some(data.interest_rate_calc)),
        interest_payout_frequency: Set(Some(data.interest_payout_freq)),
        is_overdraft_allowable: Set(Some(data.is_overdraft_allowable)),
        overdraft_limit: Set(data.overdraft_limit),
        overdraft_interest_rate: Set(Some(data.overdraft_interest_rate)),
        dormancy_period_days: Set(data.dormancy_period),
        maintenance_fee: Set(data.maintenance_fee),
        withdrawal_fee: Set(data.withdrawal_fee),
        status: Set(Some(data.status)),
        ..Default::default()
    };

    entity::account_types::Entity::insert(acc_type)
        .exec(state.pgdb.get_ref())
        .await
}

pub async fn get_account_types(
    id: i64,
    state: &web::Data<AppState>,
) -> Result<Vec<AccountTypeRow>, DbErr> {
    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        r#"
        SELECT 
            at.id,
            at.institution_id,
            at.name,
            at.code,
            at.description,
            at.currency,
            at.minimum_balance,
            at.maximum_balance,
            at.interest_rate,
            at.interest_rate_calc_method,
            at.kyc_tier,
            at.interest_payout_frequency,
            at.is_overdraft_allowable,
            at.overdraft_limit,
            at.overdraft_interest_rate,
            at.dormancy_period_days,
            at.maintenance_fee,
            at.withdrawal_fee,
            at.status,
            at.custom_fields,
            at.created_at,
            at.updated_at,
            at.category_id,

            ac.id AS category_id,
            ac.name AS category_name,
            ac.category_type AS category_category_type,
            ac.description AS category_description,
            ac.is_active AS category_is_active

        FROM account_types at
        JOIN account_categories ac ON at.category_id = ac.id
        WHERE at.institution_id = $1
        "#,
        vec![id.into()],
    );

    AccountTypeFlat::find_by_statement(stmt)
        .all(state.pgdb.get_ref())
        .await
        .map(|rows| rows.into_iter().map(Into::into).collect())
}
