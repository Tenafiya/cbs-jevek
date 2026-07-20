use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::utils::validators::{
    validate_content_type, validate_file_entity, validate_file_space, validate_snowflake,
};

#[derive(Debug, Clone)]
pub struct SetupFileUploader {
    pub owner_id: i64,
    pub file_key: String,
    pub file_name: String,
    pub mime_type: String,
    pub file_type: String,
    pub presigned_url: String,
    pub assigned_entity: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct HandlePresignResponse {
    pub id: String,
    pub url: String,
    pub expiry: chrono::TimeDelta,
}

#[derive(Debug, Clone)]
pub struct SetupFileUploaderResponse {
    pub upload_id: String,
}

#[derive(Debug, Clone)]
pub struct FieldUpdaterModel {
    pub tb: String,
    pub field: String,
    pub value: String,
    pub id: i64,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct PreSignPayload {
    #[validate(length(min = 2, max = 250, message = "fileName is invalid"))]
    #[serde(rename = "fileName")]
    pub file_name: String,

    #[validate(custom(function = "validate_content_type"))]
    #[serde(rename = "contentType")]
    pub content_type: String,

    #[validate(range(max = 5_242_880, message = "fileSize must not exceed 50MB"))]
    #[serde(rename = "fileSize")]
    pub file_size: Option<i64>,

    #[validate(custom(function = "validate_file_space"))]
    #[serde(rename = "fileSpace")]
    pub file_space: String,

    #[validate(custom(function = "validate_snowflake"))]
    #[serde(rename = "entityId")]
    pub entity_id: Option<String>,

    #[serde(default = "default_entity")]
    #[validate(custom(function = "validate_file_entity"))]
    pub entity: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct ConfirmUploadParams {
    #[validate(length(min = 4, max = 10, message = "Invalid upload id"))]
    #[serde(rename = "uploadId")]
    pub upload_id: String,

    #[validate(length(min = 4, max = 20, message = "Invalid field"))]
    pub field: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct FileRedirectParam {
    #[validate(length(min = 4, max = 10, message = "Invalid slug id"))]
    pub slug: String,
}

fn default_entity() -> Option<String> {
    Some("CUS".to_string())
}
