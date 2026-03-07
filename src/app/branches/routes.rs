use actix_web::{middleware::from_fn, web};

use crate::{AppState, app::branches::controllers, middlewares::jwt::jwt_auth};

pub fn init(cfg: &mut web::ServiceConfig, _state: web::Data<AppState>) {
    cfg.service(
        web::scope("/v1/branches")
            .route("/add", web::post().to(controllers::add_branch).wrap(from_fn(jwt_auth)))
            .route("/get/{id}", web::get().to(controllers::get_branches).wrap(from_fn(jwt_auth)))
            .route("/{id}/get", web::get().to(controllers::get_branch_details).wrap(from_fn(jwt_auth))),
    );
}
