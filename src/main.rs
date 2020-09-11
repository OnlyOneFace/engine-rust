mod model;
mod router;
mod logger;

pub mod database;

use log::info;
use actix_web::{App, HttpServer};


#[actix_web::main]
async fn main() {
    logger::init();
    info!("run ...");
    database::init_db_pool();
    HttpServer::new(|| App::new()
        .service(router::index)
    )
        .bind("127.0.0.1:8083").expect("bind failed")
        .run()
        .await.expect("run failed");
}
