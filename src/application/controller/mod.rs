pub mod pedido;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/pedidos")
            .configure(pedido::routes) 
    );
}

//todo! remove application layer. Change to api|site|admin layer name