use std::io;
use std::panic;

use actix_web::{
    App, HttpServer,
    middleware::{Logger, NormalizePath, from_fn},
    web::Data,
};
use config::Config;

pub mod app;
mod middlewares;
pub mod setup;
pub mod utils;

use crate::setup::init_system;
use crate::setup::postgres::pgdb;
use crate::{app::app_routes, middlewares::helmet::security_headers};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub pgdb: Data<sea_orm::DatabaseConnection>,
}

async fn setup_app_state() -> Result<Data<AppState>, Box<dyn std::error::Error>> {
    let settings = init_system::load_config()?;
    let pg_conn = Data::new(pgdb::connector(&settings).await);

    Ok(Data::new(AppState {
        config: settings,
        pgdb: pg_conn,
    }))
}

pub async fn start_server() -> Result<(), std::io::Error> {
    init_system::setup_logging();

    let state = setup_app_state()
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let host = state
        .config
        .get_string("app.host")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let port = state
        .config
        .get_string("app.port")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let addr = format!("{host}:{port}");

    // let job_state = state.clone();
    // tokio::spawn(async move {
    //     launchjobs(job_state).await;
    // });

    log::info!("🚀 Server starting at {}", addr);

    let server = HttpServer::new(move || {
        let cors = init_system::configure_cors();

        App::new()
            .app_data(state.clone())
            .wrap(from_fn(security_headers))
            .wrap(Logger::new(
                "%t %a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T \"%{Content-Type}o\"",
            ))
            .wrap(NormalizePath::trim())
            .wrap(cors)
            .configure(app_routes(state.clone()))
    })
    .bind(&addr)?
    .run();

    let server_handle = server.handle();

    tokio::select! {
        result = server => {
            if let Err(e) = result {
                log::error!("Server error: {}", e);
            }
        }
        _ = init_system::wait_for_shutdown(server_handle) => {
            log::info!("Graceful shutdown completed");
        }
    }

    Ok(())
}
