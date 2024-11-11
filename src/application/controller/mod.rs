pub mod pedido;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/pedido")
            .configure(pedido::consulta::routes)
            .configure(pedido::escrita::routes) 
    );
}