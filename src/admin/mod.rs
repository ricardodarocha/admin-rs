pub mod cardapio;
mod dashboard;
mod products;
mod customers;

use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .configure(dashboard::routes)
            .configure(products::routes)
            .configure(customers::routes)
            .configure(cardapio::routes)
    );
}