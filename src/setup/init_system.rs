use std::io::Write;

use actix_cors::Cors;
use actix_web::http;
use config::{Config, File, FileFormat};

use crate::middlewares::cors::validate_origin;

pub fn load_config() -> Result<Config, config::ConfigError> {
    Config::builder()
        .add_source(File::new("app.config", FileFormat::Toml))
        .build()
}

pub fn setup_logging() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format(|buf, record| writeln!(buf, "Convyn [{}] - {}", record.level(), record.args()))
        .init();
}

pub fn configure_cors() -> Cors {
    Cors::default()
        .allowed_origin_fn(|origin, _req_head| validate_origin(origin))
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allowed_headers(vec![
            http::header::CONTENT_TYPE,
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
        ])
        .supports_credentials()
        .max_age(3600)
}

pub async fn wait_for_shutdown(server_handle: actix_web::dev::ServerHandle) {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for shutdown signal");

    log::info!("🚨 Shutdown signal received, initiating graceful shutdown...");
    server_handle.stop(true).await;
    log::info!("👋 Server shutdown complete");
}
