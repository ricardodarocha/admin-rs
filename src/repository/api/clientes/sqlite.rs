
use crate::infra::result::Result;
use crate::models::cliente::{Cliente, FormCliente};
// use minijinja::value;
use sqlx::{self, Pool, Sqlite};

use crate::models as query;
use crate::models::cliente as model;
pub async fn abrir_lista_clientes(pool: &Pool<Sqlite>, filtro: &query::QueryFiltro) -> Result<Vec<model::Cliente>> {

    let (limit, offset) = (
        filtro.size, 
        filtro.size * (filtro.page - 1),
    );
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

pub async fn abrir_cliente(pool: &Pool<Sqlite>, id: &String) -> Result<Cliente> {
    sqlx::query_as!(
        Cliente,
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

pub async fn inserir_cliente(
    pool: &Pool<Sqlite>, 
    form: FormCliente,

    ) -> Result<String> {

    let id = nanoid::nanoid!(12);
    let _ = sqlx::query_as!(
        cliente,
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
        form.cidade
    )
    .execute(pool)
    .await;
    // .map_err(Into::into)

   Ok(id)

}

use crate::repository::api::clientes::sqlite::model::ClienteNovo;

pub async fn inserir_cliente_json(
    pool: &Pool<Sqlite>, 
    json: ClienteNovo,

) -> Result< String > {
    
    
    let id = nanoid::nanoid!(12);
    let _ = sqlx::query_as!(
        cliente,
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
        json.cidade
    )
    .execute(pool)
    .await;
    // .map_err(Into::into)

    Ok(id) 
}