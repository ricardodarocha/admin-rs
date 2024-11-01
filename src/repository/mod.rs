pub mod cliente;
pub mod pedido;
pub mod produto;

use crate::infra::result::Result;
use crate::infra::uuid::{generate_uuid, UuidKind};
use crate::models::produto::{Produto, FormProduto};
use crate::models::cliente::{Cliente, FormCliente};
use sqlx::{self, Pool, Sqlite};
use actix_web::web;
// use crate::infra::error::Error;

pub async fn abrir_produto(pool: &Pool<Sqlite>, id: &String) -> Result<Produto> {
    sqlx::query_as!(
        Produto,
        r#" select
                 id,
                 descricao,
                 preco as "preco: f32",
                 avatar
            from produto
           where id = $1"#,
        id,
    )
    .fetch_one(pool)
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

pub async fn atualizar_produto(
    pool: &Pool<Sqlite>, 
        id: &String,
        form: web::Form<FormProduto>,
            
    ) -> Result<Produto> {
    let _ = sqlx::query_as!(
        Produto,
        r#" update Produto set 
                 id = $1,
                 descricao = $2 ,
                 preco  = $3 
           where id = $1"#,
        id,
        form.descricao,
        form.preco,
    )
    .execute(pool)
    .await;

    abrir_produto(pool, id).await
}

pub async fn atualizar_cliente(
    pool: &Pool<Sqlite>, 
    id: &String,
    form: FormCliente,

    ) -> Result<Cliente> {
    
    let _ = sqlx::query_as!(
        Cliente,
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

    abrir_cliente(pool, id).await
}
pub async fn inserir_produto(
    pool: &Pool<Sqlite>, 
    form: web::Form<FormProduto>

    ) -> Result<Produto> {

    let id = generate_uuid(UuidKind::V7);
    let _ = sqlx::query_as!(
        Produto,
        r#" insert into produto
                 (id,
                 descricao,
                 preco) values
                 ($1,
                 $2,
                 $3)
                "#,
        id,
        form.descricao,
        form.preco
    )
    .execute(pool)
    .await;
    // .map_err(Into::into)

   abrir_produto(pool, &id).await 

}

pub async fn inserir_cliente(
    pool: &Pool<Sqlite>, 
    form: FormCliente

    ) -> Result<Cliente> {
    
    let id = generate_uuid(UuidKind::V7);
    let _ = sqlx::query_as!(
        Cliente,
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
    abrir_cliente(pool, &id).await

}