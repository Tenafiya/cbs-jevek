use aws_config::BehaviorVersion;
use aws_config::Region;
use aws_sdk_s3::Client as S3Client;
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::presigning::PresigningConfig;

use crate::fileskit::errors::RustFsError;
use crate::fileskit::models::FileExistPayload;
use crate::fileskit::models::GenUploadUrlPayload;

pub struct StorageService {
    client: S3Client,
}

impl StorageService {
    pub async fn new() -> Self {
        let access_key = std::env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID must be set");
        let secret_key =
            std::env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY must be set");

        let credentials = Credentials::new(&access_key, &secret_key, None, None, "env-credentials");

        let endpoint = std::env::var("RUSTFS_PUBLIC_ENDPOINT").ok();

        let mut config_loader = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new("us-east-1"))
            .credentials_provider(credentials);

        if let Some(endpoint_url) = endpoint {
            config_loader = config_loader.endpoint_url(&endpoint_url);
        }

        let config = config_loader.load().await;
        
        let client = S3Client::new(&config);

        Self { client }
    }

    pub async fn generate_upload_url(
        &self,
        payload: &GenUploadUrlPayload,
    ) -> Result<String, RustFsError> {
        let presigned_request = self
            .client
            .put_object()
            .bucket(&payload.bucket)
            .key(&payload.file_key)
            .presigned(
                PresigningConfig::expires_in(payload.expires_in).map_err(|e| {
                    RustFsError::RustfsReqError {
                        status: false,
                        description: e.to_string(),
                    }
                })?,
            )
            .await
            .map_err(|e| RustFsError::RustfsReqError {
                status: false,
                description: e.to_string(),
            })?;

        Ok(presigned_request.uri().to_string())
    }

    pub async fn generate_download_url(
        &self,
        payload: &GenUploadUrlPayload,
    ) -> Result<String, RustFsError> {
        let presigned_request = self
            .client
            .get_object()
            .bucket(&payload.bucket)
            .key(&payload.file_key)
            .presigned(
                PresigningConfig::expires_in(payload.expires_in).map_err(|e| {
                    RustFsError::RustfsReqError {
                        status: false,
                        description: e.to_string(),
                    }
                })?,
            )
            .await
            .map_err(|e| RustFsError::RustfsReqError {
                status: false,
                description: e.to_string(),
            })?;

        Ok(presigned_request.uri().to_string())
    }

    pub async fn file_exists(&self, payload: &FileExistPayload) -> Result<bool, RustFsError> {
        self.client
            .head_object()
            .bucket(&payload.bucket)
            .key(&payload.file_key)
            .send()
            .await
            .map(|_| true)
            .map_err(|e| RustFsError::RustfsReqError {
                status: false,
                description: e.to_string(),
            })
    }

    pub async fn del_file(&self, payload: &FileExistPayload) -> Result<(), RustFsError> {
        self.client
            .delete_object()
            .bucket(&payload.bucket)
            .key(&payload.file_key)
            .send()
            .await
            .map_err(|e| RustFsError::RustfsReqError {
                status: false,
                description: e.to_string(),
            })?;

        Ok(())
    }

    pub async fn health_check(&self) -> Result<bool, RustFsError> {
        self.client
            .list_buckets()
            .send()
            .await
            .map(|_| true)
            .map_err(|e| RustFsError::RustfsReqError {
                status: false,
                description: e.to_string(),
            })
    }
}