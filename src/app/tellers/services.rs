use actix_web::web;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, ConnectionTrait, DatabaseBackend, DatabaseTransaction, DbErr,
    EntityTrait, FromQueryResult, InsertResult, QueryFilter, Statement,
};

use crate::{
    AppState,
    app::tellers::{
        mapper::{
            TellerCashDrawerFlat, TellerCashDrawerRow, TellerFlat, TellerReconFlat, TellerReconRow,
            TellerRow,
        },
        models::{AddDrawerModel, AddTellerModel, AddTellerReconModel},
    },
    utils::{
        gen_snow_ids,
        models::{DateQuery, MetaModel, QueryModel},
    },
};

pub async fn add_teller(
    model: &AddTellerModel,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::tellers::ActiveModel>, DbErr> {
    use entity::tellers::{ActiveModel, Entity};

    let (snowflake, _) =
        gen_snow_ids::gen_snowflake_slug().map_err(|e| DbErr::Custom(e.to_string()))?;

    let data = model.clone();

    let teller = ActiveModel {
        id: Set(snowflake),
        institution_id: Set(data.institution_id),
        teller_name: Set(data.teller_name),
        teller_number: Set(data.teller_number),
        staff_id: Set(data.staff_id),
        branch_id: Set(data.branch_id),
        ..Default::default()
    };

    Entity::insert(teller).exec(state.pgdb.get_ref()).await
}

pub async fn add_recon(
    model: &AddTellerReconModel,
    trn: &DatabaseTransaction,
) -> Result<InsertResult<entity::teller_reconciliations::ActiveModel>, DbErr> {
    use entity::teller_reconciliations::{ActiveModel, Entity};

    let (snowflake, _) =
        gen_snow_ids::gen_snowflake_slug().map_err(|e| DbErr::Custom(e.to_string()))?;

    let data = model.clone();

    let recon = ActiveModel {
        id: Set(snowflake),
        cash_drawer_id: Set(data.cash_drawer_id),
        reconciliation_type: Set(data.recon_type),
        notes: Set(data.notes),
        supervisor_id: Set(data.supervisor_id),
        ..Default::default()
    };

    Entity::insert(recon).exec(trn).await
}

pub async fn open_drawer(
    model: &AddDrawerModel,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::teller_cash_drawers::ActiveModel>, DbErr> {
    use entity::teller_cash_drawers::{ActiveModel, Entity};

    let (snowflake, _) =
        gen_snow_ids::gen_snowflake_slug().map_err(|e| DbErr::Custom(e.to_string()))?;

    let data = model.clone();

    let drawer = ActiveModel {
        id: Set(snowflake),
        teller_id: Set(data.teller_id),
        opening_cash: Set(Some(data.opening_cash)),
        ..Default::default()
    };

    Entity::insert(drawer).exec(state.pgdb.get_ref()).await
}

pub async fn get_teller(
    teller_id: i64,
    institution_id: i64,
    state: &web::Data<AppState>,
) -> Result<TellerRow, DbErr> {
    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        r#"
        SELECT
            t.id,
            t.institution_id,
            t.branch_id,
            t.teller_name,
            t.teller_number,
            t.drawer_limit,
            t.current_drawer_balance,
            t.status,
            t.is_logged_in,
            t.last_login_at,
            t.current_session_id,
            t.current_terminal_id,

            -- Staff
            s.id AS staff_id,
            s.employee_number AS staff_employee_number,
            s.full_name AS staff_full_name,
            s.first_name AS staff_first_name,
            s.last_name AS staff_last_name,
            s.phone_number AS staff_phone_number,
            s.email_address AS staff_email_address,
            s.job_title AS staff_job_title,
            s.department AS staff_department,
            s.employment_status AS staff_employment_status,

            -- Supervisor
            sp.id AS supervisor_id,
            sp.employee_number AS supervisor_employee_number,
            sp.full_name AS supervisor_full_name,
            sp.first_name AS supervisor_first_name,
            sp.last_name AS supervisor_last_name,
            sp.phone_number AS supervisor_phone_number,
            sp.email_address AS supervisor_email_address,
            sp.job_title AS supervisor_job_title,
            sp.department AS supervisor_department,
            sp.employment_status AS supervisor_employment_status

        FROM tellers t

        INNER JOIN staff s
            ON s.id = t.staff_id
            AND s.institution_id = t.institution_id

        LEFT JOIN staff sp
            ON sp.id = t.supervisor_id
            AND sp.institution_id = t.institution_id

        WHERE t.id = $1
            AND t.institution_id = $2;
        "#,
        vec![teller_id.into(), institution_id.into()],
    );

    TellerFlat::find_by_statement(stmt)
        .one(state.pgdb.get_ref())
        .await?
        .ok_or_else(|| DbErr::Custom("Teller not found".to_string()))
        .map(Into::into)
}

pub async fn get_teller_list(
    institution_id: i64,
    model: QueryModel,
    state: &web::Data<AppState>,
) -> Result<(Vec<TellerRow>, MetaModel), DbErr> {
    let data = model.clone();

    let offset = (data.page.saturating_sub(1)) * data.size;

    let count_stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        r#"
        SELECT COUNT(*) as total
        FROM tellers t
        WHERE t.institution_id = $1;
        "#,
        vec![institution_id.into()],
    );

    let count = state
        .pgdb
        .get_ref()
        .query_one_raw(count_stmt)
        .await?
        .ok_or_else(|| DbErr::Custom("Failed to get teller count".to_string()))?;

    let total_items: i64 = count.try_get("", "total")?;

    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        r#"
        SELECT
            t.id,
            t.institution_id,
            t.branch_id,
            t.teller_name,
            t.teller_number,
            t.drawer_limit,
            t.current_drawer_balance,
            t.status,
            t.is_logged_in,
            t.last_login_at,
            t.current_session_id,
            t.current_terminal_id,

            -- Staff
            s.id AS staff_id,
            s.employee_number AS staff_employee_number,
            s.full_name AS staff_full_name,
            s.first_name AS staff_first_name,
            s.last_name AS staff_last_name,
            s.phone_number AS staff_phone_number,
            s.email_address AS staff_email_address,
            s.job_title AS staff_job_title,
            s.department AS staff_department,
            s.employment_status AS staff_employment_status,

            -- Supervisor
            sp.id AS supervisor_id,
            sp.employee_number AS supervisor_employee_number,
            sp.full_name AS supervisor_full_name,
            sp.first_name AS supervisor_first_name,
            sp.last_name AS supervisor_last_name,
            sp.phone_number AS supervisor_phone_number,
            sp.email_address AS supervisor_email_address,
            sp.job_title AS supervisor_job_title,
            sp.department AS supervisor_department,
            sp.employment_status AS supervisor_employment_status

        FROM tellers t

        INNER JOIN staff s
            ON s.id = t.staff_id
            AND s.institution_id = t.institution_id

        LEFT JOIN staff sp
            ON sp.id = t.supervisor_id
            AND sp.institution_id = t.institution_id

        WHERE t.institution_id = $1
        ORDER BY t.created_at DESC
        LIMIT $2 OFFSET $3
        "#,
        vec![institution_id.into(), data.size.into(), offset.into()],
    );

    let rows = TellerFlat::find_by_statement(stmt)
        .all(state.pgdb.get_ref())
        .await?;

    let items: Vec<TellerRow> = rows.into_iter().map(Into::into).collect();

    let total_pages = if total_items == 0 {
        0
    } else {
        ((total_items - 1) / data.size as i64) + 1
    };

    let meta = MetaModel {
        total_items: total_items as u64,
        total_pages: total_pages as u64,
        page: data.page,
        per_page: data.size,
    };

    Ok((items, meta))
}

pub async fn get_teller_drawers(
    teller_id: i64,
    institution_id: i64,
    dates: &DateQuery,
    state: &web::Data<AppState>,
) -> Result<Vec<TellerCashDrawerRow>, DbErr> {
    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        r#"
        SELECT
            -- Drawer
            tcd.id,
            tcd.opening_cash_amount,
            tcd.opening_cash,
            tcd.total_cash_in,
            tcd.total_cash_out,
            tcd.cheque_count,
            tcd.total_cheque_amount,
            tcd.transfer_in_count,
            tcd.total_transfer_in_amount,
            tcd.transfer_out_count,
            tcd.total_transfer_out_amount,
            tcd.closing_balance,
            tcd.closing_cash,
            tcd.expected_amount,
            tcd.variance_amount,
            tcd.variance_reason,
            tcd.status,
            tcd.opened_at,
            tcd.closed_at,

            -- Teller
            t.id AS teller_id,
            t.teller_name,
            t.teller_number,
            t.branch_id,

            -- Supervisor
            s.id AS supervisor_id,
            s.employee_number AS supervisor_employee_number,
            s.full_name AS supervisor_full_name,
            s.first_name AS supervisor_first_name,
            s.last_name AS supervisor_last_name,
            s.phone_number AS supervisor_phone_number,
            s.email_address AS supervisor_email_address,
            s.job_title AS supervisor_job_title,
            s.department AS supervisor_department,
            s.employment_status AS supervisor_employment_status

        FROM teller_cash_drawers tcd

        INNER JOIN tellers t
            ON t.id = tcd.teller_id
            AND t.institution_id = tcd.institution_id

        LEFT JOIN staff s
            ON s.id = t.supervisor_id
            AND s.institution_id = t.institution_id

        WHERE tcd.institution_id = $1
        AND tcd.teller_id = $2
        AND tcd.opened_at < $3
            AND (
                tcd.closed_at IS NULL
                OR tcd.closed_at >= $4
            )
        "#,
        vec![
            institution_id.into(),
            teller_id.into(),
            dates.to.into(),
            dates.from.into(),
        ],
    );

    TellerCashDrawerFlat::find_by_statement(stmt)
        .all(state.pgdb.get_ref())
        .await
        .map(|rows| rows.into_iter().map(Into::into).collect())
}

pub async fn get_institution_drawers(
    institution_id: i64,
    dates: &DateQuery,
    state: &web::Data<AppState>,
) -> Result<Vec<TellerCashDrawerRow>, DbErr> {
    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        r#"
        SELECT
            -- Drawer
            tcd.id,
            tcd.opening_cash_amount,
            tcd.opening_cash,
            tcd.total_cash_in,
            tcd.total_cash_out,
            tcd.cheque_count,
            tcd.total_cheque_amount,
            tcd.transfer_in_count,
            tcd.total_transfer_in_amount,
            tcd.transfer_out_count,
            tcd.total_transfer_out_amount,
            tcd.closing_balance,
            tcd.closing_cash,
            tcd.expected_amount,
            tcd.variance_amount,
            tcd.variance_reason,
            tcd.status,
            tcd.opened_at,
            tcd.closed_at,

            -- Teller
            t.id AS teller_id,
            t.teller_name,
            t.teller_number,
            t.branch_id,

            -- Supervisor
            s.id AS supervisor_id,
            s.employee_number AS supervisor_employee_number,
            s.full_name AS supervisor_full_name,
            s.first_name AS supervisor_first_name,
            s.last_name AS supervisor_last_name,
            s.phone_number AS supervisor_phone_number,
            s.email_address AS supervisor_email_address,
            s.job_title AS supervisor_job_title,
            s.department AS supervisor_department,
            s.employment_status AS supervisor_employment_status

        FROM teller_cash_drawers tcd

        INNER JOIN tellers t
            ON t.id = tcd.teller_id
            AND t.institution_id = tcd.institution_id

        LEFT JOIN staff s
            ON s.id = t.supervisor_id
            AND s.institution_id = t.institution_id

        WHERE tcd.institution_id = $1
        AND tcd.opened_at < $2
            AND (
                tcd.closed_at IS NULL
                OR tcd.closed_at >= $3
            )
        "#,
        vec![institution_id.into(), dates.to.into(), dates.from.into()],
    );

    TellerCashDrawerFlat::find_by_statement(stmt)
        .all(state.pgdb.get_ref())
        .await
        .map(|rows| rows.into_iter().map(Into::into).collect())
}

pub async fn get_teller_recons(
    teller_id: i64,
    institution_id: i64,
    dates: &DateQuery,
    state: &web::Data<AppState>,
) -> Result<Vec<TellerReconRow>, DbErr> {
    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        r#"
        SELECT
            tr.id,
            tr.cash_drawer_id,
            tr.reconciliation_type,
            tr.notes,
            tr.created_at,
            tr.updated_at,

            s.id AS supervisor_id,
            s.full_name

        FROM teller_reconciliations tr

        LEFT JOIN staff s
            ON s.id = tr.supervisor_id
            AND s.institution_id = tr.institution_id

        WHERE
            tr.institution_id = $1
            AND tr.created_at >= $2
            AND tr.created_at < $3
            AND tr.teller_id = $4
        "#,
        vec![
            institution_id.into(),
            dates.from.into(),
            dates.to.into(),
            teller_id.into(),
        ],
    );

    TellerReconFlat::find_by_statement(stmt)
        .all(state.pgdb.get_ref())
        .await
        .map(|rows| rows.into_iter().map(Into::into).collect())
}

pub async fn get_institution_recons(
    institution_id: i64,
    dates: &DateQuery,
    state: &web::Data<AppState>,
) -> Result<Vec<TellerReconRow>, DbErr> {
    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        r#"
        SELECT
            tr.id,
            tr.cash_drawer_id,
            tr.reconciliation_type,
            tr.notes,
            tr.created_at,
            tr.updated_at,

            s.id AS supervisor_id,
            s.full_name

        FROM teller_reconciliations tr

        LEFT JOIN staff s
            ON s.id = tr.supervisor_id
            AND s.institution_id = tr.institution_id

        WHERE
            tr.institution_id = $1
            AND tr.created_at >= $2
            AND tr.created_at < $3
        "#,
        vec![institution_id.into(), dates.from.into(), dates.to.into()],
    );

    TellerReconFlat::find_by_statement(stmt)
        .all(state.pgdb.get_ref())
        .await
        .map(|rows| rows.into_iter().map(Into::into).collect())
}

pub async fn get_teller_open_drawer(
    staff_id: i64,
    state: &web::Data<AppState>,
) -> Result<TellerCashDrawerRow, DbErr> {
    let teller = entity::tellers::Entity::find()
        .filter(entity::tellers::Column::StaffId.eq(staff_id))
        .one(state.pgdb.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Teller not found".to_string()))?;

    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        r#"
        SELECT
            -- Drawer
            tcd.id,
            tcd.opening_cash_amount,
            tcd.opening_cash,
            tcd.total_cash_in,
            tcd.total_cash_out,
            tcd.cheque_count,
            tcd.total_cheque_amount,
            tcd.transfer_in_count,
            tcd.total_transfer_in_amount,
            tcd.transfer_out_count,
            tcd.total_transfer_out_amount,
            tcd.closing_balance,
            tcd.closing_cash,
            tcd.expected_amount,
            tcd.variance_amount,
            tcd.variance_reason,
            tcd.status,
            tcd.opened_at,
            tcd.closed_at,

            -- Teller
            t.id AS teller_id,
            t.teller_name,
            t.teller_number,
            t.branch_id,

            -- Supervisor
            s.id AS supervisor_id,
            s.employee_number AS supervisor_employee_number,
            s.full_name AS supervisor_full_name,
            s.first_name AS supervisor_first_name,
            s.last_name AS supervisor_last_name,
            s.phone_number AS supervisor_phone_number,
            s.email_address AS supervisor_email_address,
            s.job_title AS supervisor_job_title,
            s.department AS supervisor_department,
            s.employment_status AS supervisor_employment_status

        FROM teller_cash_drawers tcd

        INNER JOIN tellers t
            ON t.id = tcd.teller_id
            AND t.institution_id = tcd.institution_id

        LEFT JOIN staff s
            ON s.id = t.supervisor_id
            AND s.institution_id = t.institution_id

        WHERE tcd.institution_id = $1
        AND tcd.teller_id = $2
        WHERE ts.opened_at < DATE_TRUNC('day', NOW()) + INTERVAL '1 day'
          AND (
              ts.closed_at IS NULL
              OR ts.closed_at >= DATE_TRUNC('day', NOW())
          )
        "#,
        vec![teller.institution_id.into(), teller.id.into()],
    );

    TellerCashDrawerFlat::find_by_statement(stmt)
        .one(state.pgdb.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Drawer not found".to_string()))
        .map(|row| row.into())
}
