use sqlx::{Pool, Sqlite};
use crate::models as query;
use crate::models::produto as model;
use crate::infra::result::Result;
pub async fn abrir_lista_produtos(pool: &Pool<Sqlite>, filtro: &query::QueryFiltro) -> Result<Vec<model::Produto>> {

    let (limit, offset) = (
        filtro.size, 
        filtro.size * (filtro.page - 1),
    );
    sqlx::query_as!(
        model::Produto,
        r#" select
                 id,
                 descricao,
                 preco as "preco: f32",
                 avatar
            from produto
           limit $1 offset $2"#,
        limit,
        offset,
    )
    .fetch_all(pool)
    .await
    .map_err(Into::into)
}