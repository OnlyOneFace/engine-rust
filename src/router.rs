use actix_web::web::{ServiceConfig, scope};

use crate::controller::index;

pub fn router(cfg: &mut ServiceConfig) {
    cfg
        .service(
            scope("/api/v1")
                .service(index::index)
        );
}