use actix_web::HttpRequest;

pub async fn extract_header(
    req: &HttpRequest,
    header_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let header = req
        .headers()
        .get(header_name)
        .ok_or_else(|| format!("{} header not found", header_name))?;

    let value = header
        .to_str()
        .map_err(|_| format!("{} header is not valid UTF-8", header_name))?;

    Ok(value.to_string())
}
