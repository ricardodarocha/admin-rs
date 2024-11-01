use sqlx::{Pool, Sqlite};
use crate::models::cliente::Cliente;
use crate::models::pedido::{PedidoModel, ItemModel};
use crate::models as query;
use crate::infra::result::Result;

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

pub async fn abrir_lista_pedidos(
    pool: &Pool<Sqlite>, 
    cliente: &String, 
    filtro: &query::QueryFiltroPedido,

) -> Result<Vec<PedidoModel>> {
    
    let (limit, offset) = (
        filtro.size, 
        filtro.size * (filtro.page - 1),
    );
    
    let pedidos_result = sqlx::query!(
        r#"
        SELECT 
            p.num AS "num",
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
        WHERE p.cliente = $1
        GROUP BY p.num
        limit $2 offset $3
        "#,
        cliente,
        limit,
        offset
    )
    .fetch_all(pool)
    .await;

    match pedidos_result {
        Ok(pedidos) => {
            let pedidos_model: Vec<PedidoModel> = pedidos
                .into_iter()
                .map(|pedido| {
                    
                    let itens: Vec<ItemModel> = serde_json::from_str(&pedido.itens.unwrap_or("[]".to_string())).unwrap();
                    let cliente: Cliente = serde_json::from_str(&pedido.cliente.unwrap_or("{}".to_string())).unwrap();

                    PedidoModel {
                        num: pedido.num,
                        cliente,
                        valor: pedido.valor,
                        status: pedido.status,
                        itens,
                    }
                })
                .collect();

            Ok(pedidos_model)
        },
        Err(err) => Err(err.into()),
    }
}