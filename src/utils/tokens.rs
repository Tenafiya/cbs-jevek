use actix_web::{HttpRequest, web};
use chrono::{Duration, Utc};
use dotenvy::dotenv;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::utils::errors::ApiError;
use crate::utils::gen_snow_ids::gen_string;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub iat: usize,
    pub exp: usize,
    pub jid: String,
    pub sub: String,
    pub org: String,
}

pub fn parse_token(token: &str) -> Result<Claims, ApiError> {
    let jwt_key = std::env::var("SECRET_KEY").map_err(|_| ApiError::InternalServerError)?;

    let token_res = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_key.as_ref()),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(|_| ApiError::Unauthorized)?;

    Ok(token_res.claims)
}

pub async fn create_jwt(
    session: &String,
    token_type: &str,
    state: &web::Data<AppState>,
) -> (String, usize) {
    dotenv().unwrap();

    let expire = match token_type {
        "NORMAL" => state.config.get::<i64>("jwt.access_expire").unwrap_or(3600),
        "REFRESH" => state
            .config
            .get::<i64>("jwt.refresh_expire")
            .unwrap_or(7200),
        _ => 0,
    };

    let jwt_key = std::env::var("SECRET_KEY").unwrap();

    let created = Utc::now();
    let expiry = created + Duration::seconds(expire);
    let jid = uuid::Uuid::new_v4();

    let claim = Claims {
        iat: created.timestamp() as usize,
        exp: expiry.timestamp() as usize,
        jid: jid.to_string(),
        sub: session.clone(),
        org: gen_string(32).await,
    };

    let token = encode(
        &Header::new(Algorithm::HS512),
        &claim,
        &EncodingKey::from_secret(jwt_key.as_ref()),
    )
    .unwrap_or_default();

    (token, claim.exp)
}

pub async fn verify_jwt(req: &HttpRequest) -> Result<Claims, ApiError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or(ApiError::Unauthorized)?
        .to_str()
        .map_err(|_| ApiError::Unauthorized)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(ApiError::Unauthorized);
    }

    let token = &auth_header[7..];

    parse_token(token)
}
