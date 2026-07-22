use actix_web::web;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, ConnectionTrait, DatabaseBackend,
    DatabaseTransaction, DbErr, EntityTrait, QueryFilter, Statement,
};

use crate::{
    AppState,
    app::media::models::{FieldUpdaterModel, SetupFileUploader, SetupFileUploaderResponse},
    utils::gen_snow_ids,
};

pub async fn create_file(
    model: &SetupFileUploader,
    state: &web::Data<AppState>,
) -> Result<SetupFileUploaderResponse, DbErr> {
    use entity::file_uploads::{ActiveModel, Entity};

    let (snowflake, slug) = gen_snow_ids::gen_snowflake_slug()
        .map_err(|_| DbErr::Custom("Failed to generate IDs".to_string()))?;

    let now = chrono::Utc::now();

    let data = model.clone();

    let assigned_entity = data
        .assigned_entity
        .as_deref()
        .map(get_file_entity)
        .transpose()?
        .ok_or(DbErr::Custom("Invalid entity".to_string()))?;

    let uploader = ActiveModel {
        id: Set(snowflake),
        slug: Set(slug.clone()),
        owner_id: Set(Some(data.owner_id)),
        file_key: Set(data.file_key),
        file_name: Set(data.file_name),
        mime_type: Set(Some(data.mime_type)),
        file_type: Set(Some(data.file_type)),
        presigned_url: Set(data.presigned_url),
        uploaded_by: Set(data.uploaded_by),
        assigned_entity: Set(Some(assigned_entity.to_string())),
        url_expires_at: Set((now + chrono::Duration::minutes(15)).into()),
        ..Default::default()
    };

    Entity::insert(uploader).exec(state.pgdb.get_ref()).await?;

    Ok(SetupFileUploaderResponse { upload_id: slug })
}

pub async fn upload_exists(
    upload_id: &str,
    tx: &DatabaseTransaction,
) -> Result<entity::file_uploads::Model, DbErr> {
    use entity::file_uploads::{Column, Entity};

    Entity::find()
        .filter(
            Condition::all()
                .add(Column::Slug.eq(upload_id))
                .add(Column::UploadedAt.is_null()),
        )
        .one(tx)
        .await?
        .ok_or_else(|| DbErr::Custom("Upload not found".to_string()))
}

pub async fn set_upload_completion(upload_id: &str, tx: &DatabaseTransaction) -> Result<(), DbErr> {
    use entity::file_uploads::{ActiveModel, Column, Entity};

    let upload = Entity::find()
        .filter(
            Condition::all()
                .add(Column::Slug.eq(upload_id))
                .add(Column::UploadedAt.is_null()),
        )
        .one(tx)
        .await?
        .ok_or_else(|| DbErr::Custom("Upload not found or already completed".to_string()))?;

    let mut active_model: ActiveModel = upload.into();

    active_model.uploaded_at = Set(Some(chrono::Utc::now().into()));

    active_model.update(tx).await?;

    Ok(())
}

pub async fn field_updater(
    model: &FieldUpdaterModel,
    tx: &DatabaseTransaction,
) -> Result<(), DbErr> {
    let data = model.clone();

    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        format!(
            r#"
            UPDATE {}
            SET {} = $1,
            WHERE id = $2
            "#,
            data.tb, data.field
        ),
        vec![data.value.into(), data.id.into()],
    );

    let result = tx.execute(stmt).await?;

    if result.rows_affected() == 0 {
        return Err(DbErr::Custom("Could not update".to_string()));
    }

    Ok(())
}

#[inline(always)]
fn get_file_entity(entity: &str) -> Result<&'static str, DbErr> {
    match entity {
        "CUS" => Ok("customers"),
        _ => Err(DbErr::Custom(format!("Unknown entity: {}", entity))),
    }
}