use actix_web::web;
use sea_orm::{
    ActiveValue::Set, DatabaseBackend, DatabaseTransaction, DbErr, EntityTrait, FromQueryResult,
    InsertResult, Statement,
};

use crate::{
    AppState,
    app::transactions::{
        mapper::{TransactionCheckerFlat, TransactionCheckerRow},
        models::{AddTransactionChannelModel, AddTransactionLimitModel},
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
