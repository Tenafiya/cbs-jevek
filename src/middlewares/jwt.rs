use actix_web::{
    Error, HttpMessage, dev::{ServiceRequest, ServiceResponse}, middleware::Next
};

use crate::utils::{errors::ApiError, tokens::verify_jwt};

pub async fn jwt_auth(
    req: ServiceRequest,
    next: Next<impl actix_web::body::MessageBody>,
) -> Result<ServiceResponse<impl actix_web::body::MessageBody>, Error> {
    let claims = verify_jwt(req.request())
        .await
        .map_err(|_| ApiError::Unauthorized)?;

    req.extensions_mut().insert(claims);

    let res = next.call(req).await?;

    Ok(res)
}