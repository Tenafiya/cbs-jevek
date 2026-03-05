use bcrypt::DEFAULT_COST;
use dotenvy::dotenv;
use sha2::{Digest, Sha512};

fn preprocess_password(password: &str, salt: &uuid::Uuid) -> String {
    dotenv().ok();
    let pepper = std::env::var("PEPPER").expect("PEPPER not set");

    let combined = format!("{}{}{}", password, salt.to_string(), pepper);

    let md5_hashed = format!(
        "{:?}{}{}",
        md5::compute(combined),
        password,
        salt.to_string()
    );

    let final_hash = Sha512::new().chain_update(md5_hashed).finalize();

    hex::encode(final_hash)
}

pub async fn encrypt_password(password: &str, salt: &uuid::Uuid) -> String {
    let prehashed = preprocess_password(password, salt);
    tokio::task::spawn_blocking(move || {
        bcrypt::hash(prehashed, DEFAULT_COST)
            .unwrap_or("837287777GYGYGY@&!8hdygvg%TTDYVV".to_string())
    })
    .await
    .expect("Spawn Failed")
}

pub async fn validate_password(password: &str, salt: &uuid::Uuid, hash: &str) -> bool {
    let prehashed = preprocess_password(password, salt);
    let to_owned_hash = hash.to_owned();
    tokio::task::spawn_blocking(move || bcrypt::verify(prehashed, &to_owned_hash).unwrap_or(false))
        .await
        .expect("Spawn Failed")
}
