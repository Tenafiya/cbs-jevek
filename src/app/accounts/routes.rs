use actix_web::{middleware::from_fn, web};

use crate::{
    AppState,
    app::accounts::controllers::{
        add_account_links, add_customer_account, get_all_cus_accounts, get_cus_account,
    },
    middlewares::{account, jwt::jwt_auth},
};

pub fn init(cfg: &mut web::ServiceConfig, state: web::Data<AppState>) {
    cfg.service(
        web::scope("/v1/accounts")
            .route(
                "/add",
                web::post()
                    .to(add_customer_account)
                    .wrap(from_fn(account::staff::verify(state.clone())))
                    .wrap(from_fn(jwt_auth)),
            )
            .route(
                "/all",
                web::get()
                    .to(get_all_cus_accounts)
                    .wrap(from_fn(account::staff::verify(state.clone())))
                    .wrap(from_fn(jwt_auth)),
            )
            .route(
                "/{id}/customer",
                web::get()
                    .to(get_cus_account)
                    .wrap(from_fn(account::staff::verify(state.clone())))
                    .wrap(from_fn(jwt_auth)),
            )
            .route(
                "/link",
                web::post()
                    .to(add_account_links)
                    .wrap(from_fn(account::staff::verify(state.clone())))
                    .wrap(from_fn(jwt_auth)),
            ),
    );
}
