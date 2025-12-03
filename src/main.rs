use cbs_jevek::start_server;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    start_server().await
}
