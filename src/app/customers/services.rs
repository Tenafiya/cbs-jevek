use actix_web::web;
use migration::Expr;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DbErr, EntityTrait, InsertResult,
    PaginatorTrait, QueryFilter, QueryOrder,
};

use crate::{
    AppState,
    app::customers::models::{AddAddressModel, AddCustomerModel, AddNextModel, AddOccupationModel},
    utils::{
        gen_snow_ids::gen_snowflake_slug,
        models::{MetaModel, QueryModel},
    },
};

pub async fn save_customer(
    model: &AddCustomerModel,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::customers::ActiveModel>, DbErr> {
    let (snowflake, _) = gen_snowflake_slug().unwrap_or_else(|_| (0, "0".into()));

    let data = model.clone();

    let customer = entity::customers::ActiveModel {
        id: Set(snowflake),
        institution_id: Set(data.institution_id),
        customer_number: Set(Some(data.customer_num)),
        first_name: Set(Some(data.first_name)),
        last_name: Set(Some(data.last_name)),
        middle_name: Set(data.middle_name),
        date_of_birth: Set(Some(data.birth_date)),
        gender: Set(Some(data.gender)),
        nationality: Set(Some(data.nationality)),
        phone_number: Set(Some(data.phone_number)),
        phone_country_code: Set(Some(data.phone_country_code)),
        email: Set(data.email),
        created_by: Set(Some(data.created_by)),
        ..Default::default()
    };

    let insertion = entity::customers::Entity::insert(customer)
        .exec(state.pgdb.get_ref())
        .await
        .map_err(|err| DbErr::Custom(err.to_string()))?;

    Ok(insertion)
}

pub async fn add_address(
    id: &i64,
    model: &AddAddressModel,
    state: &web::Data<AppState>,
) -> Result<(), DbErr> {
    let customer = entity::customers::Entity::find_by_id(*id)
        .one(state.pgdb.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Customer not found".into()))?;

    let mut active_customer: entity::customers::ActiveModel = customer.into();

    let model = model.clone();

    active_customer.residential_address = Set(model.residential_address);
    active_customer.postal_address = Set(model.postal_address);
    active_customer.city = Set(model.city);
    active_customer.country_id = Set(model.country_id);
    active_customer.state_province = Set(model.state);

    active_customer.updated_at = Set(Some(chrono::Utc::now().into()));

    ActiveModelTrait::update(active_customer, state.pgdb.get_ref())
        .await
        .map_err(|err| DbErr::Custom(err.to_string()))?;

    Ok(())
}

pub async fn add_occupation(
    id: &i64,
    model: &AddOccupationModel,
    state: &web::Data<AppState>,
) -> Result<(), DbErr> {
    let customer = entity::customers::Entity::find_by_id(*id)
        .one(state.pgdb.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Customer not found".into()))?;

    let mut active_customer: entity::customers::ActiveModel = customer.into();

    let model = model.clone();

    active_customer.occupation = Set(Some(model.occupation));
    active_customer.employer_name = Set(model.employer_name);
    active_customer.income_source = Set(model.income_source);
    active_customer.monthly_income = Set(Some(model.monthly_income));

    active_customer.updated_at = Set(Some(chrono::Utc::now().into()));

    ActiveModelTrait::update(active_customer, state.pgdb.get_ref())
        .await
        .map_err(|err| DbErr::Custom(err.to_string()))?;

    Ok(())
}

pub async fn add_next_details(
    id: &i64,
    model: &AddNextModel,
    state: &web::Data<AppState>,
) -> Result<(), DbErr> {
    let customer = entity::customers::Entity::find_by_id(*id)
        .one(state.pgdb.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Customer not found".into()))?;

    let mut active_customer: entity::customers::ActiveModel = customer.into();

    let model = model.clone();

    active_customer.next_of_kin = Set(model.next_of_kin);
    active_customer.pep_details = Set(model.pep_details);

    active_customer.updated_at = Set(Some(chrono::Utc::now().into()));

    ActiveModelTrait::update(active_customer, state.pgdb.get_ref())
        .await
        .map_err(|err| DbErr::Custom(err.to_string()))?;

    Ok(())
}

pub async fn verify_email(id: &i64, state: &web::Data<AppState>) -> Result<(), DbErr> {
    let customer = entity::customers::Entity::update_many()
        .filter(
            Condition::all()
                .add(entity::customers::Column::Id.eq(*id))
                .add(entity::customers::Column::IsEmailVerified.eq(false)),
        )
        .col_expr(
            entity::customers::Column::IsEmailVerified,
            Expr::col(entity::customers::Column::IsEmailVerified).not(),
        )
        .col_expr(
            entity::customers::Column::EmailVerifiedAt,
            Expr::value(chrono::Utc::now()),
        )
        .exec(state.pgdb.get_ref())
        .await
        .map_err(|err| DbErr::Custom(err.to_string()))?;

    if customer.rows_affected == 0 {
        return Err(DbErr::Custom("Customer not found".into()));
    };

    Ok(())
}

pub async fn verify_phone(id: &i64, state: &web::Data<AppState>) -> Result<(), DbErr> {
    let customer = entity::customers::Entity::update_many()
        .filter(
            Condition::all()
                .add(entity::customers::Column::Id.eq(*id))
                .add(entity::customers::Column::IsPhoneVerified.eq(false)),
        )
        .col_expr(
            entity::customers::Column::IsPhoneVerified,
            Expr::col(entity::customers::Column::IsPhoneVerified).not(),
        )
        .col_expr(
            entity::customers::Column::PhoneVerifiedAt,
            Expr::value(chrono::Utc::now()),
        )
        .exec(state.pgdb.get_ref())
        .await
        .map_err(|err| DbErr::Custom(err.to_string()))?;

    if customer.rows_affected == 0 {
        return Err(DbErr::Custom("Customer not found".into()));
    };

    Ok(())
}

pub async fn get_details(
    id: &i64,
    state: &web::Data<AppState>,
) -> Result<entity::customers::Model, DbErr> {
    let customer = entity::customers::Entity::find_by_id(*id)
        .one(state.pgdb.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Customer not found".into()));

    customer
}

pub async fn get_customers(
    id: &i64,
    query: &QueryModel,
    state: &web::Data<AppState>,
) -> Result<(Vec<entity::customers::Model>, MetaModel), DbErr> {
    let page = query.page.max(1);
    let per_page = query.size.max(1);

    let paginator = entity::customers::Entity::find()
        .filter(
            Condition::all()
                .add(entity::customers::Column::InstitutionId.eq(*id))
                .add(entity::customers::Column::IsBlackListed.eq(false)),
        )
        .order_by_desc(entity::customers::Column::UpdatedAt)
        .paginate(state.pgdb.get_ref(), per_page);

    let all = paginator.num_items_and_pages().await?;

    let items = paginator
        .fetch_page(page - 1)
        .await
        .map_err(|err| DbErr::Custom(err.to_string()))?;

    let meta = MetaModel {
        total_items: all.number_of_items,
        total_pages: all.number_of_pages,
        page,
        per_page,
    };

    Ok((items, meta))
}

pub async fn update_sanctions(
    id: &i64,
    provider: &String,
    state: &web::Data<AppState>,
) -> Result<(), DbErr> {
    let customer = entity::customers::Entity::update_many()
        .filter(
            Condition::all()
                .add(entity::customers::Column::Id.eq(*id))
                .add(entity::customers::Column::IsSanctionsCheckPassed.eq(false)),
        )
        .col_expr(
            entity::customers::Column::IsSanctionsCheckPassed,
            Expr::col(entity::customers::Column::IsSanctionsCheckPassed).not(),
        )
        .col_expr(
            entity::customers::Column::SanctionsProvider,
            Expr::value(provider),
        )
        .col_expr(
            entity::customers::Column::SanctionsCheckDate,
            Expr::value(chrono::Utc::now()),
        )
        .exec(state.pgdb.get_ref())
        .await
        .map_err(|err| DbErr::Custom(err.to_string()))?;

    if customer.rows_affected == 0 {
        return Err(DbErr::Custom("Customer not found".into()));
    };

    Ok(())
}

pub async fn customer_verify(
    id: &i64,
    user: &i64,
    state: &web::Data<AppState>,
) -> Result<(), DbErr> {
    let customer = entity::customers::Entity::update_many()
        .filter(
            Condition::all()
                .add(entity::customers::Column::Id.eq(*id))
                .add(entity::customers::Column::VerifiedAt.is_null()),
        )
        .col_expr(
            entity::customers::Column::VerifiedAt,
            Expr::value(chrono::Utc::now()),
        )
        .col_expr(entity::customers::Column::VerifiedBy, Expr::value(*user))
        .exec(state.pgdb.get_ref())
        .await
        .map_err(|err| DbErr::Custom(err.to_string()))?;

    if customer.rows_affected == 0 {
        return Err(DbErr::Custom("Customer not found".into()));
    };

    Ok(())
}

pub async fn customer_delete(id: &i64, state: &web::Data<AppState>) -> Result<(), DbErr> {
    let customer = entity::customers::Entity::update_many()
        .filter(
            Condition::all()
                .add(entity::customers::Column::Id.eq(*id))
                .add(entity::customers::Column::IsDeleted.eq(false)),
        )
        .col_expr(
            entity::customers::Column::IsDeleted,
            Expr::col(entity::customers::Column::IsDeleted).not(),
        )
        .col_expr(
            entity::customers::Column::DeletedAt,
            Expr::value(chrono::Utc::now()),
        )
        .exec(state.pgdb.get_ref())
        .await
        .map_err(|err| DbErr::Custom(err.to_string()))?;

    if customer.rows_affected == 0 {
        return Err(DbErr::Custom("Customer not found".into()));
    };

    Ok(())
}
