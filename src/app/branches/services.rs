use actix_web::web;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, Condition, DbErr, EntityTrait, InsertResult, PaginatorTrait,
    QueryFilter, QueryOrder,
};

use crate::{
    AppState,
    app::branches::models::{AddBranchModel, BranchResponseModel},
    utils::{
        gen_snow_ids::gen_snowflake_slug,
        models::{MetaModel, QueryModel},
    },
};

pub async fn save_branch(
    model: &AddBranchModel,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::branches::ActiveModel>, DbErr> {
    let (snowflake, _) = match gen_snowflake_slug() {
        Ok(res) => res,
        Err(_) => return Err(DbErr::Custom("Failed to generate ID's".to_string())),
    };

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
) -> Result<BranchResponseModel, DbErr> {
    let result = entity::branches::Entity::find_by_id(*id)
        .into_model::<BranchResponseModel>()
        .one(state.pgdb.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Branch not found".into()));

    result
}

pub async fn get_via_ins(
    id: &i64,
    state: &web::Data<AppState>,
) -> Result<BranchResponseModel, DbErr> {
    let result = entity::branches::Entity::find()
        .filter(Condition::all().add(entity::branches::Column::InstitutionId.eq(*id)))
        .into_model::<BranchResponseModel>()
        .one(state.pgdb.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Branch not found".into()));

    result
}

pub async fn get_all(
    id: &i64,
    query: &QueryModel,
    state: &web::Data<AppState>,
) -> Result<(Vec<BranchResponseModel>, MetaModel), DbErr> {
    let page = query.page.max(1);
    let per_page = query.size.max(1);

    let paginator = entity::branches::Entity::find()
        .filter(
            Condition::all()
                .add(entity::branches::Column::InstitutionId.eq(*id))
                .add(entity::branches::Column::IsDeleted.eq(false)),
        )
        .order_by_desc(entity::branches::Column::UpdatedAt)
        .into_model::<BranchResponseModel>()
        .paginate(state.pgdb.get_ref(), per_page);

    let items = paginator.fetch_page(page - 1).await?;

    let total_items = paginator.num_items().await?;
    let total_pages = (total_items + per_page - 1) / per_page;

    let meta = MetaModel {
        total_items,
        total_pages,
        page,
        per_page,
    };

    Ok((items, meta))
}
