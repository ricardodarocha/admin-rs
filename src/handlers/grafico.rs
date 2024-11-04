use crate::app::AppState;
use actix_web::{get, web, HttpResponse, Responder};
use log::error;
use crate::services::grafico as service;

#[get("/grafico/json/{id}")]
pub async fn json_grafico(data: web::Data<AppState>, path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    let pool = &data.database;

    // Chama a função get_charts para buscar todos os gráficos
    let charts = service::get_charts(pool).await;

    // Encontra o gráfico com o ID solicitado
    if let Some(grafico) = charts.into_iter().find(|chart| chart.id == (id as i32)) {
        HttpResponse::Ok()
            .content_type("application/json")
            .json(grafico)
    } else {
        error!("⚠️ Gráfico com ID {} não encontrado.", id);
        HttpResponse::NotFound().finish()
    }
}

#[get("/grafico/json")]
pub async fn json_all_grafico(data: web::Data<AppState>) -> impl Responder {
    let pool = &data.database;

    // Chama a função get_charts para buscar todos os gráficos
    let charts = service::get_charts(pool).await;

    HttpResponse::Ok()
        .content_type("application/json")
        .json(charts)
}
