use actix_web::web;

pub mod consultas;
pub mod processos;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(consultas::consulta_pedido_por_id)
        .service(consultas::consulta_pedido_por_cliente)
        .service(processos::atualizar_pedido_from_json)
        .service(processos::inserir_pedido_from_form)
        .service(processos::inserir_pedido_from_json)
        .service(processos::atualizar_pedido_from_form)
        ;
}