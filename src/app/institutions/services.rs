use actix_web::web;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DbErr, EntityTrait, InsertResult,
    PaginatorTrait, QueryFilter, QueryOrder,
};

use crate::{
    AppState,
    app::institutions::models::{AddInstitutionModel, UpdateInstitutionModel},
    utils::{
        self,
        models::{MetaModel, QueryModel},
    },
};

pub async fn save_institution(
    model: &AddInstitutionModel,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::institutions::ActiveModel>, DbErr> {
    let (snowflake, _) = utils::gen_snow_ids::gen_snowflake_slug().unwrap_or_else(|e| {
        println!("snowflake error: {e}; falling back to 0");
        (0, "0".into())
    });

    let data = model.clone();

    let institution = entity::institutions::ActiveModel {
        id: Set(snowflake),
        name: Set(data.name),
        code: Set(Some(data.code)),
        country_id: Set(data.country),
        license_number: Set(data.license_num),
        regulatory_number: Set(data.regulation_num),
        ..Default::default()
    };

    let insertion = entity::institutions::Entity::insert(institution)
        .exec(state.pgdb.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database insert error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(insertion)
}

pub async fn get_one(
    id: &i64,
    state: &web::Data<AppState>,
) -> Result<entity::institutions::Model, DbErr> {
    let result = entity::institutions::Entity::find_by_id(*id)
        .one(state.pgdb.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Institution not found".into()));

    result
}

pub async fn get_all(
    query: &QueryModel,
    state: &web::Data<AppState>,
) -> Result<(Vec<entity::institutions::Model>, MetaModel), DbErr> {
    let page = query.page.max(1);
    let per_page = query.size.max(1);

    let paginator = entity::institutions::Entity::find()
        .filter(
            Condition::all()
                .add(entity::institutions::Column::IsActive.eq(true))
                .add(entity::institutions::Column::IsDeleted.eq(false)),
        )
        .order_by_desc(entity::institutions::Column::UpdatedAt)
        .paginate(state.pgdb.get_ref(), per_page);

    let all = paginator.num_items_and_pages().await?;

    let items = paginator.fetch_page(page - 1).await.map_err(|err| {
        eprintln!("Database pagination error: {}", err);
        DbErr::Custom(err.to_string())
    })?;

    let meta = MetaModel {
        total_items: all.number_of_items,
        total_pages: all.number_of_pages,
        page,
        per_page,
    };

    Ok((items, meta))
}

pub async fn update(
    id: &i64,
    model: &UpdateInstitutionModel,
    state: &web::Data<AppState>,
) -> Result<(), DbErr> {
    let institution = entity::institutions::Entity::find_by_id(*id)
        .one(state.pgdb.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Institution not found".into()))?;

    let mut active: entity::institutions::ActiveModel = institution.into();

    if let Some(name) = &model.name {
        active.name = Set(name.clone());
    }
    if let Some(license_num) = &model.license_num {
        active.license_number = Set(license_num.clone());
    }
    if let Some(regulation_num) = &model.regulation_num {
        active.regulatory_number = Set(regulation_num.clone());
    }

    active.timezone = Set(model.timezone.clone());
    active.updated_at = Set(Some(chrono::Utc::now().into()));

    ActiveModelTrait::update(active, state.pgdb.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database update error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(())
}
