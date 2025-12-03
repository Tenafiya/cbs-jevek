use actix_web::http::{Uri, header::HeaderValue};
use std::env;

const ALLOWED_PRODUCTION_HOSTS: &[&str] = &[""];

pub fn validate_origin(origin: &HeaderValue) -> bool {
    let origin_str = match origin.to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };

    if origin_str == "null" || origin_str.starts_with("chrome-extension://") {
        return true;
    }

    if origin_str.starts_with("jevek-cbs://")
        || origin_str.starts_with("exp://")
        || origin_str.starts_with("jevek-cbs-expo://")
    {
        return true;
    }

    let uri = match origin_str.parse::<Uri>() {
        Ok(uri) => uri,
        Err(_) => return false,
    };

    let host = match uri.host() {
        Some(h) => h,
        None => return false,
    };

    if is_development_mode() {
        is_development_host(host, true)
    } else {
        ALLOWED_PRODUCTION_HOSTS.contains(&host)
    }
}

// Dev Only
fn is_development_host(host: &str, allow_any_origin: bool) -> bool {
    if allow_any_origin {
        return true;
    }

    if matches!(host, "localhost" | "127.0.0.1" | "0.0.0.0" | "::1") {
        return true;
    }

    if let Ok(addr) = host.parse::<std::net::IpAddr>() {
        match addr {
            std::net::IpAddr::V4(ipv4) => ipv4.is_private(),
            std::net::IpAddr::V6(ipv6) => ipv6.is_loopback() || ipv6.is_unicast_link_local(),
        }
    } else {
        false
    }
}

// Environment check
#[inline]
fn is_development_mode() -> bool {
    env::var("APP_ENV")
        .map(|env| env == "TEST")
        .unwrap_or(false)
}
