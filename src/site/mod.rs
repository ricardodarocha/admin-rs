mod pages;

use actix_web::web::{ServiceConfig};

pub fn routes(cfg: &mut ServiceConfig) {
    pages::routes(cfg);
}
