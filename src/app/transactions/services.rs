use actix_web::web;
use sea_orm::{ActiveValue::Set, DbErr, EntityTrait, InsertResult};

use crate::{
    AppState,
    app::transactions::models::{AddTransactionChannelModel, AddTransactionLimitModel},
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