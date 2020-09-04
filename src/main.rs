mod logger;

use log::info;
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{id}/{name}")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::init();
    info!("run ...");
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8083")?
        .run()
        .await
}