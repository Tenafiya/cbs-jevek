use actix_web::{
    Error, HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web,
};
use redis::AsyncCommands;
use std::pin::Pin;

use crate::{
    AppState,
    app::{
        staffs::models::StaffResponseModel,
        tellers::{self},
    },
    middlewares::account::SessionId,
    utils::errors::ApiError,
};

pub fn teller_drawer<B>(
    state: web::Data<AppState>,
) -> impl Fn(
    ServiceRequest,
    Next<B>,
) -> Pin<Box<dyn Future<Output = Result<ServiceResponse<B>, Error>> + 'static>>
where
    B: MessageBody + 'static,
{
    move |req, next| {
        let state = state.clone();

        Box::pin(async move {
            let session = req
                .extensions()
                .get::<SessionId>()
                .map(|r| r.0.clone())
                .ok_or(ApiError::Unauthorized)?;

            let mut conn = state.cache.get_ref().clone();

            let cache_key = format!("staff:{}", session);

            let cached = conn.get::<_, String>(&cache_key).await.map_err(|e| {
                tracing::error!(error = ?e, "Failed to cached staff");
                ApiError::InternalServerError
            })?;

            let staff: StaffResponseModel =
                serde_json::from_str(&cached).map_err(|_| ApiError::InternalServerError)?;

            let cash_drawer = tellers::services::get_teller_open_drawer(staff.id, &state)
                .await
                .map_err(|e| {
                    tracing::error!(error = ?e, "Failed to get teller drawer");
                    ApiError::InternalServerError
                })?;

            req.extensions_mut().insert(cash_drawer);

            let res = next.call(req).await?;

            Ok(res)
        })
    }
}
