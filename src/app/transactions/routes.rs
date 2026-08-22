use actix_web::{middleware::from_fn, web};

use crate::{
    AppState,
    app::transactions::controllers,
    middlewares::{account, jwt::jwt_auth},
};

pub fn init(cfg: &mut web::ServiceConfig, state: web::Data<AppState>) {
    cfg.service(
        web::scope("/v1/transactions")
            .route(
                "/channels/add",
                web::post()
                    .to(controllers::create_trans_channel)
                    .wrap(from_fn(account::staff::verify(state.clone())))
                    .wrap(from_fn(jwt_auth)),
            )
            .route(
                "/limits/add",
                web::post()
                    .to(controllers::create_trans_limit)
                    .wrap(from_fn(account::staff::verify(state.clone())))
                    .wrap(from_fn(jwt_auth)),
            )
            .route(
                "/limits",
                web::get()
                    .to(controllers::get_trans_limits)
                    .wrap(from_fn(account::staff::verify(state.clone())))
                    .wrap(from_fn(jwt_auth)),
            )
            .route(
                "/channels",
                web::get()
                    .to(controllers::get_trans_channels)
                    .wrap(from_fn(account::staff::verify(state.clone())))
                    .wrap(from_fn(jwt_auth)),
            ),
    );
}
