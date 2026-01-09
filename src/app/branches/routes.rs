use actix_web::web;

use crate::{AppState, app::branches::controllers};

pub fn init(cfg: &mut web::ServiceConfig, _state: web::Data<AppState>) {
    cfg.service(
        web::scope("/v1/branches")
            .route("/add", web::post().to(controllers::add_branch))
            .route("/get/{id}", web::get().to(controllers::get_branches))
            .route("/{id}/get", web::get().to(controllers::get_branch_details)),
    );
}
