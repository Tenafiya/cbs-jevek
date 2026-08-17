use actix_web::web::{self, ServiceConfig};

use crate::AppState;

pub mod account_charts;
pub mod accounts;
pub mod amls;
pub mod branches;
pub mod countries;
pub mod customers;
pub mod health;
pub mod institutions;
pub mod media;
pub mod staffs;
pub mod tellers;
pub mod transactions;

pub fn app_routes(state: web::Data<AppState>) -> impl FnOnce(&mut ServiceConfig) + Clone {
    move |cfg: &mut web::ServiceConfig| {
        cfg.configure(health::routes::init);
        cfg.configure(|c| countries::routes::init(c, state.clone()));
        cfg.configure(|c| institutions::routes::init(c, state.clone()));
        cfg.configure(|c| branches::routes::init(c, state.clone()));
        cfg.configure(|c| customers::routes::init(c, state.clone()));
        cfg.configure(|c| staffs::routes::init(c, state.clone()));
        cfg.configure(|c| account_charts::routes::init(c, state.clone()));
        cfg.configure(|c| accounts::routes::init(c, state.clone()));
        cfg.configure(|c| media::routes::init(c, state.clone()));
        cfg.configure(|c| amls::routes::init(c, state.clone()));
        cfg.configure(|c| tellers::routes::init(c, state.clone()));
        cfg.configure(|c| transactions::routes::init(c, state.clone()));
    }
}
