use actix_web::web;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, Condition, DbErr, EntityTrait, InsertResult, QueryFilter,
};

use crate::{AppState, app::branches::models::AddBranchModel, utils};

pub async fn save_branch(
    model: &AddBranchModel,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::branches::ActiveModel>, DbErr> {
    let (snowflake, _) =
        utils::gen_snow_ids::gen_snowflake_slug().unwrap_or_else(|_| (0, "0".into()));

    let data = model.clone();

    let branch = entity::branches::ActiveModel {
        id: Set(snowflake),
        name: Set(Some(data.name)),
        code: Set(Some(data.code)),
        institution_id: Set(data.institution),
        address: Set(Some(data.address)),
        phone: Set(Some(data.phone)),
        email: Set(Some(data.email)),
        location: Set(Some(data.location)),
        is_main: Set(Some(data.is_main)),
        cash_limit: Set(Some(data.cash_limit)),
        ..Default::default()
    };

    let insertion = entity::branches::Entity::insert(branch)
        .exec(state.pgdb.get_ref())
        .await
        .map_err(|err| DbErr::Custom(err.to_string()))?;

    Ok(insertion)
}

pub async fn get_details(
    id: &i64,
    state: &web::Data<AppState>,
) -> Result<entity::branches::Model, DbErr> {
    let result = entity::branches::Entity::find_by_id(*id)
        .one(state.pgdb.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Branch not found".into()));

    result
}

pub async fn get_via_ins(
    id: &i64,
    state: &web::Data<AppState>,
) -> Result<entity::branches::Model, DbErr> {
    let result = entity::branches::Entity::find()
        .filter(Condition::all().add(entity::branches::Column::InstitutionId.eq(*id)))
        .one(state.pgdb.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Branch not found".into()));

    result
}

pub async fn get_all(
    id: &i64,
    state: &web::Data<AppState>,
) -> Result<Vec<entity::branches::Model>, DbErr> {
    let results = entity::branches::Entity::find()
        .filter(
            Condition::all()
                .add(entity::branches::Column::InstitutionId.eq(*id))
                .add(entity::branches::Column::IsDeleted.eq(false)),
        )
        .all(state.pgdb.get_ref())
        .await
        .map_err(|err| DbErr::Custom(err.to_string()))?;

    Ok(results)
}
