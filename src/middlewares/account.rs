use std::{fmt::Debug, future::Future, pin::Pin};

use actix_web::{
    Error, HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web,
};

use serde::{Serialize, de::DeserializeOwned};

use crate::{AppState, app::staffs::{models::StaffResponseModel, services::get_staff_by_session}, utils::{errors::ApiError, tokens::Claims}};

pub trait AuthSubject: DeserializeOwned + Serialize + Send + Sync + Debug + 'static {
    const CACHE_PREFIX: &'static str;
    const _CACHE_TTL: u64 = 3600;

    fn fetch(
        session: uuid::Uuid,
        state: &web::Data<AppState>,
    ) -> impl Future<Output = Result<Self, ApiError>> + Send;
}

impl AuthSubject for StaffResponseModel {
    const CACHE_PREFIX: &'static str = "staff";

    async fn fetch(session: uuid::Uuid, state: &web::Data<AppState>) -> Result<Self, ApiError> {
        get_staff_by_session(session, state)
            .await
            .map_err(|_| ApiError::Unauthorized)
    }
}

pub fn check_account<S, B>(
    state: web::Data<AppState>,
) -> impl Fn(
    ServiceRequest,
    Next<B>,
) -> Pin<Box<dyn Future<Output = Result<ServiceResponse<B>, Error>> + 'static>>
where
    S: AuthSubject,
    B: MessageBody + 'static,
{
    move |req, next| {
        let state = state.clone();

        Box::pin(async move {
            let session = req
                .extensions()
                .get::<Claims>()
                .and_then(|claim| uuid::Uuid::parse_str(&claim.sub).ok())
                .ok_or(ApiError::Unauthorized)?;

            let _cache_key = format!("{}:{}", S::CACHE_PREFIX, session);

            req.extensions_mut().insert(session);

            let account = S::fetch(session, &state).await?;

            req.extensions_mut().insert(account);
            
            Ok(next.call(req).await?)
        })
    }
}

pub mod staff {
    use super::*;

    pub fn verify<B: MessageBody + 'static>(
        state: web::Data<AppState>,
    ) -> impl Fn(
        ServiceRequest,
        Next<B>,
    ) -> Pin<Box<dyn Future<Output = Result<ServiceResponse<B>, Error>> + 'static>> {
        check_account::<StaffResponseModel, B>(state)
    }
}
