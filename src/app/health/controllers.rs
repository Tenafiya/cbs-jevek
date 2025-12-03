use actix_web::{HttpRequest, HttpResponse, web};

use crate::{
    AppState,
    app::health::{self, services::MigratorParams},
    utils::errors::{ApiCode, ApiError, ApiResponse},
};

pub async fn engine_check() -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse::success(
        ApiCode::OperationSuccess,
        "Health Check Success",
        {},
        None,
    ))
}

pub async fn run_migrations(
    _req: HttpRequest,
    params: web::Path<MigratorParams>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let path = params.into_inner();

    match health::services::activate_migrations(&state, path.direct, path.steps).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            ApiCode::OperationSuccess,
            "Health Check Success",
            {},
            None,
        ))),
        Err(_) => Err(ApiError::InternalServerError),
    }
}
