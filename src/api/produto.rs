use actix_web::web::Path;
use actix_web::{get, web, HttpResponse, Responder};
use crate::models::QueryFiltro;
use crate::app::AppState;
use crate::services::produto as service;
use crate::repository::produto as repo;

#[get("/produto/json/{id}")]
async fn json_produto(
        data: web::Data<AppState>,
        path: Path<String>,

    ) -> impl Responder {
        
    let id = path.into_inner();
    let pool = &data.database;
    
    let produto = service::abrir_produto(pool, id).await;

    if let Some(produto) = produto {

        HttpResponse::Ok()
            .content_type("application/json")
            .json(produto)
    } 
    else {     
        
    HttpResponse::NotFound()
        .finish()

    }
    
}

#[get("/produtos/json")]
async fn json_all_produto(
        data: web::Data<AppState>,
        // path: Path<String>,
        query: web::Query<QueryFiltro>

    ) -> impl Responder {
        
    // let id = path.into_inner();
    let pool = &data.database;
    let web::Query(query) = query;
    
    let produto = repo::abrir_lista_produtos(pool, &query).await;

    if let Ok(produto) = produto {

        HttpResponse::Ok()
            .content_type("application/json")
            .json(produto)
    } 
    else {     
        
    HttpResponse::NotFound()
        .finish()

    }
    
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(json_produto)
        .service(json_all_produto);
}