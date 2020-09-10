mod model;
mod router;
mod logger;

use log::info;
use actix_web::{App, HttpServer};

use model::sql;

#[actix_web::main]
async fn main() {
    logger::init();
    info!("run ...");
    HttpServer::new(|| App::new()
        .service(router::index)
    )
        .bind("127.0.0.1:8083").expect("bind failed")
        .run()
        .await.expect("run failed");
    sql::get_pool().disconnect().await.expect("disconnect mysql pool");
}