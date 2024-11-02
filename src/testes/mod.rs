pub mod testes;

use actix_web::web::ServiceConfig;

pub fn routes(cfg: &mut ServiceConfig) {
    testes::routes(cfg);
}
