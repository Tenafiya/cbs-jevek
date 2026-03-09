use actix_web::web;
use migration::Expr;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DbErr, EntityTrait, InsertResult,
    PaginatorTrait, QueryFilter, QueryOrder,
};

use crate::{
    AppState,
    app::staffs::models::{
        AddStaffModel, GetAuthModel, SetupStaff, StaffResponseModel, UpdateStaffModel,
        UpdateStaffStatusModel,
    },
    utils::{
        gen_snow_ids::gen_snowflake_slug,
        models::{MetaModel, QueryModel},
        password::validate_password,
    },
};

pub async fn init_staff(
    model: &SetupStaff,
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
        first_name: Set(data.first_name),
        last_name: Set(data.last_name),
        phone_number: Set(data.phone_number),
        email_address: Set(data.email),
        employee_number: Set(slug),
        password_hash: Set(data.password),
        salt: Set(data.salt),
        ..Default::default()
    };

    let inserted = entity::staff::Entity::insert(staff)
        .exec(state.pgdb.get_ref())
        .await?;

    Ok(inserted)
}

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
        password_hash: Set(data.password),
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
    active_staff.date_of_birth = Set(model.date_of_birth);
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
) -> Result<StaffResponseModel, DbErr> {
    let staff = entity::staff::Entity::find_by_id(*id)
        .into_model::<StaffResponseModel>()
        .one(state.pgdb.get_ref())
        .await?
        .ok_or_else(|| DbErr::Custom("Staff not found".into()));

    staff
}

pub async fn get_staff_list(
    id: &i64,
    query: &QueryModel,
    state: &web::Data<AppState>,
) -> Result<(Vec<StaffResponseModel>, MetaModel), DbErr> {
    let page = query.page;
    let per_page = query.size;

    let paginator = entity::staff::Entity::find()
        .filter(entity::staff::Column::InstitutionId.eq(*id))
        .order_by_desc(entity::staff::Column::UpdatedAt)
        .into_model::<StaffResponseModel>()
        .paginate(state.pgdb.get_ref(), per_page);

    let staffs = paginator.fetch_page(page - 1).await?;

    let total_items = paginator.num_items().await?;
    let total_pages = (total_items + per_page - 1) / per_page;

    let meta = MetaModel {
        total_items,
        total_pages,
        page,
        per_page,
    };

    Ok((staffs, meta))
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

pub async fn update_failed_login(email: &String, state: &web::Data<AppState>) -> Result<(), DbErr> {
    let staff = entity::staff::Entity::update_many()
        .filter(entity::staff::Column::EmailAddress.eq(email))
        .col_expr(
            entity::staff::Column::FailedLoginAttempts,
            Expr::col(entity::staff::Column::FailedLoginAttempts).add(1),
        )
        .exec(state.pgdb.get_ref())
        .await?;

    if staff.rows_affected == 0 {
        return Err(DbErr::Custom("Staff not found".to_string()));
    };

    Ok(())
}

pub async fn signin_auth(
    model: &GetAuthModel,
    state: &web::Data<AppState>,
) -> Result<StaffResponseModel, DbErr> {
    let data = model.clone();

    let staff = entity::staff::Entity::find()
        .filter(Condition::all().add(entity::staff::Column::EmailAddress.eq(data.email)))
        .one(state.pgdb.get_ref())
        .await?
        .ok_or_else(|| DbErr::Custom("Staff not found".into()))?;

    let salt = &staff.salt;

    let hashed_password = &staff.password_hash;

    if !validate_password(&data.password, &salt, &hashed_password).await {
        return Err(DbErr::Custom("Invalid credentials".to_string()));
    }

    let mut active_users: entity::staff::ActiveModel = staff.into();

    active_users.session = Set(Some(uuid::Uuid::new_v4()));
    active_users.updated_at = Set(Some(
        chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap()),
    ));

    let updated = ActiveModelTrait::update(active_users, state.pgdb.get_ref()).await?;

    let staff_res = StaffResponseModel {
        id: updated.id,
        institution_id: updated.institution_id,
        branch_id: updated.branch_id,
        employee_number: updated.employee_number,
        first_name: updated.first_name,
        last_name: updated.last_name,
        full_name: updated.full_name,
        phone_number: updated.phone_number,
        email_address: updated.email_address,
        date_of_birth: updated.date_of_birth,
        gender: updated.gender,
        job_title: updated.job_title,
        nationality: updated.nationality,
        department: updated.department,
        employment_status: updated.employment_status,
        date_hired: updated.date_hired,
        date_terminated: updated.date_terminated,
        termination_reason: updated.termination_reason,
        role_id: updated.role_id,
        permissions: updated.permissions,
        is_password_changed: updated.is_password_changed,
        password_last_changed_at: updated.password_last_changed_at,
        is_mfa_enabled: updated.is_mfa_enabled,
        session: updated.session,
        performance_rating: updated.performance_rating,
        last_appraisal_date: updated.last_appraisal_date,
        supervisor_id: updated.supervisor_id,
        custom_fields: updated.custom_fields,
        created_at: updated.created_at,
        updated_at: updated.updated_at,
    };

    Ok(staff_res)
}

// exec_with_returning and into_model usage
// pub async fn set_supervisor(
//     id: &i64,
//     supervisor: &i64,
//     state: &web::Data<AppState>,
// ) -> Result<entity::staff::Model, DbErr> {
//     let updated_staff: Option<entity::staff::Model> = entity::staff::Entity::update_many()
//         .filter(entity::staff::Column::Id.eq(*id))
//         .col_expr(entity::staff::Column::SupervisorId, Expr::value(*supervisor))
//         .col_expr(entity::staff::Column::UpdatedAt, Expr::value(chrono::Utc::now()))
//         .exec_with_returning(state.pgdb.get_ref())
//         .await?
//         .into_model();

//     updated_staff.ok_or_else(|| DbErr::Custom("Staff not found".to_string()))
// }
