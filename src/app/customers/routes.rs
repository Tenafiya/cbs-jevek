use actix_web::web;

use crate::{AppState, app::customers::controllers};

pub fn init(cfg: &mut web::ServiceConfig, _state: web::Data<AppState>) {
    cfg.service(
        web::scope("/v1/customers")
            .route("/add", web::post().to(controllers::add_customer))
            .route(
                "/{id}/add-address",
                web::put().to(controllers::save_address),
            )
            .route(
                "/{id}/add-occupation",
                web::put().to(controllers::save_occupation),
            )
            .route("/{id}/add-kin", web::put().to(controllers::save_kin))
            .route(
                "/email/{id}/verify",
                web::get().to(controllers::email_verification),
            )
            .route(
                "/phone/{id}/verify",
                web::get().to(controllers::sms_verification),
            )
            .route(
                "/{id}/details",
                web::get().to(controllers::customer_details),
            )
            .route("/all", web::get().to(controllers::all_customers))
            .route(
                "/{id}/sanctions",
                web::get().to(controllers::update_sanctions),
            )
            .route("/{id}/verify", web::get().to(controllers::verify_customer))
            .route("/{id}", web::delete().to(controllers::delete_customer)),
    );
}
