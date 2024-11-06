use actix_session::Session;
use actix_web::web::Path;
use actix_web::{get, post, web, HttpResponse, Responder};
use log::{info, error};
// use minijinja::context;
use crate::repository::pedido as repo;

use crate::app::AppState;
use crate::models::pedido::PostPedido;
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

    #[post("/pedido/json")]
    pub async fn json_post_new_pedido(
        _data: web::Data<AppState>,
        _path: Path<i64>,
        _pedido: web::Json<PostPedido>,
        _session: Session 

    ) -> impl Responder {
        HttpResponse::SeeOther()
                .insert_header(( actix_web::http::header::LOCATION, "/pedido/json/0"))
                .finish()
    }
    
// Pedido API recebe o pedido via json. Diferente de form_post_pedido
#[post("/pedido/json/{num_pedido}")]
pub async fn json_post_pedido(
        data: web::Data<AppState>,
        path: Path<i64>,
        pedido: web::Json<PostPedido>,
        _session: Session

    ) -> impl Responder {

    let pool = &data.database;
    let path_pedido = path.into_inner();
    let id_pedido = if path_pedido == 0 { None } else {Some(path_pedido)};
    let pedido = pedido.into_inner();

    let pedido = repo::inserir_pedido_from_json(pool, &pedido, &id_pedido).await;

    match pedido {
        Ok(value) => {
            info!("Pedido salvo com sucesso {}", value.num);
            HttpResponse::Ok().json(value)
        }
        Err(err) => {
            error!("❌ {}", err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }

}
    
    /// Move pedido para preparando
    /// Exige que o pedido esteja no status novo
    #[post("/pedido/preparar/{id}")]
    pub async fn preparar_pedido(
        data: web::Data<AppState>,
        path: Path<i64>,
        _session: Session 

    ) -> impl Responder {
        
    let pool = &data.database;
    let id_pedido = path.into_inner();
    
    let body = repo::abrir_pedido(pool, id_pedido).await.unwrap();
    if body.status != "novo".to_owned() {
        return HttpResponse::NotModified().json(body)
    } else {
        let _ = repo::preparar_pedido(pool, id_pedido).await;
        HttpResponse::Ok().json(body) 
    }   
    }
    
    /// Move pedido para pronto
    /// Exige que o pedido esteja no status preparando
    #[post("/pedido/finalizar/{id}")]
    pub async fn finalizar_pedido(
        data: web::Data<AppState>,
        path: Path<i64>,
        _session: Session 

    ) -> impl Responder {
        
    let pool = &data.database;
    let id_pedido = path.into_inner();
    
    let body = repo::abrir_pedido(pool, id_pedido).await.unwrap();
    if body.status != "preparando".to_owned() {
        return HttpResponse::NotModified().json(body)
    } else {
        let _ = repo::finalizar_pedido(pool, id_pedido).await;
        HttpResponse::Ok().json(body) 
    }
    
    
        
    }