use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiCode {
    // Success codes
    OperationSuccess,
    ResourceCreated,
    RequestAccepted,
    RequestDenied,

    // Error codes
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    ValidationFailed,
    InternalServerError,
    RateLimitExhaused,

    // Server error codes
    DatabasError,
    ExternalServiceError,
    ProcessingError,
    InternalError,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub code: ApiCode,
    pub message: String,
    pub data: Option<T>,
    pub request_id: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(code: ApiCode, message: &str, data: T, request_id: Option<String>) -> Self {
        Self {
            success: true,
            code,
            message: message.to_string(),
            data: Some(data),
            request_id,
        }
    }

    pub fn error(code: ApiCode, message: &str, request_id: Option<String>) -> Self {
        Self {
            success: false,
            code,
            message: message.to_string(),
            data: None,
            request_id,
        }
    }
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Not found")]
    NotFound,

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Validation failed: {0}")]
    Unprocessable(String),

    #[error("Rate limit exceeded")]
    TooManyRequests,

    #[error("Internal server error")]
    InternalServerError,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::BadRequest(msg) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                ApiCode::BadRequest,
                msg,
                None,
            )),

            ApiError::Unauthorized => HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                ApiCode::Unauthorized,
                "Missing or invalid credentials",
                None,
            )),

            ApiError::Forbidden => HttpResponse::Forbidden().json(ApiResponse::<()>::error(
                ApiCode::Forbidden,
                "Access denied",
                None,
            )),

            ApiError::NotFound => HttpResponse::NotFound().json(ApiResponse::<()>::error(
                ApiCode::NotFound,
                "Resource not found",
                None,
            )),

            ApiError::Conflict(msg) => HttpResponse::Conflict().json(ApiResponse::<()>::error(
                ApiCode::Conflict,
                msg,
                None,
            )),

            ApiError::Unprocessable(msg) => HttpResponse::UnprocessableEntity().json(
                ApiResponse::<()>::error(ApiCode::ValidationFailed, msg, None),
            ),

            ApiError::TooManyRequests => HttpResponse::TooManyRequests().json(
                ApiResponse::<()>::error(ApiCode::RateLimitExhaused, "Try again later", None),
            ),

            ApiError::InternalServerError => {
                HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                    ApiCode::InternalServerError,
                    "Unexpected server error",
                    None,
                ))
            }

            _ => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                ApiCode::InternalServerError,
                "Unexpected server error",
                None,
            )),
        }
    }
}
