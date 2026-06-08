use actix_web::web;
use entity::sea_orm_active_enums::AccTypeStatus;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseBackend, DbErr,
    EntityTrait, FromQueryResult, InsertResult, QueryFilter, Statement,
};

use crate::{
    AppState,
    app::accounts::{
        mapper::{AccountFlat, AccountRow},
        models::AddAccountModel,
    },
    utils::{
        gen_snow_ids,
        models::{CursorMetaModel, CursorModel},
    },
};

pub async fn save_customer_acc(
    model: &AddAccountModel,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::accounts::ActiveModel>, DbErr> {
    use entity::accounts::{ActiveModel, Entity};

    let (snowflake, _) =
        gen_snow_ids::gen_snowflake_slug().map_err(|e| DbErr::Custom(e.to_string()))?;

    let data = model.clone();

    let account = ActiveModel {
        id: Set(snowflake),
        institution_id: Set(data.institution_id),
        customer_id: Set(data.customer_id),
        account_type_id: Set(data.account_type_id),
        account_name: Set(Some(data.account_name)),
        account_number: Set(Some(data.account_number)),
        status: Set(Some(AccTypeStatus::Inactive)),
        ..Default::default()
    };

    Entity::insert(account).exec(state.pgdb.get_ref()).await
}

pub async fn activate_account(
    account_id: i64,
    customer_id: i64,
    state: &web::Data<AppState>,
) -> Result<(), DbErr> {
    use entity::accounts::{ActiveModel, Column, Entity};

    let account = Entity::find_by_id(account_id)
        .filter(
            Condition::all()
                .add(Column::CustomerId.eq(customer_id))
                .add(Column::Status.eq(AccTypeStatus::Inactive)),
        )
        .one(state.pgdb.get_ref())
        .await?
        .ok_or_else(|| DbErr::Custom("Account not found".to_string()))?;

    let mut active_account: ActiveModel = account.into();

    active_account.activation_date = Set(Some(chrono::Utc::now().into()));
    active_account.status = Set(Some(AccTypeStatus::Active));
    active_account.updated_at = Set(Some(chrono::Utc::now().into()));

    active_account.update(state.pgdb.get_ref()).await?;

    Ok(())
}

pub async fn get_accounts(
    institution_id: i64,
    model: CursorModel,
    state: &web::Data<AppState>,
) -> Result<(Vec<AccountRow>, CursorMetaModel), DbErr> {
    let data = model.clone();

    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        r#"
        SELECT
            acc.id,
            acc.institution_id,
            acc.acccount_number,
            acc.account_name,
            acc.currency,
            acc.current_balance,
            acc.available_balance,
            acc.ledger_balance,
            acc.hold_balance,
            acc.status,
            acc.activation_date,
            acc.dormancy_date,
            acc.frozen_at,
            acc.frozen_reason,
            acc.is_overdraft_allowable,
            acc.overdraft_limit,
            acc.overdraft_used,
            acc.tags,

            pa.id AS parent_account_id,
            pa.account_number AS parent_account_acccount_number,
            pa.account_name AS parent_account_account_name,
            pa.currency AS parent_account_currency,
            pa.current_balance AS parent_account_current_balance,
            pa.available_balance AS parent_account_available_balance,
            pa.ledger_balance AS parent_account_ledger_balance,
            pa.hold_balance AS parent_account_hold_balance,

            at.id AS account_type_id,
            at.institution_id AS account_type_institution_id,
            at.name AS account_type_name,
            at.code AS account_type_code,
            at.description AS account_type_description,
            at.minimum_balance AS account_type_minimum_balance,
            at.maximum_balance AS account_type_maximum_balance,
            at.interest_rate AS account_type_interest_rate,
            at.maintenance_fee AS account_type_maintenance_fee,
            at.withdrawal_fee AS account_type_withdrawal_fee,

            cu.id AS customer_id,
            cu.customer_type,
            cu.customer_number,
            cu.first_name AS customer_first_name,
            cu.last_name AS customer_last_name,

        FROM accounts acc
        JOIN account_types at ON acc.account_type_id = at.id
        JOIN customers cu ON acc.customer_id = cu.id
        LEFT JOIN accounts pa ON acc.parent_account_id = pa.id
        WHERE ($1::bigint IS NULL OR acc.id < $1::bigint) AND acc.institution_id = $3
        LIMIT $2
        "#,
        vec![
            data.cursor.into(),
            ((data.limit as i64) + 1).into(),
            institution_id.into(),
        ],
    );

    let rows = AccountFlat::find_by_statement(stmt)
        .all(state.pgdb.get_ref())
        .await?;

    let has_next = rows.len() > data.limit as usize;

    let next_cursor = has_next
        .then(|| rows.last().map(|r| r.id.to_string()))
        .flatten();

    let items: Vec<AccountRow> = rows
        .into_iter()
        .take(data.limit as usize)
        .map(Into::into)
        .collect();

    let meta = CursorMetaModel {
        next_cursor,
        has_next,
        limit: data.limit,
    };

    Ok((items, meta))
}

pub async fn fetch_customer_acc(
    institution_id: i64,
    state: &web::Data<AppState>,
) -> Result<Option<AccountRow>, DbErr> {
    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        r#"
        SELECT
            acc.id,
            acc.institution_id,
            acc.acccount_number,
            acc.account_name,
            acc.currency,
            acc.current_balance,
            acc.available_balance,
            acc.ledger_balance,
            acc.hold_balance,
            acc.status,
            acc.activation_date,
            acc.dormancy_date,
            acc.frozen_at,
            acc.frozen_reason,
            acc.is_overdraft_allowable,
            acc.overdraft_limit,
            acc.overdraft_used,
            acc.tags,

            pa.id AS parent_account_id,
            pa.account_number AS parent_account_acccount_number,
            pa.account_name AS parent_account_account_name,
            pa.currency AS parent_account_currency,
            pa.current_balance AS parent_account_current_balance,
            pa.available_balance AS parent_account_available_balance,
            pa.ledger_balance AS parent_account_ledger_balance,
            pa.hold_balance AS parent_account_hold_balance,

            at.id AS account_type_id,
            at.institution_id AS account_type_institution_id,
            at.name AS account_type_name,
            at.code AS account_type_code,
            at.description AS account_type_description,
            at.minimum_balance AS account_type_minimum_balance,
            at.maximum_balance AS account_type_maximum_balance,
            at.interest_rate AS account_type_interest_rate,
            at.maintenance_fee AS account_type_maintenance_fee,
            at.withdrawal_fee AS account_type_withdrawal_fee,

            cu.id AS customer_id,
            cu.customer_type,
            cu.customer_number,
            cu.first_name AS customer_first_name,
            cu.last_name AS customer_last_name,

        FROM accounts acc
        JOIN account_types at ON acc.account_type_id = at.id
        JOIN customers cu ON acc.customer_id = cu.id
        LEFT JOIN accounts pa ON acc.parent_account_id = pa.id
        acc.institution_id = $1
        "#,
        vec![institution_id.into()],
    );

    AccountFlat::find_by_statement(stmt)
        .one(state.pgdb.get_ref())
        .await
        .map(|opt_row| opt_row.map(Into::into))
}
