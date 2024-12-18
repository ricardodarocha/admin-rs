use actix_web::web;
use actix_web::middleware::from_fn;

use crate::infra::sessao_usuario::check_api_auth;
use crate::application::controller;
pub mod kpis;
pub mod produto;
pub mod cardapio;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .wrap(from_fn(check_api_auth))
            .configure(kpis::routes)
            .configure(controller::routes)
            .configure(cardapio::routes)
    );
}
