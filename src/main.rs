mod router;
mod logger;

use log::info;
use actix_web::{App, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::init();
    info!("run ...");
    HttpServer::new(|| App::new()
        .service(router::index)
    )
        .bind("127.0.0.1:8083")?
        .run()
        .await
}