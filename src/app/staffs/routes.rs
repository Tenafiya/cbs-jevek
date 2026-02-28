use actix_web::web;

use crate::{AppState, app::staffs::controllers};

pub fn init(cfg: &mut web::ServiceConfig, _state: web::Data<AppState>) {
    cfg.service(
        web::scope("/v1/staff")
            .route("/add", web::post().to(controllers::add_staff))
            .route("/status", web::put().to(controllers::update_status))
            .route("/{id}/update", web::put().to(controllers::staff_update))
            .route("/{id}", web::get().to(controllers::staff_details))
            .route("/{id}/all", web::get().to(controllers::get_staffs)),
    );
}
