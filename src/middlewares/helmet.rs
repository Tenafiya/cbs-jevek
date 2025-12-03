use actix_web::{
    Error,
    dev::{ServiceRequest, ServiceResponse},
    http,
    middleware::Next,
};

pub async fn security_headers(
    req: ServiceRequest,
    next: Next<impl actix_web::body::MessageBody>,
) -> Result<ServiceResponse<impl actix_web::body::MessageBody>, Error> {
    let mut res = next.call(req).await?;

    let headers = res.headers_mut();
    headers.insert(
        http::header::STRICT_TRANSPORT_SECURITY,
        "max-age=31536000; includeSubDomains".parse().unwrap(),
    );
    headers.insert(http::header::X_FRAME_OPTIONS, "DENY".parse().unwrap());
    headers.insert(
        http::header::X_CONTENT_TYPE_OPTIONS,
        "nosniff".parse().unwrap(),
    );
    headers.insert(
        http::header::X_XSS_PROTECTION,
        "1; mode=block".parse().unwrap(),
    );
    headers.insert(
        http::header::CONTENT_SECURITY_POLICY,
        "default-src 'self'".parse().unwrap(),
    );
    headers.insert(
        http::header::REFERRER_POLICY,
        "no-referrer".parse().unwrap(),
    );

    Ok(res)
}
