use redis::aio::ConnectionManager;

pub async fn connector() -> ConnectionManager {
    let dragonfly_url = std::env::var("DRAGONFLY_URL").expect("Cannot find DRAGONFLY URL");

    let client = redis::Client::open(dragonfly_url).expect("Failed to create Dragonfly client");

    let manager = ConnectionManager::new(client)
        .await
        .expect("Failed to connect to Dragonfly");

    manager
}
