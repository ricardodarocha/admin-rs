use log::info;
use sqlx::{Pool, Sqlite};
use crate::infra::strings::anonimizar;
use crate::models::cliente::Cliente;
use crate::models::pedido::{ItemModel, PedidoModel, PostPedido};
use crate::models as query;
use crate::infra::result::Result;

pub async fn inserir_pedido(pool: &Pool<Sqlite>, cliente: &String) -> Result<i64> {
    // certifica que o cliente existe;
    let nome_cliente = sqlx::query_scalar!("select nome from cliente where id = $1", cliente)
    .fetch_one(pool)
    .await?;

    info!("Inserindo pedido para o cliente {}", anonimizar(nome_cliente.as_ref()));

    let _ = sqlx::query!("insert into pedido (cliente) values ($1); ", cliente)
    .execute(pool)
    .await?;

    let id = sqlx::query_scalar!("select max(num) from pedido where cliente = $1", cliente)
    .fetch_one(pool)
    .await?.unwrap();

    Ok(id)
}

pub async fn inserir_pedido_from_json(pool: &Pool<Sqlite>, pedido: &PostPedido) -> Result<PedidoModel> {
    
    //se o id do pedido foi informado, entao edita
    //se o id do pedido nao foi informado, entao insere
    let id_pedido = if let Some(id_pedido) = pedido.clone().num {
        id_pedido
    } else  {
        inserir_pedido(pool, &pedido.clone().cliente.id.as_ref().unwrap()).await?
    };
    
    info!("Inserindo pedido via json para o cliente {}", anonimizar(pedido.clone().cliente.nome.as_ref()));
    
    //limpa itens e insere novamente
    let _ = sqlx::query!("delete from item where num_pedido = $1", id_pedido).execute(pool).await;
    for item in pedido.clone().itens.into_iter(){
        inserir_item_pedido(pool,  id_pedido, &item).await ?;              
        
    }   

    //atualiza o total
    let _ = sqlx::query!("UPDATE pedido
            SET valor = (
                SELECT SUM(i.quant * p.preco)
                FROM item i
                JOIN produto p ON i.produto = p.id 
                WHERE i.num_pedido = pedido.num and pedido.num = $1)
                 WHERE pedido.num = $1
                ; ", 
        id_pedido,
    )
    .execute(pool)
    .await?;

    let pedido = abrir_pedido(pool, id_pedido).await?;

    Ok(pedido)
}


pub async fn inserir_item_pedido(pool: &Pool<Sqlite>, pedido: i64, item: &ItemModel) -> Result<bool> {
    // certifica que o pedido existe
    let num_pedido = sqlx::query_scalar!("select num from pedido where num  = $1", pedido)
    .fetch_one(pool)
    .await?;

    info!("Inserindo item para o pedido {}", num_pedido);

    let _ = sqlx::query!("insert into item ( num_pedido,
        produto, quant) values ($1, $2, $3)
        ; ", 
        pedido,
        item.produto.id,
        item.quant,
    )
    .execute(pool)
    .await?;

    //atualiza o total
    let _ = sqlx::query!("UPDATE pedido
            SET valor = (
                SELECT SUM(i.quant * p.preco)
                FROM item i
                JOIN produto p ON i.produto = p.id 
                WHERE i.num_pedido = pedido.num and pedido.num = $1)
                 WHERE pedido.num = $1
            ; ", 
        pedido,
    )
    .execute(pool)
    .await?;

    Ok(true)
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
            valor: pedido.valor.unwrap_or_default(),
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
                        valor: pedido.valor.unwrap_or_default(),
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

pub async fn preparar_pedido(pool: &Pool<Sqlite>, id_pedido: i64) -> Result<()> {
      let _ = sqlx::query!("UPDATE pedido
            SET status = 'preparando'
                 WHERE num = $1 and status = 'novo'
            ; ", 
        id_pedido,
    )
    .execute(pool)
    .await?;
    
    Ok(())
}
pub async fn finalizar_pedido(pool: &Pool<Sqlite>, id_pedido: i64) -> Result<()> {
      let _ = sqlx::query!("UPDATE pedido
            SET status = 'pronto'
                 WHERE num = $1 and status = 'preparando'
            ; ", 
        id_pedido,
    )
    .execute(pool)
    .await?;
    
    Ok(())
}