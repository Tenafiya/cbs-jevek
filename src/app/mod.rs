use actix_web::web::{self, ServiceConfig};

use crate::AppState;

pub mod branches;
pub mod countries;
pub mod health;
pub mod institutions;

pub fn app_routes(state: web::Data<AppState>) -> impl FnOnce(&mut ServiceConfig) + Clone {
    move |cfg: &mut web::ServiceConfig| {
        cfg.configure(health::routes::init);
        cfg.configure(|c| countries::routes::init(c, state.clone()));
        cfg.configure(|c| institutions::routes::init(c, state.clone()));
        cfg.configure(|c| branches::routes::init(c, state.clone()));
    }
}
