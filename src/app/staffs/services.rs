use actix_web::web;
use migration::Expr;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbErr, EntityTrait, InsertResult,
    PaginatorTrait, QueryFilter, QueryOrder,
};

use crate::{
    AppState,
    app::staffs::models::{AddStaffModel, UpdateStaffModel, UpdateStaffStatusModel},
    utils::{
        gen_snow_ids::gen_snowflake_slug,
        models::{MetaModel, QueryModel},
    },
};

pub async fn save_staff(
    model: &AddStaffModel,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::staff::ActiveModel>, DbErr> {
    let (id, slug) = match gen_snowflake_slug() {
        Ok(res) => res,
        Err(_) => return Err(DbErr::Custom("Failed to generate ID's".to_string())),
    };

    let data = model.clone();

    let staff = entity::staff::ActiveModel {
        id: Set(id),
        institution_id: Set(data.institution_id),
        branch_id: Set(data.branch_id),
        employee_number: Set(slug),
        first_name: Set(data.first_name),
        last_name: Set(data.last_name),
        phone_number: Set(data.phone_number),
        email_address: Set(data.email),
        gender: Set(Some(data.gender)),
        nationality: Set(Some(data.nationality)),
        job_title: Set(Some(data.job_title)),
        date_hired: Set(Some(data.hired_date)),
        ..Default::default()
    };

    let inserted = entity::staff::Entity::insert(staff)
        .exec(state.pgdb.get_ref())
        .await
        .map_err(|err| DbErr::Custom(err.to_string()))?;

    Ok(inserted)
}

pub async fn update_emp_status(
    model: &UpdateStaffStatusModel,
    state: &web::Data<AppState>,
) -> Result<(), DbErr> {
    let data = model.clone();

    let staff = entity::staff::Entity::update_many()
        .filter(entity::staff::Column::Id.eq(data.id))
        .col_expr(
            entity::staff::Column::EmploymentStatus,
            Expr::value(data.employment_status),
        )
        .col_expr(
            entity::staff::Column::UpdatedAt,
            Expr::value(chrono::Utc::now()),
        )
        .exec(state.pgdb.get_ref())
        .await
        .map_err(|err| DbErr::Custom(err.to_string()))?;

    if staff.rows_affected == 0 {
        return Err(DbErr::Custom("Staff not found".to_string()));
    }

    Ok(())
}

pub async fn update_staff(
    id: &i64,
    model: &UpdateStaffModel,
    state: &web::Data<AppState>,
) -> Result<(), DbErr> {
    let staff = entity::staff::Entity::find_by_id(*id)
        .one(state.pgdb.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Staff not found".into()))?;

    let mut active_staff: entity::staff::ActiveModel = staff.into();

    let model = model.clone();

    if let Some(phone_number) = model.phone_number {
        active_staff.phone_number = Set(phone_number);
    }

    active_staff.branch_id = Set(model.branch_id);
    active_staff.date_of_birth = Set(model.data_of_birth);
    active_staff.department = Set(model.department);
    active_staff.job_title = Set(model.job_title);

    active_staff.updated_at = Set(Some(chrono::Utc::now().into()));

    ActiveModelTrait::update(active_staff, state.pgdb.get_ref())
        .await
        .map_err(|err| DbErr::Custom(err.to_string()))?;

    Ok(())
}

pub async fn get_staff_details(
    id: &i64,
    state: &web::Data<AppState>,
) -> Result<entity::staff::Model, DbErr> {
    let staff = entity::staff::Entity::find_by_id(*id)
        .one(state.pgdb.get_ref())
        .await?
        .ok_or_else(|| DbErr::Custom("Staff not found".into()));

    staff
}

pub async fn get_staff_list(
    id: &i64,
    query: &QueryModel,
    state: &web::Data<AppState>,
) -> Result<(Vec<entity::staff::Model>, MetaModel), DbErr> {
    let page = query.page;
    let per_page = query.size;

    let paginator = entity::staff::Entity::find()
        .filter(entity::staff::Column::InstitutionId.eq(*id))
        .order_by_desc(entity::staff::Column::UpdatedAt)
        .paginate(state.pgdb.get_ref(), per_page);

    let all_details = paginator.num_items_and_pages().await?;

    let items = paginator
        .fetch_page(page - 1)
        .await
        .map_err(|err| DbErr::Custom(err.to_string()))?;

    let meta = MetaModel {
        total_items: all_details.number_of_items,
        total_pages: all_details.number_of_pages,
        page,
        per_page,
    };

    Ok((items, meta))
}

pub async fn set_supervisor(
    id: &i64,
    supervisor: &i64,
    state: &web::Data<AppState>,
) -> Result<(), DbErr> {
    let staff = entity::staff::Entity::update_many()
        .filter(entity::staff::Column::Id.eq(*id))
        .col_expr(
            entity::staff::Column::SupervisorId,
            Expr::value(*supervisor),
        )
        .col_expr(
            entity::staff::Column::UpdatedAt,
            Expr::value(chrono::Utc::now()),
        )
        .exec(state.pgdb.get_ref())
        .await
        .map_err(|err| DbErr::Custom(err.to_string()))?;

    if staff.rows_affected == 0 {
        return Err(DbErr::Custom("Staff not found".to_string()));
    };

    Ok(())
}
