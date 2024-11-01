mod dashboard;

use actix_web::web;
use actix_web::web::{ServiceConfig};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .configure(dashboard::routes)
    );
}