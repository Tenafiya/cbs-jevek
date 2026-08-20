use actix_web::{middleware::from_fn, web};

use crate::{
    AppState,
    app::tellers::controllers,
    middlewares::{account, jwt::jwt_auth},
};

pub fn init(cfg: &mut web::ServiceConfig, state: web::Data<AppState>) {
    cfg.service(
        web::scope("/v1/tellers")
            .route(
                "/add",
                web::post()
                    .to(controllers::create_teller)
                    .wrap(from_fn(account::staff::verify(state.clone())))
                    .wrap(from_fn(jwt_auth)),
            )
            .route(
                "/reconciliation/add",
                web::post()
                    .to(controllers::create_daily_recon)
                    .wrap(from_fn(account::staff::verify(state.clone())))
                    .wrap(from_fn(jwt_auth)),
            )
            .route(
                "/drawers/start",
                web::post()
                    .to(controllers::start_drawer_session)
                    .wrap(from_fn(account::staff::verify(state.clone())))
                    .wrap(from_fn(jwt_auth)),
            )
            .route(
                "/{id}",
                web::get()
                    .to(controllers::fetch_teller_details)
                    .wrap(from_fn(account::staff::verify(state.clone())))
                    .wrap(from_fn(jwt_auth)),
            )
            .route(
                "/all",
                web::get()
                    .to(controllers::fetch_teller_list)
                    .wrap(from_fn(account::staff::verify(state.clone())))
                    .wrap(from_fn(jwt_auth)),
            ),
    );
}
