use cbs_jevek::start_server;
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    start_server().await
}
