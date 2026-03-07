use actix_web::{web, middleware::from_fn};

use crate::{AppState, app::institutions::controllers, middlewares::jwt::jwt_auth};

pub fn init(cfg: &mut web::ServiceConfig, _state: web::Data<AppState>) {
    cfg.service(
        web::scope("/v1/institutions")
            .route("/add", web::post().to(controllers::add_institution))
            .route("/get", web::get().to(controllers::get_institutions).wrap(from_fn(jwt_auth)))
            .route("/{id}/get", web::get().to(controllers::get_institution).wrap(from_fn(jwt_auth)))
            .route("/update", web::put().to(controllers::update_institution).wrap(from_fn(jwt_auth))),
    );
}
