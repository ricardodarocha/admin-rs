
use crate::infra::result::Result;
use crate::models::produto::{Produto, FormProduto};
// use minijinja::value;
use sqlx::{self, Pool, Sqlite};
use actix_web::web;
use log::info;

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

pub async fn abrir_cardapio(pool: &Pool<Sqlite>, nome: &String) -> Result<Vec<model::Cardapio>> {
    
    // let (limit, offset) = (
    //     filtro.size, 
    //     filtro.size * (filtro.page - 1),
    // );
    
    let cardapio_result = sqlx::query!(
            r#"SELECT DISTINCT 
    coalesce(json_object(
        'id', p.id,
        'produto', p.nome,
        'descricao', coalesce(p.descricao, '  '),
        'avatar', p.avatar,
        'cardapio', c.id,
        'tamanhos', (
            SELECT json_group_array(
                json_object(
                    'tamanho', c2.opcao,
                    'preco', printf("%.2f", c2.preco) 
                )
            )
            FROM cardapio c2
            WHERE c2.produto = p.id AND c2.id = c.id
        )
    ),'[]') AS "cardapio: String"
FROM 
    produto p
JOIN 
    cardapio c ON c.produto = p.id
WHERE 
    upper(c.id) = upper($1)
    AND EXISTS (
        SELECT 1
        FROM cardapio c2
        WHERE c2.produto = p.id AND c2.id = c.id
    )
ORDER BY 
    c.id, p.id;
        "#,
        nome
    )
    .fetch_all(pool)
    .await?;

   let mut catalogos = Vec::new();

    for row in cardapio_result {
        if let Some(cardapio_json) = &row.cardapio  {
            info!("{}", &cardapio_json);
            // Tenta deserializar o JSON retornado na consulta em uma struct `Cardapio`
            match serde_json::from_str::<model::Cardapio>(cardapio_json) {
                Ok(cardapio) => catalogos.push(cardapio),
                Err(err) => {
                    eprintln!("Erro ao desserializar JSON: {}", err);
                }
            }
        }
    }

    Ok(catalogos)

}
