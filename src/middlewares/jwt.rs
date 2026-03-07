use actix_web::{
    Error, HttpMessage, dev::{ServiceRequest, ServiceResponse}, error::ErrorUnauthorized, middleware::Next
};
use std::sync::Arc;

use crate::utils::{errors::ApiError, tokens::verify_jwt};

pub async fn jwt_auth(
    req: ServiceRequest,
    next: Next<impl actix_web::body::MessageBody>,
) -> Result<ServiceResponse<impl actix_web::body::MessageBody>, Error> {
    match verify_jwt(req.request()).await {
        Ok(claims) => {
            req.extensions_mut().insert(Arc::new(claims));
            let res = next.call(req).await.map_err(|_| ApiError::Unauthorized)?;
            Ok(res)
        }
        Err(_) => Err(ErrorUnauthorized("Unauthorized")),
    }
}