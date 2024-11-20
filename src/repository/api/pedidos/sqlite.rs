use log::info;
use sqlx::{Pool, Sqlite};
use async_trait::async_trait;
use crate::core::entidades::pedido::{EntidadeCliente, EntidadeItem};
use crate::infra::error::Error;
use crate::infra::result::Result;
use crate::infra::strings::anonimizar;
// use crate::infra::error::Error;
use crate::models::pedido::{EntidadePedido, NovoPedido, PayloadPedido, PostProduto, PostItem};
use crate::core::tratados::Repository;
use crate::models::QueryFiltroPedido;
use crate::models as query;
use crate::repository as repository;
use crate::services;
use time::OffsetDateTime;

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
            p.data as "data: OffsetDateTime",
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
            data: pedido.data,
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

pub async fn inserir_pedido(pool: &Pool<Sqlite>, cliente: &String) -> Result<i64> {
    // certifica que o cliente existe;
    let nome_cliente = sqlx::query_scalar!("select nome from cliente where upper(id) = upper($1)", cliente)
    .fetch_one(pool)
    .await?;

    info!("Inserindo pedido para o cliente {}", anonimizar(nome_cliente.as_ref()));

    let _ = sqlx::query!("insert into pedido (cliente) select id from cliente where upper(id) = upper($1) limit 1 ; ", cliente)
    .execute(pool)
    .await?;

    let id = sqlx::query_scalar!("select max(num) from pedido where upper(cliente) = upper($1)", cliente)
    .fetch_one(pool)
    .await?.unwrap();

    Ok(id)
}

pub async fn atualizar_pedido(pool: &Pool<Sqlite>, num_pedido: &i64, cliente: &String) -> Result<i64> {
    // Só pode atualizar pedido com status = novo
    let pedido_status = sqlx::query_scalar!(
        r#"select  status  as "status: String" from pedido where num = $1 and lower(status) = 'novo'"#, num_pedido)
    .fetch_one(pool)
    .await;

    if pedido_status.is_err() {
        return Err(Error::Str("Somente pedidos novos podem ser alterados"));
    }
    
    // certifica que o cliente existe;
    let nome_cliente = sqlx::query_scalar!("select nome from cliente where upper(id) = upper($1)", cliente)
    .fetch_one(pool)
    .await?;

    info!("Atualizando pedido {num_pedido}  para o cliente {}", anonimizar(nome_cliente.as_ref()));

    let _ = sqlx::query!("update pedido set cliente =  (select id from cliente where upper(id) = upper($1) limit 1)  where num = $2 ; ", 
    cliente, 
    num_pedido)
    .execute(pool)
    .await?;

    let id = sqlx::query_scalar!("select max(num) from pedido where upper(cliente) = upper($1)", cliente)
    .fetch_one(pool)
    .await?.unwrap();

    Ok(id)
}

pub async fn inserir_pedido_from_json(pool: &Pool<Sqlite>, pedido: &PayloadPedido, id_pedido: &Option<i64>) -> Result<EntidadePedido> {
    
    //Antes de inserir o pedido, vamos verificar se o cliente ja foi cadastrado
    //Para isso precisa verificar o ID do cliente
        let id_cliente = match pedido.clone().cliente {
            query::cliente::PostCliente::IdCliente(id) => id,
            query::cliente::PostCliente::ClienteJaExiste(cliente) => cliente.id,
            query::cliente::PostCliente::NovoCliente(post_cliente) => {
                let id = repository::api::clientes::sqlite::inserir_cliente_json(&pool, post_cliente.clone()).await.unwrap();
                id
            },
        };


    //se o id do pedido foi informado, entao edita
    //se o id do pedido nao foi informado, entao insere, retornando o nome id
    let id_pedido = if let Some(id_pedido) = id_pedido {
        atualizar_pedido(pool, id_pedido, &id_cliente).await?
    } else  {
        inserir_pedido(pool, &id_cliente).await?
    };
    info!("⏳ Criando pedido");
    
    //limpa itens e insere novamente
    let _ = sqlx::query!("delete from item where num_pedido = $1", id_pedido).execute(pool).await;
    for item in pedido.clone().itens.into_iter(){
        inserir_item_pedido(pool,  id_pedido, &item).await ?;              
        
    }   
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
        id_pedido,
    )
    .execute(pool)
    .await?;

    let pedido = abrir_pedido(pool, id_pedido).await?;
    
    info!("✅ Pedido inserido via json para o cliente {}", anonimizar(pedido.clone().cliente.nome.as_ref()));

    Ok(pedido)
}

pub async fn inserir_item_pedido(pool: &Pool<Sqlite>, pedido: i64, item: &PostItem) -> Result<bool> {
    // certifica que o pedido existe
    let num_pedido = sqlx::query_scalar!("select num from pedido where num  = $1", pedido)
    .fetch_one(pool)
    .await?;

    info!("Inserindo item para o pedido {}", num_pedido);

    //Certifica que o item existe
    let id_produto = match item.clone().produto  {
        query::pedido::PostProduto::IdProduto(id) => id,
        query::pedido::PostProduto::ProdutoJaExiste(produto) => produto.id,
        query::pedido::PostProduto::NovoProduto(produto_novo) => {
                let id = services::produto::inserir_produto_json(&pool, produto_novo.clone()).await.unwrap().id;
                id
            },
        };

    let _ = sqlx::query!("insert into item ( num_pedido,
        produto, quant) values ($1, (select id from produto where upper(id) = upper($2)), $3)
        ; ", 
        pedido,
        id_produto,
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

pub async fn abrir_pedido(pool: &Pool<Sqlite>, numero: i64) -> Result<EntidadePedido> {
    let query = sqlx::query!(
        r#" SELECT 
            p.num AS "num",
            p.data as "data: OffsetDateTime",
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
            data: pedido.data,
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
            p.data as "data: OffsetDateTime",
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
                        data: pedido.data,
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