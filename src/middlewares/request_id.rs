use actix_http::header::HeaderValue;
use actix_web::{
    Error,
    HttpMessage,
    dev::{ServiceRequest, ServiceResponse},
    http::header::HeaderName,
    middleware::Next,
};
use std::sync::Arc;

use crate::utils::{errors::ApiError, headers};

pub async fn request_id(
    req: ServiceRequest,
    next: Next<impl actix_web::body::MessageBody>,
) -> Result<ServiceResponse<impl actix_web::body::MessageBody>, Error> {
    let request_id = match headers::extract_header(&req.request(), "x-request-id").await {
        Ok(id) => id,
        Err(_) => uuid::Uuid::new_v4().to_string(),
    };

    req.extensions_mut().insert(Arc::new(request_id.clone()));

    let mut res = next
        .call(req)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    if let Ok(value) = HeaderValue::from_str(&request_id) {
        res.headers_mut()
            .insert(HeaderName::from_static("x-request-id"), value);
    }

    Ok(res)
}
