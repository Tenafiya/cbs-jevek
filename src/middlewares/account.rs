use redis::AsyncCommands;
use std::{fmt::Debug, future::Future, pin::Pin};

use actix_web::{
    Error, HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web,
};

use serde::{Serialize, de::DeserializeOwned};

#[derive(Clone, Debug)]
pub struct SessionId(pub uuid::Uuid);

use crate::{
    AppState,
    app::staffs::{models::StaffResponseModel, services::get_staff_by_session},
    utils::{errors::ApiError, tokens::Claims},
};

pub trait AuthSubject: DeserializeOwned + Serialize + Send + Sync + Debug + 'static {
    const CACHE_PREFIX: &'static str;
    const CACHE_TTL: u64 = 3600;

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

            let mut conn = state.cache.get_ref().clone();

            let cache_key = format!("{}:{}", S::CACHE_PREFIX, session);

            req.extensions_mut().insert(SessionId(session));

            let account: S = match conn.get::<_, String>(&cache_key).await {
                Ok(cached) => {
                    serde_json::from_str(&cached).map_err(|_| ApiError::InternalServerError)?
                }
                Err(_) => {
                    let subject = S::fetch(session, &state).await?;

                    let json = serde_json::to_string(&subject)
                        .map_err(|_| ApiError::InternalServerError)?;

                    let _: () = conn
                        .set_ex(&cache_key, json, S::CACHE_TTL)
                        .await
                        .map_err(|_| ApiError::InternalServerError)?;

                    subject
                }
            };

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
