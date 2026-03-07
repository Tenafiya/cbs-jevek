use actix_web::{web, middleware::from_fn};

use crate::{AppState, app::staffs::controllers, middlewares::jwt::jwt_auth};

pub fn init(cfg: &mut web::ServiceConfig, _state: web::Data<AppState>) {
    cfg.service(
        web::scope("/v1/staff")
            .route("/add", web::post().to(controllers::add_staff).wrap(from_fn(jwt_auth)))
            .route("/status", web::put().to(controllers::update_status).wrap(from_fn(jwt_auth)))
            .route("/{id}/update", web::put().to(controllers::staff_update).wrap(from_fn(jwt_auth)))
            .route("/{id}", web::get().to(controllers::staff_details).wrap(from_fn(jwt_auth)))
            .route("/{id}/all", web::get().to(controllers::get_staffs).wrap(from_fn(jwt_auth))),
    );
}
