// use log::{debug, error, log_enabled, info, Level};
use log::info;
use crate::entidade::EntidadeId;
use crate::infra::uuid::{generate_uuid, UuidKind};
// use crate::infra::render::reject_not_found;
use crate::produto::model::*;
use sqlx::{Pool, Postgres};
use crate::infra::result::Result;
use crate::infra::error::Error::*;

pub async fn abrir_produto (
    pool: &Pool<Postgres>,
    id_empresa: String, 
    identificador: &String,

) -> Option<Produto> {

    info!("looking for produto where id, codbarras = {id}", id = identificador.clone());
    
    let result = sqlx::query_as!(
        Produto, r#"
        select p.* from produto p
        inner join empresa e on e.id = p.id_empresa
        where p.id <> '0' and (p.codbarras = $1 or p.id = $1) and (p.id_empresa = $2 or e.id_empresa_produtos = $2)
        "#, 
        identificador,
        id_empresa,
        )
        .fetch_optional(pool).await;

    if let Ok(value) = result {
        info!("Produto localizado");
        value
    }
    else {
        info!("Produto não encontrado");
        None
    }
}

pub async fn inserir_produto (
    pool: &Pool<Postgres>,
    id_empresa: String,
    id: String,
    produto: &PostProduto,

) -> Result<Produto> {  

    let found = abrir_produto(pool, id_empresa.clone(), &id.clone()). await;

    let rec = if let Some(_prod) = found {
        panic!("Tentando inserir um produto que já existe")
    } else
    {

        //verifica se o ID exclusivo já existe
        let ja_existe = sqlx::query_as!(EntidadeId, "SELECT id from produto where id = $1", id)
        .fetch_optional(pool)
        .await.unwrap();

    let novo_id = match ja_existe {
        None => generate_uuid(UuidKind::V7),
        Some(entidade) if entidade.id == "".to_string() => generate_uuid(UuidKind::V7),
        Some(entidade) if entidade.id == "0".to_string() => generate_uuid(UuidKind::V7),
        Some(entidade) if entidade.id.to_lowercase() == "add".to_string() => generate_uuid(UuidKind::V7),
        Some(entidade) if entidade.id.to_lowercase() == "new".to_string() => generate_uuid(UuidKind::V7),
        Some(entidade) if entidade.id.to_uppercase() == "INDEFINIDO".to_string() => generate_uuid(UuidKind::V7),
        Some(entidade) => entidade.id,
    };

        sqlx::query_as!(
            Produto,
            "INSERT INTO produto (
                id,
                id_empresa,
                codbarras,
                nome,
                id_grupo_produto,
                id_categoria_produto,
                preco,
                estoque,
                descricao)
            VALUES ($1
            , $2
            , $3
            , $4
            , $5
            , $6
            , $7
            , $8
            , $9
            )
            RETURNING *",
            novo_id, 
            id_empresa,
            produto.codbarras.as_ref().unwrap(),
            produto.nome,
            produto.id_grupo_produto,
            "INDEFINIDO".to_owned(),
            produto.preco,
            produto.estoque,
            produto.descricao.as_ref().unwrap(),
            // produto.tipo,
            // produto.descricao,
            // produto.url,
            // produto.custo,
            // produto.ativo,
            // produto.referencia,
            // produto.und,
            // produto.medida.unwrap(),
            // produto.tamanho.as_ref().unwrap(),
            // produto.formato,
            // produto.id_grupo_produto,
            // produto.id_categoria_produto,
            // produto.icone) 
        )
        .fetch_one(pool)
        .await?
    };

    Ok(rec)
}

pub async fn atualizar_produto (
    pool: &Pool<Postgres>,
    id_empresa: String,
    id: String,
    produto: &PostProduto,

) -> Result<Produto> {  

    let found = abrir_produto(pool, id_empresa.clone(), &id.clone()). await;

    let rec = if let None = found {
        inserir_produto(pool, id_empresa, id, produto).await.unwrap()
    } else
    {
        sqlx::query_as!(
            Produto,
            "UPDATE produto set 
                id_empresa = $1,
                codbarras = $2,
                nome = $3,
                id_grupo_produto = $4,
                id_categoria_produto = $5 ,
                preco = $6,
                estoque = $7,
                descricao = $8
                where id = $9
            RETURNING *",         
            id_empresa,
            produto.codbarras.as_ref().unwrap(),
            produto.nome,
            produto.id_grupo_produto,
            "INDEFINIDO".to_owned(),
            produto.preco,
            produto.estoque,
            produto.descricao.as_ref().unwrap(),
            id.clone(), 
            // produto.tipo,
            // produto.descricao,
            // produto.url,
            // produto.custo,
            // produto.ativo,
            // produto.referencia,
            // produto.und,
            // produto.medida.unwrap(),
            // produto.tamanho.as_ref().unwrap(),
            // produto.formato,
            // produto.id_grupo_produto,
            // produto.id_categoria_produto,
            // produto.icone) 
        )
        .fetch_one(pool)
        .await?
    };

    Ok(rec)
}

pub async fn alterar_produto (
    pool: &Pool<Postgres>,
    id_empresa: String,
    id: String,
    produto: &PutProduto,

) -> Result<Produto> {  
    
    let found = abrir_produto(pool, id_empresa.clone(), &id.clone()). await;
    if let None = found {
        panic!("Produto não encontrado" )
    };

    let mut transaction = pool.begin().await.unwrap();
    
    if let Some(value) = produto.codbarras.as_ref() {
        sqlx::query_as!(
            Produto,
            "UPDATE  produto SET 
                codbarras = $1 where id = $2",
            value, 
            id)            
        .execute(&mut *transaction)
        .await?;
    } else {};

    if let Some(value) = produto.nome.as_ref() {
        sqlx::query_as!(
            Produto,
            "UPDATE  produto SET 
                nome = $1 where id = $2",
            value, 
            id)            
        .execute(&mut *transaction)
        .await?;
    } else {};
    if let Some(value) = produto.preco.as_ref() {
        sqlx::query_as!(
            Produto,
            "UPDATE  produto SET 
                preco = $1 where id = $2",
            value, 
            id)            
        .execute(&mut *transaction)
        .await?;
    } else {};
    if let Some(value) = produto.estoque.as_ref() {
        sqlx::query_as!(
            Produto,
            "UPDATE  produto SET 
                estoque = $1 where id = $2",
            value, 
            id)            
        .execute(&mut *transaction)
        .await?;
    } else {};

    transaction.commit().await.unwrap();
    

    let rec = abrir_produto(pool, id_empresa, &id).await.unwrap();
    Ok(rec)
}

pub async fn lista_produtos (
    pool: &Pool<Postgres>,
        id_empresa: String,
        args: ProdutoPagination,

) -> Result<Vec<ProdutoList>> {

    info!("looking for produtos where id_empresa = {id}", id = id_empresa.clone());

    let (limit, offset) = (
        args.pagination.size, 
        args.pagination.size * (args.pagination.page - 1),
    );
    let rec = sqlx::query_as! (
        ProdutoList, r#"
        select codigo, 
            id, 
            nome, 
            descricao, 
            codbarras,
            right(id, 6) as id_, 
            id_grupo_produto, 
            id_categoria_produto, 
            preco, 
            estoque 
        from produto 
        where id_empresa = $1 and id <> 'INDEFINIDO' and id <> '0' 
        order by nome limit $2 offset $3
        "#,
        id_empresa,
        limit as i32,
        offset as i32,
    )
        .fetch_all(pool).await;

   match rec {
    Ok(rec) => Ok(rec),
    Err(err) => Err(Sqlx(err))
   }
}

pub async fn lista_grupos_produtos (
    pool: &Pool<Postgres>,
        id_empresa: String,

) -> Result<Vec<GrupoProduto>> {

    let rec =
    sqlx::query_as!(
        GrupoProduto, r#"
        select g.* from grupo_produto g 
        join grupo_produto_empresa e on g.id = e.id_grupo_produto 
        where e.id_empresa = $1 and id <> '0' and id <> 'INDEFINIDO' "#,
        id_empresa)
        .fetch_all(pool).await;

   match rec {
    Ok(rec) => Ok(rec),
    Err(err) => Err(Sqlx(err))
   }
}