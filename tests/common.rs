use actix_web::{body::to_bytes, web};
use cbs_jevek::setup::init_system::load_config;
use cbs_jevek::{AppState, fileskit::config::StorageService, nats::config::StreamManager};
use redis::aio::ConnectionManager;
use sea_orm::{Database, DatabaseConnection};
use serde_json::Value;
use testcontainers_modules::{
    postgres::Postgres,
    testcontainers::{GenericImage, ImageExt, core::ContainerPort, runners::AsyncRunner},
};

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

async fn start_dragonfly() -> Option<String> {
    let image = GenericImage::new("docker.dragonflydb.io/dragonflydb/dragonfly", "latest")
        .with_exposed_port(ContainerPort::Tcp(6379));

    let container = match image.start().await {
        Ok(container) => container,
        Err(_) => {
            eprintln!("Warning: Could not start Dragonfly container. Make sure Docker is running.");
            return None;
        }
    };

    let port = container
        .get_host_port_ipv4(6379)
        .await
        .expect("no port mapped");

    Some(format!("redis://localhost:{port}"))
}

async fn start_natsjs() -> Option<String> {
    let image = GenericImage::new("nats", "latest")
        .with_exposed_port(4222.into())
        .with_cmd(vec!["-js"]);

    let container = match image.start().await {
        Ok(container) => container,
        Err(_) => {
            eprintln!("Warning: Could not start NATS container. Make sure Docker is running.");
            return None;
        }
    };

    let port = container
        .get_host_port_ipv4(4222)
        .await
        .expect("no port mapped");

    Some(format!("nats://localhost:{port}"))
}

pub async fn build_state(db_url: &str, cache_url: &str) -> web::Data<AppState> {
    let db: DatabaseConnection = Database::connect(db_url).await.unwrap();
    let settings = load_config().expect("config");
    let storage = StorageService::new().await;
    let client = redis::Client::open(cache_url).expect("No redis");
    let cache = ConnectionManager::new(client)
        .await
        .expect("Redis connection failed");
    let stream_manager = setup_test_nats().await.expect("Failed to setup NATS");

    let state = AppState {
        pgdb: web::Data::new(db),
        config: settings,
        storage: web::Data::new(storage),
        cache: web::Data::new(cache),
        streamer: web::Data::new(stream_manager),
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

pub async fn setup_test_dragonfly() -> Option<String> {
    start_dragonfly().await
}

pub async fn setup_test_nats() -> Result<StreamManager, Box<dyn std::error::Error>> {
    let nats_url = start_natsjs().await.expect("NATS failed to start");

    unsafe {
        std::env::set_var("NATS_JETSTREAM", &nats_url);
    }

    let mut stream_manager = StreamManager::new().await?;

    stream_manager
        .register_stream("AMLS", vec!["amls.>".into()], "aml-processor".to_string())
        .await?;

    Ok(stream_manager)
}
