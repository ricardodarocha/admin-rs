use crate::models::grafico::Chart;
use crate::repository::grafico as repo;
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