use actix_web::web;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, Condition, ConnectionTrait, DatabaseBackend,
    DatabaseTransaction, DbErr, EntityTrait, InsertResult, QueryFilter, Statement,
};

use crate::{
    AppState,
    app::gls::{
        mapper::LedgerLockModel,
        models::{
            AddGlDailyBalance, AddGlPostings, AddLedgerEntry, CloseGlDailyBalance,
            CreateLedgerLockPeriod,
        },
    },
    utils::gen_snow_ids,
};

pub async fn save_ledger_entry(
    model: &AddLedgerEntry,
    trn: &DatabaseTransaction,
) -> Result<InsertResult<entity::ledger_entries::ActiveModel>, DbErr> {
    use entity::ledger_entries::{ActiveModel, Entity};

    let data = model.clone();

    let (snowflake, _) =
        gen_snow_ids::gen_snowflake_slug().map_err(|e| DbErr::Custom(e.to_string()))?;

    let entry = ActiveModel {
        id: Set(snowflake),
        institution_id: Set(data.institution_id),
        transaction_id: Set(data.transaction_id),
        account_id: Set(data.account_id),
        gl_account_id: Set(data.gl_account_id),
        entry_type: Set(data.entry_type),
        amount: Set(data.amount),
        currency_code: Set(data.currency_code),
        description: Set(Some(data.description)),
        reference: Set(Some(data.reference)),
        ..Default::default()
    };

    Entity::insert(entry).exec(trn).await
}

pub async fn save_gl_posting(
    model: &AddGlPostings,
    trn: &DatabaseTransaction,
) -> Result<InsertResult<entity::gl_postings::ActiveModel>, DbErr> {
    use entity::gl_postings::{ActiveModel, Entity};

    let data = model.clone();

    let (snowflake, _) =
        gen_snow_ids::gen_snowflake_slug().map_err(|e| DbErr::Custom(e.to_string()))?;

    let posting = ActiveModel {
        id: Set(snowflake),
        institution_id: Set(data.institution_id),
        transaction_id: Set(Some(data.transaction_id)),
        reference_number: Set(Some(data.ref_number)),
        value_date: Set(data.value_date),
        posting_date: Set(Some(data.posting_date.into())),
        debit_account_id: Set(data.debit_account_id),
        debit_amount: Set(data.debit_amount),
        credit_account_id: Set(data.credit_account_id),
        credit_amount: Set(data.credit_amount),
        narration: Set(data.narration),
        is_reversed: Set(Some(data.is_reversed)),
        posted_by: Set(Some(data.posted_by)),
        ..Default::default()
    };

    Entity::insert(posting).exec(trn).await
}

pub async fn create_ledger_lock(
    model: &CreateLedgerLockPeriod,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::ledger_lock_periods::ActiveModel>, DbErr> {
    use entity::ledger_lock_periods::{ActiveModel, Entity};

    let data = model.clone();

    let (snowflake, _) =
        gen_snow_ids::gen_snowflake_slug().map_err(|e| DbErr::Custom(e.to_string()))?;

    let lock_period = ActiveModel {
        id: Set(snowflake),
        institution_id: Set(data.institution_id),
        start_date: Set(data.start_date),
        end_date: Set(data.end_date),
        lock_type: Set(data.lock_type),
        is_locked: Set(Some(true)),
        locked_by: Set(Some(data.locked_by)),
        locked_at: Set(Some(chrono::Utc::now().into())),
        ..Default::default()
    };

    Entity::insert(lock_period).exec(state.pgdb.get_ref()).await
}

pub async fn upsert_gl_daily_balances(
    model: &AddGlDailyBalance,
    trn: &DatabaseTransaction,
) -> Result<(), DbErr> {
    let data = model.clone();

    for gl_info in data.gl_infos {
        let (snowflake, _) =
            gen_snow_ids::gen_snowflake_slug().map_err(|e| DbErr::Custom(e.to_string()))?;

        let stmt = Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"
            WITH prior AS (
                SELECT closing_balance
                  FROM gl_daily_balances
                 WHERE institution_id = $2
                   AND account_id     = $3
                   AND balance_date   < $4
                 ORDER BY balance_date DESC
                 LIMIT 1
            )
            INSERT INTO gl_daily_balances (
                id, institution_id, account_id, balance_date,
                opening_balance,
                total_debits, total_credits, transaction_count,
                closing_balance, is_reconciled, created_at, updated_at
            )
            SELECT
                $1, $2, $3, $4,
                COALESCE(prior.closing_balance, 0),
                $5, $6, $7,
                COALESCE(prior.closing_balance, 0),
                false, NOW(), NOW()
            FROM (SELECT 1) AS anchor
            LEFT JOIN prior ON true
            ON CONFLICT (institution_id, account_id, balance_date)
            DO UPDATE SET
                total_debits      = COALESCE(gl_daily_balances.total_debits, 0) + EXCLUDED.total_debits,
                total_credits     = COALESCE(gl_daily_balances.total_credits, 0) + EXCLUDED.total_credits,
                transaction_count = COALESCE(gl_daily_balances.transaction_count, 0) + EXCLUDED.transaction_count,
                updated_at        = NOW()
            "#,
            vec![
                snowflake.into(),
                data.institution_id.into(),
                gl_info.gl_account_id.into(),
                data.balance_date.into(),
                gl_info.debit_delta.into(),
                gl_info.credit_delta.into(),
                data.count_delta.into(),
            ],
        );

        let result = trn.execute_raw(stmt).await?;

        if result.rows_affected() == 0 {
            return Err(DbErr::RecordNotFound(
                "Could not update upsert gl daily balances".to_string(),
            ));
        };
    }

    Ok(())
}

pub async fn close_gl_daily_balances(
    model: &CloseGlDailyBalance,
    state: &web::Data<AppState>,
) -> Result<(), DbErr> {
    let data = model.clone();

    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        r#"
        UPDATE gl_daily_balances
        SET closing_balance = $4,
            updated_at      = NOW()
        WHERE institution_id = $1
            AND account_id     = $2
            AND balance_date   = $3;
        "#,
        vec![
            data.institution_id.into(),
            data.gl_account_id.into(),
            data.balance_date.into(),
            data.closing_balance.into(),
        ],
    );

    let result = state.pgdb.get_ref().execute_raw(stmt).await?;

    if result.rows_affected() == 0 {
        return Err(DbErr::RecordNotFound(
            "Could not update close gl daily balances".to_string(),
        ));
    };

    Ok(())
}

pub async fn get_ledger_lock_period(
    institution_id: i64,
    state: &web::Data<AppState>,
) -> Result<Option<LedgerLockModel>, DbErr> {
    use entity::ledger_lock_periods::{Column, Entity};

    let now = chrono::Utc::now().date_naive();

    Entity::find()
        .filter(
            Condition::all()
                .add(Column::InstitutionId.eq(institution_id))
                .add(Column::StartDate.lte(now))
                .add(Column::EndDate.gte(now))
                .add(Column::IsLocked.eq(true)),
        )
        .into_model::<LedgerLockModel>()
        .one(state.pgdb.get_ref())
        .await
}
