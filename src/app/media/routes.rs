use actix_web::{middleware::from_fn, web};

use crate::{
    AppState,
    app::media::controllers::{file_redirect, handle_presign, handle_upload_confirm},
    middlewares::{account, jwt::jwt_auth},
};

pub fn init(cfg: &mut web::ServiceConfig, state: web::Data<AppState>) {
    cfg.service(
        web::scope("/v1/media")
            .route(
                "/presign",
                web::post()
                    .to(handle_presign)
                    .wrap(from_fn(account::staff::verify(state.clone())))
                    .wrap(from_fn(jwt_auth)),
            )
            .route(
                "/confirm",
                web::post()
                    .to(handle_upload_confirm)
                    .wrap(from_fn(jwt_auth)),
            )
            .route("/{slug}", web::get().to(file_redirect)),
    );
}
