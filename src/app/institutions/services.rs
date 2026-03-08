use actix_web::web;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DbErr, EntityTrait, InsertResult,
    PaginatorTrait, QueryFilter, QueryOrder,
};

use crate::{
    AppState,
    app::institutions::models::{AddInstitutionModel, InstitutionResponseModel, UpdateInstitutionModel},
    utils::{
        gen_snow_ids::gen_snowflake_slug,
        models::{MetaModel, QueryModel},
    },
};

pub async fn init_institution(
    name: &String,
    code: &String,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::institutions::ActiveModel>, DbErr> {
    let (snowflake, _) = match gen_snowflake_slug() {
        Ok(res) => res,
        Err(_) => return Err(DbErr::Custom("Failed to generate ID's".to_string())),
    };

    let institution = entity::institutions::ActiveModel {
        id: Set(snowflake),
        code: Set(Some(code.to_string())),
        name: Set(name.to_string()),
        ..Default::default()
    };

    let insert = entity::institutions::Entity::insert(institution)
        .exec(state.pgdb.get_ref())
        .await?;

    Ok(insert)
}

pub async fn save_institution(
    model: &AddInstitutionModel,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::institutions::ActiveModel>, DbErr> {
    let (snowflake, _) = match gen_snowflake_slug() {
        Ok(res) => res,
        Err(_) => return Err(DbErr::Custom("Failed to generate ID's".to_string())),
    };

    let data = model.clone();

    let institution = entity::institutions::ActiveModel {
        id: Set(snowflake),
        name: Set(data.name),
        code: Set(Some(data.code)),
        country_id: Set(Some(data.country)),
        license_number: Set(Some(data.license_num)),
        regulatory_number: Set(Some(data.regulation_num)),
        city: Set(Some(data.city)),
        zip_code: Set(Some(data.zip_code)),
        state: Set(Some(data.state)),
        date_format: Set(Some(data.date_format)),
        date_time_format: Set(Some(data.date_time_format)),
        address: Set(Some(data.address)),
        postal_address: Set(Some(data.postal_address)),
        ..Default::default()
    };

    let insertion = entity::institutions::Entity::insert(institution)
        .exec(state.pgdb.get_ref())
        .await?;

    Ok(insertion)
}

pub async fn get_one(
    id: &i64,
    state: &web::Data<AppState>,
) -> Result<InstitutionResponseModel, DbErr> {
    let result = entity::institutions::Entity::find_by_id(*id)
        .into_model::<InstitutionResponseModel>()
        .one(state.pgdb.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Institution not found".into()));

    result
}

pub async fn get_all(
    query: &QueryModel,
    state: &web::Data<AppState>,
) -> Result<(Vec<InstitutionResponseModel>, MetaModel), DbErr> {
    let page = query.page.max(1);
    let per_page = query.size.max(1);

    let paginator = entity::institutions::Entity::find()
        .filter(
            Condition::all()
                .add(entity::institutions::Column::IsActive.eq(true))
                .add(entity::institutions::Column::IsDeleted.eq(false)),
        )
        .order_by_desc(entity::institutions::Column::UpdatedAt)
        .into_model::<InstitutionResponseModel>()
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

    active.regulatory_number = Set(model.regulation_num.clone());
    active.license_number = Set(model.license_num.clone());
    active.timezone = Set(model.timezone.clone());
    active.updated_at = Set(Some(chrono::Utc::now().into()));

    ActiveModelTrait::update(active, state.pgdb.get_ref()).await?;

    Ok(())
}
