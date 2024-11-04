use sqlx::{Pool, Sqlite};
use crate::models::relatorio::VendasMes;
pub use crate::infra::result::Result;
pub async fn fetch_vendas_por_mes(pool: &Pool<Sqlite>) -> Result<Vec<VendasMes>> {
    let vendas = sqlx::query_as!(
        VendasMes,
        r#"
        SELECT 
            ano, 
            mes_numero,  
            mes, 
            coalesce(SUM(total_vendas),0.0) AS total
        FROM 
            vendas_mensais
        GROUP BY 
            ano, mes_numero, mes
        ORDER BY 
            ano DESC, mes_numero DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(vendas)
}