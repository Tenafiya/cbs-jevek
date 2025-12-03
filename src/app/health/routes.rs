use actix_web::web;

use crate::app::health;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/health")
            .route("/engine", web::get().to(health::controllers::engine_check))
            .route(
                "/migrations/{direct}/{steps}",
                web::get().to(health::controllers::run_migrations),
            ),
    );
}
