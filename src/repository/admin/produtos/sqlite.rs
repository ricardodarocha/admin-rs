use sqlx::{Pool, Sqlite};
use actix_web::web;
use crate::models::produto::{FormProduto, Produto};
use crate::infra::result::Result;

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