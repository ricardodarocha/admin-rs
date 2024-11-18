use log::info;
use sqlx::{Pool, Sqlite};
use async_trait::async_trait;
use crate::core::entidades::pedido::{EntidadeCliente, EntidadeItem};
use crate::infra::error::Error;
use crate::infra::result::Result;
// use crate::infra::error::Error;
use crate::models::pedido::{EntidadePedido, NovoPedido, PayloadPedido, PostProduto};
use crate::core::tratados::Repository;
use crate::models::QueryFiltroPedido;

#[async_trait]
impl Repository for EntidadePedido {
    type Entity = EntidadePedido;
    type CreatePayload = NovoPedido;
    type UpdatePayload = PayloadPedido;
    type Id = i32;
    // type Filter = QueryFiltroPedido;

    async fn create(pool: &Pool<Sqlite>, payload: &Self::CreatePayload) -> Result<Self::Entity> {
        let mut transacao = pool.begin().await?;

        let next_id = sqlx::query_scalar!("select coalesce(max(num), 0) + 1 from pedido")
            .fetch_one(&mut transacao)
            .await?;

        if let Some(id_novo_pedido) = next_id { 
            let _ = sqlx::query!("insert into pedido (cliente) 
                select id from cliente 
                where upper(id) = upper($1) limit 1 ; ", 
                payload.cliente)
                .execute(pool)
                .await?;

                transacao.commit().await?;

                return Self::get_by_id(pool, id_novo_pedido).await
        } else {

                transacao.rollback().await?;

                Err(Error::Str("Erro ao pegar o próximo código do pedido"))
        }

    }

    async fn update(pool: &Pool<Sqlite>, id: Self::Id, payload: &Self::UpdatePayload) -> Result<Self::Entity> {
        let mut transacao = pool.begin().await?;
        
        //limpa itens e insere novamente
        let _ = sqlx::query!("delete from item where num_pedido = $1", id)
            .execute(&mut transacao)
            .await;
        
        // percorre cada item
        for item in payload.clone().itens.into_iter(){
            
            let id_produto = 
            match item.produto {
                PostProduto::IdProduto(id) => id,
                PostProduto::ProdutoJaExiste(produto_existe) => produto_existe.id,
                _ => return Err(Error::Str("Produto ainda não foi cadastrado")),
            };
                
            let _ = sqlx::query!("insert into item ( num_pedido,
                produto, quant) values ($1, (select id from produto where upper(id) = upper($2)), $3)
                ; ", 
                id,
                id_produto,
                item.quant,
            )
            .execute(&mut transacao)
            .await?;           
            };
        
        info!("⏳ recalculando totais...");

        //atualiza o total
        let _ = sqlx::query!("UPDATE pedido
                SET valor = (
                    SELECT SUM(i.quant * p.preco)
                    FROM item i
                    JOIN produto p ON i.produto = p.id 
                    WHERE i.num_pedido = pedido.num and pedido.num = $1)
                    WHERE pedido.num = $1
                    ; ", 
            id,
    )
    .execute(&mut transacao);

        transacao.commit().await?;


        Self::get_by_id(pool, id).await
    }

    async fn delete(pool: &Pool<Sqlite>, id: Self::Id) -> Result<()> {
        sqlx::query!("DELETE FROM pedido WHERE num = ?", id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn get_by_id(pool: &Pool<Sqlite>, id: Self::Id) -> Result<Self::Entity> {
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
        id,
    )
    .fetch_one(pool)
    .await;

    match query {
        Ok(pedido) => { 
            let itens: Vec<EntidadeItem> = serde_json::from_str(&pedido.itens.unwrap_or("[]".to_string())).unwrap(); 
            let cliente: EntidadeCliente = serde_json::from_str(&pedido.cliente.unwrap_or("{}".to_string())).unwrap(); 
            let pedido = EntidadePedido {
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
}

pub async fn abrir_pedido(pool: &Pool<Sqlite>, numero: i64) -> Result<EntidadePedido> {
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
            let itens: Vec<EntidadeItem> = serde_json::from_str(&pedido.itens.unwrap_or("[]".to_string())).unwrap(); 
            let cliente: EntidadeCliente = serde_json::from_str(&pedido.cliente.unwrap_or("{}".to_string())).unwrap(); 
            let pedido = EntidadePedido {
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
    filtro: &QueryFiltroPedido,

) -> Result<Vec<EntidadePedido>> {
    
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
                             'nome', pro.nome,
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
            let pedidos_model: Vec<EntidadePedido> = pedidos
                .into_iter()
                .map(|pedido| {
                    
                    let itens: Vec<EntidadeItem> = match serde_json::from_str(&pedido.itens.unwrap_or("[]".to_string())) {
                        Ok(itens) => itens,
                        Err(e) => {
                            println!("Erro ao desserializar itens: {:?}", e);
                            Vec::new() // Retorna um vetor vazio em caso de erro
                        }
                    };

                    // Tenta desserializar o cliente; em caso de erro, retorna None e imprime o erro
                    let cliente: Option<EntidadeCliente> = match serde_json::from_str(&pedido.cliente.unwrap_or("{}".to_string())) {
                        Ok(cliente) => Some(cliente),
                        Err(e) => {
                            println!("Erro ao desserializar cliente: {:?}", e);
                            None // Retorna None em caso de erro
                        }
                    };

                    EntidadePedido {
                        num: pedido.num,
                        cliente: cliente.unwrap(),
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