use crate::admin::model::EmpresaAssociada;
use crate::infra::uuid::{generate_uuid, UuidKind};
use crate::entidade::{EntidadeId, EntidadeIdOpt};
use crate::pedido::model::GaleriaItem;
use crate::pedido::CadastroProduto;
use log::info;
use sqlx::{PgPool, Pool, Postgres};
use crate::infra::result::Result;

use super::model::{EntidadeItemPedido, FormItem, Galeria, ItemPedido, PedidoX};

//Retorna a lista de produtos da empresa atual
pub async fn lista_produtos(
    pool: &Pool<Postgres>,
    id_empresa: String,
    
    ) 
    -> Result<Vec<CadastroProduto>> {

    let result = sqlx::query_as!(
    CadastroProduto,    
    r#"SELECT id, nome, to_char(preco, 'L99D99') as preco FROM produto
        where id_empresa = $1 and id <> '0'
    "#,
    id_empresa)
    // r#"select users.*, id as "id: EntidadeId" FROM users"#)
// .bind
    .fetch_all(pool)
    .await?;

    Ok(result)
}

pub async fn abrir_pedido_modelo(
    pool: &PgPool,
    id_empresa: String,
    id_usuario: String,

) -> Option<EntidadeIdOpt> {

    info!("looking for pedido modelo  id_empresa {id}", id = id_empresa.clone());
    info!("                           id_usuario  {id}", id = id_usuario.clone());
        
    let pedido_modelo = sqlx::query_as!(EntidadeIdOpt, 
    "select coalesce(modelo_usuario.id, modelo_empresa.id) id 
	from pedido modelo_empresa
	left join (
		select id from pedido where id_empresa = $1 and id_usuario = $2 and id_tipo_pedido = 'MODELO' limit 1
		) modelo_usuario on modelo_usuario.id <> modelo_empresa.id
    where id_tipo_pedido = 'MODELO' and id_empresa = $1  limit 1",
    id_empresa,
    id_usuario,)
    .fetch_optional(pool)
    .await 
    .unwrap();
    pedido_modelo
}
    

pub async fn criar_pedido_editavel(
    pool: &Pool<Postgres>, 
    id_usuario: String, 
    empresa: Option<String>, 
    cliente: Option<String>,) 

    -> Result<EntidadeId> {
    
    let id = generate_uuid(UuidKind::V7);
    info!("Criando pedido {id}", id = id.clone());
    
    let rec = sqlx::query_as!(
            EntidadeId,
            "INSERT INTO pedido (
                id,
                numero,
                id_usuario,
                id_empresa,
                id_cliente
            )
            VALUES (
              $1
            , (select coalesce(max(numero), 0) + 1 from pedido where id_empresa = $3 and id_cliente = $4)
            , $2
            , $3
            , $4
            )
            RETURNING id",
            id,
            id_usuario,
            empresa,
            cliente)
            .fetch_one(pool)
            .await?;

        let modelo = abrir_pedido_modelo(pool, "".to_string(), id_usuario.clone()).await;
        if let None = modelo {
            info!("Pedido modelo não encontrado");
            return Ok(rec)
        } else {
        //insere itens do pedido, a partir de um pedido modelo
            match modelo.unwrap().id {
                Some(id_modelo) => {

                    info!("Modelo encontrado {id}", id = id_modelo.clone());

                    let _itens = sqlx::query!(
                        "insert into item_pedido (
                    id_empresa, id_cliente, id_pedido, id_produto, preco 
                ) select
                    (select id_empresa from pedido where id = $1), 
                    (select id_cliente from pedido where id = $1), 
                    $1, 
                    pro.id,
                    ROUND( CAST(pro.preco AS numeric), 2 ) 
                    from pedido p, produto pro
                    where p.id = $2"
                    , id
                    , id_modelo)
                            .execute(pool)
                            .await?;
                    },
                    None => {
                        info!("Não encontrou modelo para o usuário {id}", id = id_usuario.clone());
                    }
                };
                
            Ok(rec) 
        }

}

// Traz o ID da empresa se o usuario tem apenas uma empresa configurado, do contrário traz Null
// Isso é importante porque se o usuário tem mais de uma empresa configurada, então a empresa do pedido precisará
// ser informada no formulário do pedido
pub async fn empresa_exclusiva(pool: &Pool<Postgres>, id_usuario: String) -> Option<EntidadeId> {
   sqlx::query_as!(EntidadeId, 
    "select id_empresa as id from empresa_usuario e
inner join (
	select count(id_empresa) as count 
	from empresa_usuario u 
	where u.id_usuario  = $1) as unq 
 on unq.count = 1", id_usuario)
    .fetch_optional(pool)
    .await 
    .unwrap()
}

pub async fn cliente_exclusivo(pool: &Pool<Postgres>, id_usuario: String) -> Option<EntidadeId> {
   sqlx::query_as!(EntidadeId, 
    "select id_empresa as id from empresa_usuario e
inner join (
	select count(id_empresa) as count 
	from empresa_usuario u 
	where u.id_usuario  = $1) as unq 
 on unq.count = 1", id_usuario)
    .fetch_optional(pool)
    .await 
    .unwrap()
}

pub async fn ultimo_pedido_editavel(pool: &Pool<Postgres>, id_usuario: String, id_cliente: String) -> Option<EntidadeId> {
   
   info!("Pegando ultimo pedido id_usuario {}, id_cliente {}", id_usuario.clone(), id_cliente.clone());
   sqlx::query_as!(EntidadeId,
        "select p.id from empresa_usuario u
right join pedido p on p.id_usuario = $1 and p.id_cliente  = $2 and p.id_empresa = u.id_empresa 
where u.id_usuario = $1
and id_tipo_pedido <> 'MODELO'
and id_status_pedido = (select id from status_pedido where descricao = 'NOVO')
order by p.created
limit 1", 
        id_usuario,
        id_cliente,
    ).fetch_optional(pool).await.unwrap()
}

pub async fn abrir_item_pedido(pool: &Pool<Postgres>, id_pedido: String) -> Vec<ItemPedido> {

    info!("looking for id pedido {id}", id = id_pedido.clone());
    let result = sqlx::query_as!(
        ItemPedido, 
    r#"
     with sub as (select 
        ped.numero,
        id_pedido,
        id_produto,
        id_item, 
        p.nome as produto,
        p.descricao,
        coalesce(p.url, 'https://img.freepik.com/free-vector/3d-delivery-box-parcel_78370-825.jpg?size=338&ext=jpg&ga=GA1.1.2008272138.1723334400&semt=ais_hybrid') as url,
        coalesce(i.quantidade, 0.0) as quantidade,
        coalesce(u.simbolo, u.descricao, u.id, 'Unidade') as unidade,
        coalesce(i.preco, p.preco, 0.0) as preco,
        i.campo1::Numeric(16, 0) as medida1,
        p.nome_campo1 as nome_medida1,
        p.id_unidade1 as id_unidade_medida1,
        i.campo2::Numeric(16, 0) as medida2,
        p.nome_campo2 as nome_medida2,
        p.id_unidade2 as id_unidade_medida2,
        i.campo3::Numeric(16, 0) as medida3,
        p.nome_campo3 as nome_medida3,
        p.id_unidade3 as id_unidade_medida3
       from item_pedido i
       inner join pedido ped on ped.id = i.id_pedido
       inner join produto p on p.id = i.id_produto 
       left join unidade u on u.id = p.id_unidade 
       where id_pedido = $1 and p.id <> '0') 
       select 
          numero,
          id_pedido,
          id_produto,
          id_item,
          produto,
          descricao,
          url,
          quantidade::numeric(16,2),	
          unidade,
          preco::numeric(16,2),
          (quantidade * preco)::numeric(16,2) as total,
          medida1, id_unidade_medida1, nome_medida1,       
          medida2, id_unidade_medida2, nome_medida2,
          medida3, id_unidade_medida3, nome_medida3       
       from sub
    "#, 
       id_pedido)
    .fetch_all(pool)
    .await;

    result.unwrap()
}

pub async fn abrir_item_pedido_one(pool: &Pool<Postgres>, id_pedido: String, id_produto: String, id_item: i32) -> ItemPedido {
    
    let result = sqlx::query_as!(
        ItemPedido, 
    r#"with sub as (select 
        ped.numero,
        id_pedido,
        id_produto,
        id_item, 
        p.nome as produto,
        p.descricao,
        coalesce(p.url, 'https://img.freepik.com/free-vector/3d-delivery-box-parcel_78370-825.jpg?size=338&ext=jpg&ga=GA1.1.2008272138.1723334400&semt=ais_hybrid') as url,
        coalesce(i.quantidade, 0.0) as quantidade,
        coalesce(u.simbolo, u.descricao, u.id, 'Unidade') as unidade,
        coalesce(i.preco, p.preco, 0.0) as preco,
        i.campo1::Numeric(16, 0) as medida1,
        p.nome_campo1 as nome_medida1,
        p.id_unidade1 as id_unidade_medida1,
        i.campo2::Numeric(16, 0) as medida2,
        p.nome_campo2 as nome_medida2,
        p.id_unidade2 as id_unidade_medida2,
        i.campo3::Numeric(16, 0) as medida3,
        p.nome_campo3 as nome_medida3,
        p.id_unidade3 as id_unidade_medida3
       from item_pedido i
       inner join pedido ped on ped.id = i.id_pedido
       inner join produto p on p.id = i.id_produto 
       left join unidade u on u.id = p.id_unidade 
       where id_pedido = $1 and  p.id <> '0' and id_produto = $2 and id_item = $3 )
       select 
          numero,
          id_pedido,
          id_produto,
          id_item,
          produto,
          descricao,
          url,
          quantidade::numeric(16,2),	
          unidade,
          preco::numeric(16,2),
          (quantidade * preco)::numeric(16,2) as total,
          medida1, id_unidade_medida1, nome_medida1,       
          medida2, id_unidade_medida2, nome_medida2,
          medida3, id_unidade_medida3, nome_medida3       
       from sub  "#, 
       id_pedido,
       id_produto,
       id_item,
    )
    .fetch_one(pool)
    .await;

    result.unwrap()
}

pub async fn abrir_pedido(pool: &Pool<Postgres>, id_pedido: String) -> Option<EntidadeId> {
    info!("looking for id pedido {id}", id = id_pedido.clone());
    let result = sqlx::query_as!(
        EntidadeId,
    r#"SELECT p.id 
    FROM pedido p
        WHERE p.id = $1"#,
        id_pedido,
    )
    .fetch_optional(pool)
    .await;

    result.unwrap()
}

pub async fn abrir_pedido_legado(pool: &Pool<Postgres>, id_pedido: String) -> PedidoX {
    info!("looking for id pedido {id}", id = id_pedido.clone());
    let result = sqlx::query_as::<_, PedidoX>(
    r#"SELECT p.id, JSON_AGG( jsonb_build_object(
                    'id', i.id_item,
                    'id_produto', i.id_produto,
                    'url', coalesce(pro.url, 'https://img.freepik.com/free-vector/3d-delivery-box-parcel_78370-825.jpg?size=338&ext=jpg&ga=GA1.1.2008272138.1723334400&semt=ais_hybrid'),
                    'produto', pro.nome,
                    'preco', i.preco,
                    'quantidade', i.quantidade  
                )) AS "produtos"
FROM pedido p
        left JOIN item_pedido i ON p.id = i.id_pedido
        left JOIN produto pro ON pro.id = i.id_produto and pro.id <> '0'
        WHERE p.id = $1
        GROUP BY p.id"#
    )
    .bind(id_pedido)
    .fetch_one(pool)
    .await;

    result.unwrap()
}

pub async fn get_galeria(pool: &Pool<Postgres>, id_pedido: String, id_empresa: String, _id_usuario: String) -> Galeria {

    let empresas_associadas: EmpresaAssociada = crate::admin::repo::empresas_associadas(pool, id_empresa).await;
    let id_empresa = empresas_associadas.id_empresa_produto.unwrap();

    let itens =  sqlx::query_as!(
       GaleriaItem,
    r#"with sub as (select 
        p.id,
        p.nome,
        coalesce(p.descricao, '') as descricao,
        coalesce(i.preco, p.preco, 1.0) as preco,
        coalesce(p.url, 'https://img.freepik.com/free-vector/3d-delivery-box-parcel_78370-825.jpg?size=338&ext=jpg&ga=GA1.1.2008272138.1723334400&semt=ais_hybrid') as url,
        coalesce(i.quantidade, 0.0) as quantidade,        
        case when i.id_pedido is null then false else true end as in_cart
       from produto p 
        left join item_pedido i on p.id = i.id_produto and id_pedido = $1 
       where p.id <> '0' and p.id_empresa = $2) 
       select 
       	id,
       	nome,
       	descricao,
       	preco::numeric(16,2),
       	url,
       	quantidade::numeric(16,2),
       	(quantidade * preco)::numeric(16,2) as total,
       	in_cart
         from sub "#,
        id_pedido,
        id_empresa,
        // id_usuario,
    )
    .fetch_all(pool)
    .await;

    let itens = itens.unwrap();
    let x = itens.clone().len();
    info!("Galeria {x}");

    Galeria {
        itens
    }
}

pub async fn get_item_pedido(pool: &Pool<Postgres>, id_pedido: String, id_produto: String, id_item: i32) -> Option<EntidadeItemPedido> {
info!("looking for id pedido {id} id_produto {id_pro} id_item {id_item}", id = id_pedido.clone(), id_pro = id_produto.clone(), id_item = id_item.clone());
    let result = sqlx::query_as!(EntidadeItemPedido,
    r#"SELECT id_item, id_produto, id_pedido                    
        FROM item_pedido p
        WHERE id_pedido = $1 and id_produto = $2 and id_item = $3 limit 1"#,
        id_pedido, 
        id_produto, 
        id_item,
    )
    .fetch_one(pool)
    .await;

    match result {
        Ok(value) => Some(value),    
        Err(_) => None,    }    
    
}

pub async fn update_item_pedido(
    pool: &Pool<Postgres>, 
    id_pedido: String, 
    id_produto: String, 
    id_item: i32, 
    form: FormItem

) -> EntidadeItemPedido {

    info!("looking for id pedido {id} id_produto {id_pro} id_item {id_item}", id = id_pedido.clone(), id_pro = id_produto.clone(), id_item = id_item.clone());
    let result = sqlx::query_as!(EntidadeItemPedido,
    r#"update item_pedido set quantidade = $1, id_produto = $2
        WHERE id_pedido = $3 and id_item = $4 returning id_pedido, id_produto, id_item"#,
        form.quantidade,
        id_produto, 
        id_pedido, 
        id_item,
    )
    .fetch_one(pool)
    .await;

    result.unwrap()
}
pub async fn create_item_pedido(
    pool: &Pool<Postgres>, 
    id_pedido: String, 
    id_produto: String, 
    form: FormItem

) -> EntidadeItemPedido {

    info!("looking for id pedido {id} id_produto {id_pro}", id = id_pedido.clone(), id_pro = id_produto.clone());
    let result = sqlx::query_as!(EntidadeItemPedido,
    r#"insert into item_pedido (id_pedido, id_produto, quantidade)
        values ($1, $2, $3)
        returning id_item, id_produto, id_pedido"#,
        id_pedido, 
        id_produto, 
        form.quantidade,
    )
    .fetch_one(pool)
    .await;

    result.unwrap()
}