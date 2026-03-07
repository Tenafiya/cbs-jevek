use actix_web::{middleware::from_fn, web};

use crate::{AppState, app::countries, middlewares::jwt::jwt_auth};

pub fn init(cfg: &mut web::ServiceConfig, _state: web::Data<AppState>) {
    cfg.service(
        web::scope("/v1/countries")
            .route("/add", web::post().to(countries::controllers::add_country))
            .route("/get", web::get().to(countries::controllers::get_countries))
            .route(
                "/{id}/get",
                web::get().to(countries::controllers::get_country),
            )
            .route(
                "/{id}/operate/{operation}",
                web::get().to(countries::controllers::operate_country).wrap(from_fn(jwt_auth)),
            ),
    );
}
