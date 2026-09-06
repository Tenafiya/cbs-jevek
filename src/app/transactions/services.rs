use actix_web::web;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, DatabaseBackend, DatabaseTransaction, DbErr, EntityTrait,
    FromQueryResult, InsertResult, QueryFilter, Statement,
};

use crate::{
    AppState,
    app::transactions::{
        mapper::{
            TransactionChannelResponseModel, TransactionCheckerFlat, TransactionCheckerRow,
            TransactionLimitFlat, TransactionLimitRow,
        },
        models::{AddDepositModel, AddTransactionChannelModel, AddTransactionLimitModel},
    },
    utils::gen_snow_ids,
};

pub async fn add_trans_limit(
    model: &AddTransactionLimitModel,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::transaction_limits::ActiveModel>, DbErr> {
    use entity::transaction_limits::{ActiveModel, Entity};

    let (snowflake, _) =
        gen_snow_ids::gen_snowflake_slug().map_err(|e| DbErr::Custom(e.to_string()))?;

    let data = model.clone();

    let limit = ActiveModel {
        id: Set(snowflake),
        institution_id: Set(data.institution_id),
        transaction_channel_id: Set(data.trans_channel_id),
        customer_type: Set(data.customer_type),
        account_category_id: Set(data.acc_category_id),
        limit_type: Set(data.limit_type),
        max_amount: Set(data.max_amount),
        max_count: Set(data.max_count),
        effective_from: Set(data.effective_from),
        effective_to: Set(data.effective_to),
        currency: Set(data.currency),
        ..Default::default()
    };

    Entity::insert(limit).exec(state.pgdb.get_ref()).await
}

pub async fn add_deposit_transaction(
    model: &AddDepositModel,
    state: &web::Data<AppState>,
) -> Result<entity::transactions::Model, DbErr> {
    use entity::transactions::{ActiveModel, Entity};

    let (snowflake, _) =
        gen_snow_ids::gen_snowflake_slug().map_err(|e| DbErr::Custom(e.to_string()))?;

    let data = model.clone();

    let deposit = ActiveModel {
        id: Set(snowflake),
        institution_id: Set(data.core.institution_id),
        transaction_channel_id: Set(data.core.trans_channel_id),
        transaction_reference: Set(Some(data.core.reference)),
        credit_account_id: Set(Some(data.credit_account_id)),
        credit_customer_id: Set(Some(data.credit_customer_id)),
        amount: Set(data.core.amount),
        currency: Set(Some(data.core.currency)),
        total_amount: Set(data.core.total_amount),
        transaction_group_id: Set(data.core.transaction_group_id),
        transaction_type: Set(data.core.transaction_type),
        transaction_category: Set(data.core.transaction_category),
        description: Set(data.description),
        status: Set(data.core.status),
        posted_at: Set(Some(chrono::Utc::now().into())),
        ip_address: Set(data.core.ip_address),
        created_by: Set(Some(data.core.created_by)),
        teller_cash_drawer_id: Set(Some(data.drawer_id)),
        ..Default::default()
    };

    Entity::insert(deposit)
        .exec_with_returning(state.pgdb.get_ref())
        .await
}

pub async fn add_trans_channel(
    model: &AddTransactionChannelModel,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::transaction_channels::ActiveModel>, DbErr> {
    use entity::transaction_channels::{ActiveModel, Entity};

    let (snowflake, _) =
        gen_snow_ids::gen_snowflake_slug().map_err(|e| DbErr::Custom(e.to_string()))?;

    let data = model.clone();

    let channel = ActiveModel {
        id: Set(snowflake),
        institution_id: Set(data.institution_id),
        channel_name: Set(data.channel_name),
        channel_code: Set(data.channel_code),
        description: Set(data.description),
        requires_maker_checker: Set(data.requires_maker_checker),
        metadata: Set(data.metadata),
        ..Default::default()
    };

    Entity::insert(channel).exec(state.pgdb.get_ref()).await
}

pub async fn fetch_checker_limit(
    institution_id: i64,
    channel_id: i64,
    trn: &DatabaseTransaction,
) -> Result<TransactionCheckerRow, DbErr> {
    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        r#"
        SELECT
            tc.id,
            tc.institution_id,
            tc.channel_name,
            tc.channel_code,
            tc.description,
            tc.requires_maker_checker,
            tc.metadata AS channel_metadata,

            tl.id AS limit_id,
            tl.transaction_channel_id AS limit_transaction_channel_id,
            tl.customer_type::TEXT AS limit_customer_type,
            tl.account_category_id AS limit_account_category_id,
            tl.limit_type::TEXT AS limit_limit_type,
            tl.max_amount AS limit_max_amount,
            tl.max_count AS limit_max_count,
            tl.currency AS limit_currency,
            tl.effective_from AS limit_effective_from,
            tl.effective_to AS limit_effective_to,

        FROM transaction_channels tc
        LEFT JOIN transaction_limits tl
            ON tl.transaction_channel_id = tc.id
        AND tl.institution_id = tc.institution_id
        AND tl.is_active = TRUE
        WHERE
            tc.institution_id = $1
            AND tc.id = $2
            AND tc.is_active = TRUE;
        "#,
        vec![institution_id.into(), channel_id.into()],
    );

    TransactionCheckerFlat::find_by_statement(stmt)
        .one(trn)
        .await?
        .ok_or_else(|| DbErr::Custom("Transaction Checker Not Found".to_string()))
        .map(Into::into)
}

pub async fn fetch_checker_limits(
    institution_id: i64,
    state: &web::Data<AppState>,
) -> Result<Vec<TransactionCheckerRow>, DbErr> {
    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        r#"
        SELECT
            tc.id,
            tc.institution_id,
            tc.channel_name,
            tc.channel_code,
            tc.description,
            tc.requires_maker_checker,
            tc.metadata AS channel_metadata,

            tl.id AS limit_id,
            tl.transaction_channel_id AS limit_transaction_channel_id,
            tl.customer_type::TEXT AS limit_customer_type,
            tl.account_category_id AS limit_account_category_id,
            tl.limit_type::TEXT AS limit_limit_type,
            tl.max_amount AS limit_max_amount,
            tl.max_count AS limit_max_count,
            tl.currency AS limit_currency,
            tl.effective_from AS limit_effective_from,
            tl.effective_to AS limit_effective_to,

        FROM transaction_channels tc
        LEFT JOIN transaction_limits tl
            ON tl.transaction_channel_id = tc.id
        AND tl.institution_id = tc.institution_id
        AND tl.is_active = TRUE
        WHERE
            tc.institution_id = $1
            AND tc.is_active = TRUE;
        "#,
        vec![institution_id.into()],
    );

    TransactionCheckerFlat::find_by_statement(stmt)
        .all(state.pgdb.get_ref())
        .await
        .map(|rows| rows.into_iter().map(Into::into).collect::<Vec<_>>())
}

pub async fn fetch_transaction_limit(
    institution_id: i64,
    trn_id: i64,
    trn: &DatabaseTransaction,
) -> Result<TransactionLimitRow, DbErr> {
    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        r#"
        SELECT
            tl.id,
            tl.institution_id,
            tl.customer_type,
            tl.limit_type::TEXT,
            tl.max_amount,
            tl.max_count,
            tl.currency,
            tl.is_active,
            tl.effective_from,
            tl.effective_to,
            tl.kyc_tier,
            tl.created_at,
            tl.updated_at,

            tc.id AS transaction_channel_id,
            tc.institution_id AS channel_institution_id,
            tc.channel_name,
            tc.channel_code,
            tc.requires_maker_checker,
            tc.metadata,

            ac.id AS account_category_id,
            ac.name AS category_name,
            ac.category_type::TEXT,
            ac.description AS category_description,
            ac.is_active AS category_is_active

        FROM transaction_limits AS tl

        INNER JOIN transaction_channels AS tc
            ON tc.id = tl.transaction_channel_id

        INNER JOIN account_categories AS ac
            ON ac.id = tl.account_category_id

        WHERE tl.institution_id = $1
          AND tl.id = $2
          AND tl.is_active = TRUE
          AND tc.is_active = TRUE
          AND ac.is_active = TRUE
          AND tl.effective_from <= NOW()
          AND (
              tl.effective_to IS NULL
              OR tl.effective_to >= NOW()
          )

        ORDER BY
            tl.customer_type,
            tl.kyc_tier,
            tl.limit_type;
        "#,
        vec![institution_id.into(), trn_id.into()],
    );

    TransactionLimitFlat::find_by_statement(stmt)
        .one(trn)
        .await?
        .ok_or_else(|| DbErr::Custom("Transaction Limit Not Found".to_string()))
        .map(Into::into)
}

pub async fn fetch_transaction_limits(
    institution_id: i64,
    state: &web::Data<AppState>,
) -> Result<Vec<TransactionLimitRow>, DbErr> {
    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        r#"
        SELECT
            tl.id,
            tl.institution_id,
            tl.customer_type,
            tl.limit_type::TEXT,
            tl.max_amount,
            tl.max_count,
            tl.currency,
            tl.is_active,
            tl.effective_from,
            tl.effective_to,
            tl.kyc_tier,
            tl.created_at,
            tl.updated_at,

            tc.id AS transaction_channel_id,
            tc.institution_id AS channel_institution_id,
            tc.channel_name,
            tc.channel_code,
            tc.requires_maker_checker,
            tc.metadata,

            ac.id AS account_category_id,
            ac.name AS category_name,
            ac.category_type::TEXT,
            ac.description AS category_description,
            ac.is_active AS category_is_active

        FROM transaction_limits AS tl

        INNER JOIN transaction_channels AS tc
            ON tc.id = tl.transaction_channel_id

        INNER JOIN account_categories AS ac
            ON ac.id = tl.account_category_id

        WHERE tl.institution_id = $1
          AND tl.is_active = TRUE
          AND tc.is_active = TRUE
          AND ac.is_active = TRUE
          AND tl.effective_from <= NOW()
          AND (
              tl.effective_to IS NULL
              OR tl.effective_to >= NOW()
          )

        ORDER BY
            tl.customer_type,
            tl.kyc_tier,
            tl.limit_type;
        "#,
        vec![institution_id.into()],
    );

    TransactionLimitFlat::find_by_statement(stmt)
        .one(state.pgdb.get_ref())
        .await
        .map(|rows| rows.into_iter().map(Into::into).collect::<Vec<_>>())
}

pub async fn fetch_transaction_channels(
    institution_id: i64,
    state: &web::Data<AppState>,
) -> Result<Vec<TransactionChannelResponseModel>, DbErr> {
    use entity::transaction_channels::{Column, Entity};

    Entity::find()
        .filter(Column::InstitutionId.eq(institution_id))
        .into_model::<TransactionChannelResponseModel>()
        .all(state.pgdb.get_ref())
        .await
}
