use sqlx::{Pool, Sqlite};
use crate::models as query;
use crate::models::cliente as model;
use crate::infra::result::Result;
pub async fn abrir_lista_clientes(pool: &Pool<Sqlite>, filtro: &query::QueryFiltroCliente) -> Result<Vec<model::Cliente>> {

    let (limit, offset) = (
        filtro.size, 
        filtro.size * (filtro.page - 1),
    );

    if let Some(cidade) = &filtro.cidade {
        sqlx::query_as!(
                model::Cliente,
                r#" select
                        id,
                        nome,
                        cidade,
                        avatar
                    from cliente
                where cidade = $1
                limit $2 offset $3"#,
                cidade,
                limit,
                offset,
            )
            .fetch_all(pool)
            .await
            .map_err(Into::into)
    } else {
        sqlx::query_as!(
        model::Cliente,
        r#" select
                 id,
                 nome,
                 cidade,
                 avatar
            from cliente
           limit $1 offset $2"#,

        limit,
        offset,
    )
    .fetch_all(pool)
    .await
    .map_err(Into::into)
    }

    
}