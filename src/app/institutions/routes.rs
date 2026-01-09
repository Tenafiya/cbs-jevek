use actix_web::web;

use crate::{AppState, app::institutions::controllers};

pub fn init(cfg: &mut web::ServiceConfig, _state: web::Data<AppState>) {
    cfg.service(
        web::scope("/v1/institutions")
            .route("/add", web::post().to(controllers::add_institution))
            .route("/get", web::get().to(controllers::get_institutions))
            .route("/{id}/get", web::get().to(controllers::get_institution))
            .route("/update", web::put().to(controllers::update_institution)),
    );
}
