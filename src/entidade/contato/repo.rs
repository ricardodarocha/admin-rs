use sqlx::{Pool, Postgres};
use crate::{entidade::EntidadeId, infra::{result::Result, uuid::{generate_uuid, UuidKind}}};
use super::model::*;

pub async fn abrir_tipo_contato (
    pool: &Pool<Postgres>,
    value: &String,
) -> Option<EntidadeTipoContato> {
    sqlx::query_as!(
        EntidadeTipoContato, r#"
        select 
            id, 
            nome
            from tipo_contato
        where nome = $1 or id = $1"#,
        value)
        .fetch_optional(pool).await.unwrap()
}

pub async fn inserir_tipo_contato (
    pool: &Pool<Postgres>,
    value: &String,
) -> EntidadeTipoContato {
        let id = generate_uuid(UuidKind::V7);
        sqlx::query_as!(EntidadeTipoContato, r#"
            insert into tipo_contato (id, nome)
                values ($1, $2) returning id, nome
            "#,
            id,
            value)
        .fetch_one(pool).await.unwrap()
}

// Inclui a entidade contato para email, telefone e endereco
pub async fn upsert_tipo_contato (
    pool: &Pool<Postgres>,
    value: &String,
) -> Result<EntidadeTipoContato> {
    
    let entidade = abrir_tipo_contato(pool, value).await;

    if let Some(entidade) = entidade {
        Ok(entidade)
    } else
    {   
        let novo_tipo_contato = inserir_tipo_contato(pool, value).await;
        Ok(novo_tipo_contato)
    }
}

pub async fn abrir_contato (
    pool: &Pool<Postgres>,
    value: &String,
) -> Option<EntidadeContato> {
    sqlx::query_as!(
        EntidadeContato, r#"
        select 
            c.id, 
            descricao,
            nome as tipo_contato 
            from contato c join tipo_contato tc on tc.id = c.id_tipo_contato
        where descricao = $1"#,
        value)
        .fetch_optional(pool).await.unwrap()
}

pub async fn inserir_contato (
    pool: &Pool<Postgres>,
    value: &String,
    tipo_contato: EntidadeId,
) -> Option<EntidadeContato> {
        dbg!(value.clone());
        dbg!(tipo_contato.clone());
        let id = generate_uuid(UuidKind::V7);
        _ = sqlx::query_as!(EntidadeId, r#"
            insert into contato (id, id_tipo_contato, descricao)
                values ($1, $2, $3) returning id
            "#,
            id,
            tipo_contato.id,
            value)
        .fetch_one(pool).await;

    abrir_contato(pool, value).await
}
// Inclui a entidade contato para email, telefone e endereco
pub async fn upsert_contato (
    pool: &Pool<Postgres>,
    value: &String,
    tipo_contato: EntidadeId,
) -> Result<EntidadeContato> {
    
    let entidade = abrir_contato(pool, value).await;

    if let Some(entidade) = entidade {
        Ok(entidade)
    } else
    {   
        let novo_contato = inserir_contato(pool, value, tipo_contato).await;
        Ok(novo_contato.unwrap())
    }
}