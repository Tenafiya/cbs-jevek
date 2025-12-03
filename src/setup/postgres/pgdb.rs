use config::Config;
use dotenvy::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

pub async fn connector(settings: &Config) -> DatabaseConnection {
    dotenv().unwrap();

    let idle_timeout = settings.get::<u64>("pg.idle_timeout").unwrap();
    let connect_timeout = settings.get::<u64>("pg.connect_timeout").unwrap();
    let acquire_timeout = settings.get::<u64>("pg.acquire_timeout").unwrap();
    let max = settings.get::<u32>("pg.max").unwrap();
    let min = settings.get::<u32>("pg.min").unwrap();

    let db_env_check = settings.get::<String>("app.environment").unwrap();

    let (database_url, sqlx_logger) = match db_env_check.as_str() {
        "TEST" => (
            std::env::var("DATABASE_URL").expect("Cannot Find DATABASE URL"),
            false,
        ),
        _ => (
            std::env::var("DATABASE_URL").expect("Cannot Find DATABASE URL"),
            false,
        ),
    };

    let mut options = ConnectOptions::new(database_url);

    options
        .max_connections(max)
        .min_connections(min)
        .connect_timeout(Duration::new(connect_timeout, 0))
        .acquire_timeout(Duration::new(acquire_timeout, 0))
        .idle_timeout(Duration::new(idle_timeout, 0))
        .max_lifetime(Duration::from_secs(3600))
        .sqlx_logging(sqlx_logger)
        .sqlx_logging_level(log::LevelFilter::Warn);

    let pool = Database::connect(options).await.unwrap();

    pool
}
