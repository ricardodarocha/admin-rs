
use crate::infra::result::Result;
use crate::models::produto::{Produto, FormProduto};
// use minijinja::value;
use sqlx::{self, Pool, Sqlite};
use actix_web::web;

use crate::models as query;
use crate::models::produto as model;
pub async fn abrir_lista_produtos(pool: &Pool<Sqlite>, filtro: &query::QueryFiltro) -> Result<Vec<model::Produto>> {

    let (limit, offset) = (
        filtro.size, 
        filtro.size * (filtro.page - 1),
    );
    sqlx::query_as!(
        model::Produto,
        r#" select
                 id,
                 nome,
                 descricao,
                 printf("%.2f", preco) as "preco: f64",
                 printf("%.2f", preco) as "precofmt: String",
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

pub async fn abrir_produto(pool: &Pool<Sqlite>, id: &String) -> Result<Produto> {
    sqlx::query_as!(
        Produto,
        r#" select
                 id,
                 nome,
                 descricao,
                 printf("%.2f", preco) as "preco: f64",
                 printf("%.2f", preco) as "precofmt: String",
                 avatar
            from produto
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

pub async fn inserir_produto(
    pool: &Pool<Sqlite>, 
    form: web::Form<FormProduto>

    ) -> Result<String> {

    let id = nanoid::nanoid!(12);
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

   Ok(id)

}

pub async fn inserir_produto_json(
    pool: &Pool<Sqlite>, 
    json: query::pedido::ProdutoNovo

) -> Result< String > {
    
    
    let id = nanoid::nanoid!(12);
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
        json.descricao,
        json.preco
    )
    .execute(pool)
    .await;
    // .map_err(Into::into)

    Ok(id) 
}