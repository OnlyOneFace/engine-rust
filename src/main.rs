mod logger;

pub mod database;
pub mod model;
pub mod router;
pub mod controller;

use log::info;

use actix_web::{App, HttpServer, middleware};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::init();
    info!("run ...");
    database::init_db_pool();
    HttpServer::new(move || App::new()
        .wrap(middleware::Logger::default())
        .configure(router::router))
        .bind("127.0.0.1:8083").expect("bind addr failed")
        .run()
        .await
}
