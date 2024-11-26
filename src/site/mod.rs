pub mod cardapio;
mod pages;

use actix_web::web;

use actix_web::web::ServiceConfig;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("")
            .configure(pages::routes)
        //  .wrap(from_fn(require_login))
            .configure(cardapio::routes)
    );
}

