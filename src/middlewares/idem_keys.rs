use actix_web::{
    Error, HttpMessage, HttpResponse,
    body::{BoxBody, EitherBody, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web,
};
use redis::AsyncCommands;
use serde_json::json;
use std::pin::Pin;

use entity::sea_orm_active_enums::{IdempotencyChannel, IdempotencyOperation};

use crate::{
    AppState,
    app::{
        idem_keys::{models::InitIdemModel, services},
        staffs::models::StaffResponseModel,
    },
    middlewares::account::SessionId,
    utils::{
        errors::{ApiCode, ApiError, ApiResponse},
        headers::extract_header,
    },
};

pub fn verify_idem_key<B>(
    state: web::Data<AppState>,
    channel: IdempotencyChannel,
    opt: IdempotencyOperation,
) -> impl Fn(
    ServiceRequest,
    Next<B>,
) -> Pin<
    Box<dyn Future<Output = Result<ServiceResponse<EitherBody<B, BoxBody>>, Error>> + 'static>,
>
where
    B: MessageBody + 'static,
{
    move |req, next| {
        let state = state.clone();
        let channel = channel.clone();
        let operation = opt.clone();

        Box::pin(async move {
            let idem_key = extract_header(&req.request(), "x-idempotency-key")
                .await
                .map_err(|e| {
                    tracing::error!(error = ?e, "Failed to extract idempotency key");
                    ApiError::BadRequest(e.to_string())
                })?;

            let request_method = req.method().as_str().to_string();
            let request_path = req.path().to_string();

            let session = req
                .extensions()
                .get::<SessionId>()
                .map(|r| r.0.clone())
                .ok_or(ApiError::Unauthorized)?;

            let mut conn = state.cache.get_ref().clone();

            let cache_key = format!("staff:{}", session);

            let cached = conn.get::<_, String>(&cache_key).await.map_err(|e| {
                tracing::error!(error = ?e, "Failed to get cached staff");
                ApiError::InternalServerError
            })?;

            let staff: StaffResponseModel =
                serde_json::from_str(&cached).map_err(|_| ApiError::InternalServerError)?;

            let idem_init = InitIdemModel {
                institution_id: staff.institution_id,
                idem_key,
                request_method,
                request_path,
                channel,
                operation,
            };

            let idem_response = services::find_idem_key(&idem_init, &state)
                .await
                .map_err(|e| {
                    tracing::error!(error = ?e, "Failed to find idempotency key");
                    ApiError::InternalServerError
                })?;

            // early return here if idempotency key exists
            if let Some(idem_model) = idem_response {
                let http_response = HttpResponse::Ok().json(ApiResponse::success(
                    ApiCode::Conflict,
                    "Successful",
                    json!({
                        "idempotency_key": idem_model.idempotency_key,
                        "response": idem_model.response_body,
                        "error": idem_model.error_message
                    }),
                ));

                let (req, _) = req.into_parts();

                let service_response =
                    ServiceResponse::new(req, http_response.map_into_right_body());

                return Ok(service_response);
            }

            let message = serde_json::to_string(&idem_init).map_err(|e| {
                tracing::error!(error = ?e, "Failed to parse idempotency init body");
                ApiError::InternalServerError
            })?;

            state
                .streamer
                .publish_to_stream("idems.initialize.new", message)
                .await
                .map_err(|e| {
                    tracing::error!(error = ?e, "Failed to publish idempotency init string");
                    ApiError::InternalServerError
                })?;

            let res = next.call(req).await?;

            Ok(res.map_into_left_body())
        })
    }
}
