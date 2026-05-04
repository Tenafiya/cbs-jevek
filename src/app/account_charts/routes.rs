use actix_web::{middleware::from_fn, web};

use crate::{
    AppState,
    app::account_charts::controllers,
    middlewares::{account, jwt::jwt_auth},
};

pub fn init(cfg: &mut web::ServiceConfig, state: web::Data<AppState>) {
    cfg.service(
        web::scope("/v1/account-charts")
            .route(
                "/add",
                web::post()
                    .to(controllers::add_acc_chart)
                    .wrap(from_fn(account::staff::verify(state.clone())))
                    .wrap(from_fn(jwt_auth)),
            )
            .route(
                "/categories/add",
                web::post()
                    .to(controllers::add_acc_cat)
                    .wrap(from_fn(account::staff::verify(state.clone())))
                    .wrap(from_fn(jwt_auth)),
            )
            .route(
                "/get",
                web::get()
                    .to(controllers::fetch_charts)
                    .wrap(from_fn(account::staff::verify(state.clone())))
                    .wrap(from_fn(jwt_auth)),
            )
            .route(
                "/categories/get",
                web::get()
                    .to(controllers::fetch_categories)
                    .wrap(from_fn(account::staff::verify(state.clone())))
                    .wrap(from_fn(jwt_auth)),
            )
            .route(
                "/types/add",
                web::post()
                    .to(controllers::add_acc_types)
                    .wrap(from_fn(account::staff::verify(state.clone())))
                    .wrap(from_fn(jwt_auth)),
            )
            .route(
                "/types/get",
                web::get()
                    .to(controllers::fetch_account_types)
                    .wrap(from_fn(account::staff::verify(state.clone())))
                    .wrap(from_fn(jwt_auth)),
            ),
    );
}
