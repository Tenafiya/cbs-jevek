use actix_web::web;
use migration::Expr;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, Condition, DbErr, EntityTrait, InsertResult, QueryFilter,
};

use crate::{AppState, app::countries::models::CountryResponseModel, utils};

use crate::app::countries::models::AddCountryModel;

pub async fn save_country(
    model: &AddCountryModel,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::countries::ActiveModel>, DbErr> {
    let (snowflake, slug) = utils::gen_snow_ids::gen_snowflake_slug().unwrap_or_else(|e| {
        println!("snowflake error: {e}; falling back to 0");
        (0, "0".into())
    });

    let data = model.clone();

    let country = entity::countries::ActiveModel {
        id: Set(snowflake),
        slug: Set(slug),
        name: Set(data.name),
        official_name: Set(Some(data.official_name)),
        capital_city: Set(Some(data.capital_city)),
        currency: Set(Some(data.currency)),
        flag_url: Set(Some(data.flag_url)),
        call_code: Set(data.call_code),
        iso_code: Set(data.iso_code),
        more_data: Set(data.more_data),
        ..Default::default()
    };

    let insertion = entity::countries::Entity::insert(country)
        .exec(state.pgdb.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database insert error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(insertion)
}

pub async fn get_all(state: &web::Data<AppState>) -> Result<Vec<CountryResponseModel>, DbErr> {
    let results = entity::countries::Entity::find()
        .filter(
            Condition::all()
                .add(entity::countries::Column::IsActive.eq(true))
                .add(entity::countries::Column::IsDeleted.eq(false)),
        )
        .all(state.pgdb.as_ref())
        .await
        .map_err(|err| {
            eprintln!("Database retrieval error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    let countries: Vec<CountryResponseModel> = results
        .into_iter()
        .map(CountryResponseModel::from)
        .collect();

    Ok(countries)
}

pub async fn get_one(
    id: &String,
    state: &web::Data<AppState>,
) -> Result<CountryResponseModel, DbErr> {
    let result = entity::countries::Entity::find()
        .filter(
            Condition::all()
                .add(entity::countries::Column::Slug.eq(id))
                .add(entity::countries::Column::IsActive.eq(true))
                .add(entity::countries::Column::IsDeleted.eq(false)),
        )
        .one(state.pgdb.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Country not found".into()));

    let country = result?;

    Ok(CountryResponseModel::from(country))
}

pub async fn operate_country(
    id: &String,
    operation: &String,
    state: &web::Data<AppState>,
) -> Result<(), DbErr> {
    let country = match operation.as_str() {
        "TOGGLE" => entity::countries::Entity::update_many()
            .filter(entity::countries::Column::Slug.eq(id))
            .col_expr(
                entity::countries::Column::IsActive,
                Expr::col(entity::countries::Column::IsActive).not(),
            )
            .col_expr(
                entity::countries::Column::UpdatedAt,
                Expr::value(chrono::Utc::now()),
            )
            .exec(state.pgdb.as_ref())
            .await
            .map_err(|err| DbErr::Custom(err.to_string()))?,
        "DELETE" => entity::countries::Entity::update_many()
            .filter(entity::countries::Column::Slug.eq(id))
            .col_expr(
                entity::countries::Column::IsDeleted,
                Expr::col(entity::countries::Column::IsDeleted).not(),
            )
            .col_expr(
                entity::countries::Column::UpdatedAt,
                Expr::value(chrono::Utc::now()),
            )
            .exec(state.pgdb.as_ref())
            .await
            .map_err(|err| DbErr::Custom(err.to_string()))?,
        _ => return Err(DbErr::Custom("Invalid Operation".to_string())),
    };

    if country.rows_affected == 0 {
        return Err(DbErr::Custom("Failed To Update Country".to_string()));
    };

    Ok(())
}
