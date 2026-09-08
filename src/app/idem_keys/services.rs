use actix_web::web;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, Condition, DbErr, EntityTrait, ExprTrait, InsertResult,
    QueryFilter,
};

use crate::{
    AppState,
    app::idem_keys::models::{IdemModel, InitIdemModel},
    utils::{gen_snow_ids, password::compute_hash},
};

pub async fn write_idem_key(
    model: &IdemModel,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::idempotency_keys::ActiveModel>, DbErr> {
    use entity::idempotency_keys::{ActiveModel, Entity};

    let data = model.clone();

    let (snowflake, _) =
        gen_snow_ids::gen_snowflake_slug().map_err(|e| DbErr::Custom(e.to_string()))?;

    let idem = ActiveModel {
        institution_id: Set(snowflake),
        customer_id: Set(data.customer_id),
        account_id: Set(data.account_id),
        transaction_id: Set(data.transaction_id),
        transaction_group_id: Set(data.transaction_group_id),
        idempotency_key: Set(data.idem_key),
        operation: Set(data.operation),
        request_method: Set(data.request_method),
        request_path: Set(data.request_path),
        request_hash: Set(data.request_hash),
        channel: Set(data.channel),
        ..Default::default()
    };

    Entity::insert(idem).exec(state.pgdb.get_ref()).await
}

pub async fn find_idem_key(
    model: &InitIdemModel,
    state: &web::Data<AppState>,
) -> Result<Option<entity::idempotency_keys::Model>, DbErr> {
    use entity::idempotency_keys::{Column, Entity};

    let data = model.clone();

    let string_idem =
        serde_json::to_string(&data).map_err(|e| DbErr::Custom(format!("Parse Error: {}", e)))?;

    let req_hash = compute_hash(string_idem.as_bytes());

    Entity::find()
        .filter(
            Condition::all()
                .and(Column::InstitutionId.eq(data.institution_id))
                .and(Column::IdempotencyKey.eq(data.idem_key))
                .add(Column::RequestHash.eq(req_hash)),
        )
        .one(state.pgdb.get_ref())
        .await
}
