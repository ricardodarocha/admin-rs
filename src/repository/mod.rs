pub mod cliente;
pub mod pedido;
pub mod produto;

use crate::infra::result::Result;
use crate::models::produto::Produto;
use crate::models::cliente::Cliente;
use crate::models::pedido::{PedidoModel, ItemModel};
use sqlx::{self, Pool, Sqlite};
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

pub async fn abrir_pedido(pool: &Pool<Sqlite>, numero: i64) -> Result<PedidoModel> {
    let query = sqlx::query!(
        r#" SELECT 
            p.num AS "num",
            --p.data AS "data",
            json_object(
                    'id', cli.id,
                    'nome', cli.nome, 
                    'cidade', cli.cidade,
                    'avatar', cli.avatar
                ) AS "cliente: String",
            p.valor AS "valor",
            p.status AS "status: String",
            json_group_array(
                json_object(
                    'num_pedido', i.num_pedido,
                    'produto', 
                         json_object(
                             'id', pro.id,
                             'descricao', pro.descricao,
                             'preco', pro.preco,
                             'avatar', pro.avatar
                             ),
                    'quant', i.quant
                )
            ) AS "itens: String"
        FROM pedido p
        INNER JOIN item i ON p.num = i.num_pedido
        INNER JOIN cliente cli ON p.cliente = cli.id
        INNER JOIN produto pro ON pro.id = i.produto
        WHERE p.num = $1
        GROUP BY p.num"#,
        numero,
    )
    .fetch_one(pool)
    .await;

    match query {
        Ok(pedido) => { 
            let itens: Vec<ItemModel> = serde_json::from_str(&pedido.itens.unwrap_or("[]".to_string())).unwrap(); 
            let cliente: Cliente = serde_json::from_str(&pedido.cliente.unwrap_or("{}".to_string())).unwrap(); 
            let pedido = PedidoModel {
            num: pedido.num,
            // data: pedido.data,
            cliente,
            valor: pedido.valor,
            status: pedido.status,
            itens,
        };

        Ok(pedido)
        },

        Err(err) => Err(err.into()),
    }
}