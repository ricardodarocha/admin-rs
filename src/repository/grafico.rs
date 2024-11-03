use crate::models::grafico::{Chart, Series};
use sqlx::{Pool, Sqlite};
use crate::infra::result::Result;
// use crate::repository::grafico as repo;
// use log::{info, error};

pub async fn atualizar_charts(pool: &Pool<Sqlite>) -> Result<()> {
  sqlx::query!("delete from series")
        .execute(pool)
        .await?; 
  sqlx::query!("delete from charts")
        .execute(pool)
        .await?; 

    //Insere grafico de vendas
    sqlx::query!("insert into charts (title, labels, valores) values ('Vendas', 
    (SELECT group_concat(mes)   FROM (SELECT mes FROM vendas_mensais GROUP BY mes ORDER BY ano DESC, mes_numero)),
    (SELECT group_concat(total_vendas) FROM (SELECT mes, SUM(total_vendas) AS total_vendas FROM vendas_mensais
     GROUP BY mes ORDER BY ano DESC, mes_numero)) 
    ) ")
        .execute(pool)
        .await?; 

    //Insere todas as séries do gráfico de vendas
    sqlx::query!(" INSERT INTO series (chart_id, valores, tipo, nome)
SELECT 
    id,
    --(SELECT group_concat(mes)   FROM (SELECT mes FROM vendas_mensais GROUP BY mes ORDER BY ano DESC, mes_numero)) AS labels,
    (SELECT group_concat(total_vendas) FROM (SELECT mes, SUM(total_vendas) AS total_vendas FROM vendas_mensais
     GROUP BY mes ORDER BY ano DESC, mes_numero)) AS valores,
    'bar' AS tipo,
    'Vendas'
FROM charts
WHERE title = 'Vendas'
LIMIT 1 ")
        .execute(pool)
        .await?; 

    Ok(())
}

pub async fn get_charts_from_db(pool: &Pool<Sqlite>) -> Result<Vec<Chart>> {
    // Query para buscar todos os gráficos
    let charts_data = sqlx::query!("SELECT id, title, labels, valores FROM charts")
        .fetch_all(pool)
        .await?;

    let mut charts = Vec::new();

    for chart_row in charts_data {
        // Divide os rótulos da string em um vetor
        let labels: Vec<String> =  if let Some(labels) = chart_row.labels {
            labels.split(',').map(|s| s.trim().to_string()).collect()
        } else {
            vec!()};

        // Query para buscar as séries associadas a este gráfico
        let series_data = sqlx::query!(
            "SELECT nome, tipo, valores, backgroundColor, borderColor, borderWidth FROM series WHERE chart_id = ?",
            chart_row.id
        )
        .fetch_all(pool)
        .await?;

        let valores_chart: Vec<f32> = if let Some(valores) = chart_row.valores {
            valores.split(',')
                .filter_map(|v| v.trim().parse().ok())
                .collect()
        } else {
            Vec::new()
        };

        let mut series_list = Vec::new();

       for series_row in series_data {
    // Divide os valores e cores em vetores, ou utiliza um vetor vazio se o campo for None
    let valores: Vec<f32> = if let Some(valores) = series_row.valores {
        valores.split(',')
            .filter_map(|v| v.trim().parse().ok())
            .collect()
    } else {
        Vec::new()
    };

    let background_color: Vec<String> = if let Some(background_color) = series_row.backgroundColor {
        background_color.split(';')
            .map(|s| s.trim().to_string())
            .collect()
    } else {
        Vec::new()
    };

    let border_color: Vec<String> = if let Some(border_color) = series_row.borderColor {
        border_color.split(';')
            .map(|s| s.trim().to_string())
            .collect()
    } else {
        Vec::new()
    };

    series_list.push(Series {
        name: series_row.nome.unwrap_or_default(),
        tipo: series_row.tipo.unwrap_or_default(),
        values: valores,
        background_color,
        border_color,
        border_width: series_row.borderWidth.unwrap_or(1) as i32,
    });
}

        charts.push(Chart {
            id: chart_row.id as i32,
            title: chart_row.title.unwrap_or_default(),
            labels,
            series: series_list,
            valores: valores_chart,
        });
    }

    Ok(charts)
}