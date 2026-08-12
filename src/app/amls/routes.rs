use actix_web::{middleware::from_fn, web};

use crate::{
    AppState,
    app::amls::controllers::{create_aml_action, create_new_case_note, create_new_rule},
    middlewares::{account, jwt::jwt_auth},
};

pub fn init(cfg: &mut web::ServiceConfig, state: web::Data<AppState>) {
    cfg.service(
        web::scope("/v1/amls")
            .route(
                "/rule",
                web::post()
                    .to(create_new_rule)
                    .wrap(from_fn(account::staff::verify(state.clone())))
                    .wrap(from_fn(jwt_auth)),
            )
            .route(
                "/notes",
                web::post()
                    .to(create_new_case_note)
                    .wrap(from_fn(account::staff::verify(state.clone())))
                    .wrap(from_fn(jwt_auth)),
            )
            .route(
                "/actions",
                web::post()
                    .to(create_aml_action)
                    .wrap(from_fn(account::staff::verify(state.clone())))
                    .wrap(from_fn(jwt_auth)),
            ),
    );
}
