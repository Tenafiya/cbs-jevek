use actix_web::{body::to_bytes, web};
use cbs_jevek::AppState;
use cbs_jevek::setup::init_system::load_config;
use sea_orm::{Database, DatabaseConnection};
use serde_json::Value;
use testcontainers_modules::{postgres::Postgres, testcontainers::runners::AsyncRunner};

async fn start_postgres() -> Option<String> {
    let container = match Postgres::default()
        .with_db_name("test")
        .with_user("test")
        .with_password("test")
        .start()
        .await
    {
        Ok(container) => container,
        Err(_) => {
            eprintln!(
                "Warning: Could not start PostgreSQL container. Make sure Docker is running."
            );
            return None;
        }
    };

    let port = container
        .get_host_port_ipv4(5432)
        .await
        .expect("no port mapped");

    Some(format!("postgres://test:test@localhost:{port}/test"))
}

pub async fn build_state(db_url: &str) -> web::Data<AppState> {
    let db: DatabaseConnection = Database::connect(db_url).await.unwrap();
    let settings = load_config().expect("config");

    let state = AppState {
        pgdb: web::Data::new(db),
        config: settings,
    };

    web::Data::new(state)
}

pub async fn body_json(res: actix_web::dev::ServiceResponse) -> Value {
    let body = res.into_body();
    let bytes = to_bytes(body).await.unwrap();
    serde_json::from_slice(&bytes).expect("invalid json")
}

pub async fn setup_test_database() -> Option<String> {
    start_postgres().await
}
