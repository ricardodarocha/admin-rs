use crate::app::AppState;
use crate::models::grafico::Chart;
use crate::repository::grafico as repo;
use actix_web::{get, web, HttpResponse, Responder};
use log::{error, info};
use sqlx::{Pool, Sqlite};

pub async fn get_charts(pool: &Pool<Sqlite>) -> Vec<Chart> {

    // antes é necessário atualizar os graficos
    let _ = repo::atualizar_charts(pool).await;

    let charts = repo::get_charts_from_db(pool).await;

    match charts {
        Ok(charts_list) => {
            info!("📊 Gráficos recuperados com sucesso.");
            charts_list
        }
        Err(err) => {
            error!("⚠️ Erro ao recuperar gráficos: {}", err);
            vec![]
        }
    }
}

#[get("/grafico/json/{id}")]
pub async fn json_grafico(data: web::Data<AppState>, path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    let pool = &data.database;

    // Chama a função get_charts para buscar todos os gráficos
    let charts = get_charts(pool).await;

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
    let charts = get_charts(pool).await;

    HttpResponse::Ok()
        .content_type("application/json")
        .json(charts)
}
