use actix_web::web::Path;
use actix_web::{get, web, HttpResponse, Responder};
// use minijinja::context;

use crate::app::AppState;
use crate::models::QueryFiltroPedido;
use crate::services::pedido as service;

#[get("/pedido/json/{id}")]
async fn json_pedido(
        data: web::Data<AppState>,
        path: Path<i64>,

    ) -> impl Responder {
        
    let id = path.into_inner();
    let pool = &data.database;
    
    let pedido = service::abrir_pedido(pool, id).await;

    if let Some(pedido) = pedido {

        HttpResponse::Ok()
            .content_type("application/json")
            .json(pedido)
    } 
    else {     
        
    HttpResponse::NotFound()
        .finish()

    }
    
}
/// exemplo http://localhost:8080/pedidos/json?cliente=00008756486
#[get("/pedido/json")]
async fn json_all_pedido(
        data: web::Data<AppState>,
        // path: Path<String>,
        query: web::Query<QueryFiltroPedido>
        // session: Session

    ) -> impl Responder {
        
    // let cliente = path.into_inner();
    let pool = &data.database;
    let web::Query(query) = query;
    let cliente = &query.cliente;
    
    let pedido = service::abrir_lista_pedidos(pool, cliente, &query).await;

    HttpResponse::Ok()
            .content_type("application/json")
            .json(pedido)
    } 
    
