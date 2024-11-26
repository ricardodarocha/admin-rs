use actix_web::web::Path;
use actix_web::{get, web, HttpResponse, Responder};
// use crate::models::QueryFiltro;
use crate::app::AppState;
use crate::repository::api::produtos::sqlite as repo;

#[get("/cardapio/json/{nome}")]
async fn json_cardapio(
    
        data: web::Data<AppState>,
        path: Path<String>,

    ) -> impl Responder {
        
    let nome = path.into_inner();
    let pool = &data.database;
    
    let cardapio = repo::abrir_cardapio(pool, &nome).await;

    if let Ok(cardapio) = cardapio {

        HttpResponse::Ok()
            .content_type("application/json")
            .json(cardapio)
    } 
    else {     
        
    HttpResponse::NotFound()
        .finish()

    }
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(json_cardapio);
}