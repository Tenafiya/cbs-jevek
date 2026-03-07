use actix_web::{web, middleware::from_fn};

use crate::{AppState, app::customers::controllers, middlewares::jwt::jwt_auth};

pub fn init(cfg: &mut web::ServiceConfig, _state: web::Data<AppState>) {
    cfg.service(
        web::scope("/v1/customers")
            .route("/add", web::post().to(controllers::add_customer).wrap(from_fn(jwt_auth)))
            .route(
                "/{id}/add-address",
                web::put().to(controllers::save_address)
                .wrap(from_fn(jwt_auth)),
            )
            .route(
                "/{id}/add-occupation",
                web::put().to(controllers::save_occupation)
                .wrap(from_fn(jwt_auth)),
            )
            .route("/{id}/add-kin", web::put().to(controllers::save_kin).wrap(from_fn(jwt_auth)))
            .route(
                "/email/{id}/verify",
                web::get().to(controllers::email_verification)
                .wrap(from_fn(jwt_auth)),
            )
            .route(
                "/phone/{id}/verify",
                web::get().to(controllers::sms_verification).wrap(from_fn(jwt_auth)),
            )
            .route(
                "/{id}/details",
                web::get().to(controllers::customer_details).wrap(from_fn(jwt_auth)),
            )
            .route("/all", web::get().to(controllers::all_customers).wrap(from_fn(jwt_auth)))
            .route(
                "/{id}/sanctions",
                web::get().to(controllers::update_sanctions).wrap(from_fn(jwt_auth)),
            )
            .route("/{id}/verify", web::get().to(controllers::verify_customer).wrap(from_fn(jwt_auth)))
            .route("/{id}", web::delete().to(controllers::delete_customer).wrap(from_fn(jwt_auth))),
    );
}
