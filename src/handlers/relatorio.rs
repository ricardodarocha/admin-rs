// use actix_session::Session;
use actix_web::get;
use actix_web::{web, Responder, HttpResponse};
use crate::app::AppState;
use crate::repository::relatorio as repo;

#[get("/admin/json/vendasmes")]
pub async fn vendas_por_mes(
    data: web::Data<AppState>

) -> impl Responder {

    let pool = &data.database;
    match repo::fetch_vendas_por_mes(pool).await {
        Ok(vendas) => HttpResponse::Ok().json(vendas),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao buscar dados de vendas"),
    }
}