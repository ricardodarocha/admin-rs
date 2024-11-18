
use crate::infra::result::Result;

use sqlx::{self, Pool, Sqlite};
use crate::models as query;
use crate::models::cliente as model;

pub async fn abrir_cliente(pool: &Pool<Sqlite>, id: &String) -> Result<model::Cliente> {
    sqlx::query_as!(
        model::Cliente,
        r#" select
                 id,
                 nome,
                 cidade,
                 avatar
            from cliente
           where id = $1"#,
        id,
    )
    .fetch_one(pool)
    .await
    .map_err(Into::into)
}

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

pub async fn inserir_cliente(
    pool: &Pool<Sqlite>, 
    form: model::FormCliente

    ) -> Result< String> {
    
    let id = nanoid::nanoid!(12);
    let _ = sqlx::query!(
        r#" insert into cliente
                 (id,
                 nome,
                 cidade) values
                 ($1,
                 $2,
                 $3)
                "#,
        id,
        form.nome,
        form.cidade,
    )
    .execute(pool)
    .await;
    // .map_err(Into::into)
    Ok(id)

}

pub async fn inserir_cliente_json(
    pool: &Pool<Sqlite>, 
    json:  model::ClienteNovo,

) -> Result< String > {
   
    let id = nanoid::nanoid!(12);
    let _ = sqlx::query!(
        r#" insert into cliente
                 (id,
                 nome,
                 cidade) values
                 ($1,
                 $2,
                 $3)
                "#,
        id,
        json.nome,
        json.cidade,
    )
    .execute(pool)
    .await;
    // .map_err(Into::into)
    Ok(id)
}

pub async fn atualizar_cliente(
    pool: &Pool<Sqlite>, 
    id: &String,
    form:  model::FormCliente,

    ) -> Result< String > {
    
    let _ = sqlx::query_as!(
         model::Cliente,
        r#" update Cliente set 
                 id = $1,
                 nome = $2,
                 cidade = $3
           where id = $1"#,
        id,
        form.nome,
        form.cidade
    )
    .execute(pool)
    .await;

    Ok(id.clone())
}