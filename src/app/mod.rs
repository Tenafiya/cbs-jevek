use actix_web::web::{self, ServiceConfig};

use crate::AppState;

pub mod health;

pub fn app_routes(_state: web::Data<AppState>) -> impl FnOnce(&mut ServiceConfig) + Clone {
    move |cfg: &mut web::ServiceConfig| {
        cfg.configure(health::routes::init);
    }
}
