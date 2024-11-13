use actix_web::web;
use actix_web::middleware::from_fn;

use crate::infra::sessao_usuario::check_api_auth;

pub mod kpis;
pub mod produto;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .wrap(from_fn(check_api_auth))
            .configure(kpis::routes)
            .configure(produto::routes),
    );
}
