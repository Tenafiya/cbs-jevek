use actix_web::{HttpRequest, HttpResponse, http::header, web};
use sea_orm::TransactionTrait;
use validator::Validate;

use crate::{
    AppState,
    app::{
        media::{
            models::{
                ConfirmUploadParams, FieldUpdaterModel, FileRedirectParam, HandlePresignResponse,
                PreSignPayload, SetupFileUploader,
            },
            services,
        },
        staffs::models::StaffResponseModel,
    },
    fileskit::models::{FileExistPayload, GenUploadUrlPayload},
    utils::{
        errors::{ApiCode, ApiError, ApiResponse},
        gen_snow_ids,
    },
};

pub async fn handle_presign(
    _req: HttpRequest,
    state: web::Data<AppState>,
    staff: web::ReqData<StaffResponseModel>,
    payload: web::Json<PreSignPayload>,
) -> Result<HttpResponse, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let StaffResponseModel { id, .. } = staff.into_inner();

    let data = payload.into_inner();

    let owner_id = gen_snow_ids::id_parser(&data.entity_id, "Owner ID")?;

    let expires_in = chrono::Duration::minutes(15);

    let file_name = data.file_name.replace(['/', '\\', ' '], "_");

    let file_key = format!(
        "{}/{}/{}_{}",
        data.file_space,
        owner_id,
        chrono::Utc::now().timestamp(),
        file_name
    );

    let genner = GenUploadUrlPayload {
        file_key: file_key.clone(),
        bucket: data.file_space.clone(),
        expires_in: expires_in
            .to_std()
            .map_err(|e| ApiError::BadRequest(e.to_string()))?,
    };

    let upload_url = state
        .storage
        .generate_upload_url(&genner)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    let uploader = SetupFileUploader {
        owner_id,
        file_key: file_key.clone(),
        file_name: data.file_name,
        mime_type: data.content_type,
        file_type: data.file_space,
        presigned_url: upload_url.clone(),
        uploaded_by: id,
    };

    let res = services::create_file(&uploader, &state)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(
        ApiCode::OperationSuccess,
        "Successful",
        HandlePresignResponse {
            id: res.upload_id,
            url: upload_url,
            expiry: expires_in,
        },
    )))
}

pub async fn handle_upload_confirm(
    _req: HttpRequest,
    state: web::Data<AppState>,
    payload: web::Json<ConfirmUploadParams>,
) -> Result<HttpResponse, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let data = payload.into_inner();

    let txn = state
        .pgdb
        .get_ref()
        .begin()
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    let upload = services::upload_exists(&data.upload_id, &txn)
        .await
        .map_err(|_| ApiError::NotFound)?;

    let path = format!("/v1/media/{}", upload.slug);

    let file_type = upload
        .file_type
        .clone()
        .ok_or_else(|| ApiError::BadRequest("File type is missing".to_string()))?;

    let storage_exist = FileExistPayload {
        file_key: upload.file_key.clone(),
        bucket: file_type.clone(),
    };

    if !state
        .storage
        .file_exists(&storage_exist)
        .await
        .map_err(|_| ApiError::NotFound)?
    {
        return Err(ApiError::NotFound);
    };

    services::set_upload_completion(&upload.slug, &txn)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    let updater = FieldUpdaterModel {
        tb: "TB NAME HERE".to_string(),
        field: data
            .field
            .ok_or_else(|| ApiError::BadRequest("Field is needed".to_string()))?,
        value: path,
        id: upload
            .owner_id
            .ok_or_else(|| ApiError::BadRequest("Owner is missing".to_string()))?,
    };

    services::field_updater(&updater, &txn)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    txn.commit()
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(
        ApiCode::OperationSuccess,
        "Successful",
        {},
    )))
}

pub async fn file_redirect(
    _req: HttpRequest,
    state: web::Data<AppState>,
    params: web::Path<FileRedirectParam>,
) -> Result<HttpResponse, ApiError> {
    params
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let path = params.into_inner();

    let txn = state
        .pgdb
        .get_ref()
        .begin()
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    let upload = services::upload_exists(&path.slug, &txn)
        .await
        .map_err(|_| ApiError::NotFound)?;

    txn.commit()
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    let expires_in = chrono::Duration::minutes(60);

    let genner = GenUploadUrlPayload {
        file_key: upload.file_key,
        bucket: upload
            .file_type
            .ok_or_else(|| ApiError::BadRequest("File type is missing".to_string()))?,
        expires_in: expires_in
            .to_std()
            .map_err(|e| ApiError::BadRequest(e.to_string()))?,
    };

    let presigned_url = state
        .storage
        .generate_download_url(&genner)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(HttpResponse::TemporaryRedirect()
        .append_header((header::LOCATION, presigned_url))
        .append_header((
            header::CACHE_CONTROL,
            "public, max-age=3600, must-revalidate",
        ))
        .finish())
}
