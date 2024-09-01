use sqlx::{Pool, Postgres};
use crate::{entidade::EntidadeId, infra::{result::Result, uuid::{generate_uuid, UuidKind}}};
use super::model::*;

pub async fn abrir_tipo_identificacao (
    pool: &Pool<Postgres>,
    value: &String,
) -> Option<EntidadeTipoIdentificacao> {
    sqlx::query_as!(
        EntidadeTipoIdentificacao, r#"
        select 
            id, 
            nome,
            simbolo
            from tipo_identificacao
        where nome = $1 or simbolo = $1 or id = $1"#,
        value)
        .fetch_optional(pool).await.unwrap()
}

pub async fn inserir_tipo_identificacao (
    pool: &Pool<Postgres>,
    value: &String,
    simbolo: &String,
) -> EntidadeTipoIdentificacao {
        let id = generate_uuid(UuidKind::V7);
        sqlx::query_as!(EntidadeTipoIdentificacao, r#"
            insert into tipo_identificacao (id, nome, simbolo)
                values ($1, $2, $3) returning id, nome, simbolo
            "#,
            id,
            value,
        simbolo)
        .fetch_one(pool).await.unwrap()
}

// Inclui a entidade identificacao para email, telefone e endereco
pub async fn upsert_tipo_identificacao (
    pool: &Pool<Postgres>,
    value: &String,
    simbolo: &String,
) -> Result<EntidadeTipoIdentificacao> {
    
    let entidade = abrir_tipo_identificacao(pool, value).await;

    if let Some(entidade) = entidade {
        Ok(entidade)
    } else
    {   
        let novo_tipo_identificacao = inserir_tipo_identificacao(pool, value, simbolo).await;
        Ok(novo_tipo_identificacao)
    }
}

pub async fn abrir_identificacao (
    pool: &Pool<Postgres>,
    value: &String,
) -> Option<EntidadeIdentificacao> {
    sqlx::query_as!(
        EntidadeIdentificacao, r#"
        select 
            c.id, 
            descricao,
            nome as tipo_identificacao 
            from identificacao c join tipo_identificacao tc on tc.id = c.id_tipo_identificacao
        where descricao = $1"#,
        value)
        .fetch_optional(pool).await.unwrap()
}

pub async fn inserir_identificacao (
    pool: &Pool<Postgres>,
    value: &String,
    tipo_identificacao: EntidadeId,
) -> Option<EntidadeIdentificacao> {
        let id = generate_uuid(UuidKind::V7);
        _ = sqlx::query_as!(EntidadeId, r#"
            insert into identificacao (id, id_tipo_identificacao, descricao)
                values ($1, $2, $3)
            "#,
            id,
            tipo_identificacao.id,
            value)
        .fetch_one(pool).await;

    abrir_identificacao(pool, value).await
}
// Inclui a entidade identificacao para email, telefone e endereco
pub async fn upsert_identificacao (
    pool: &Pool<Postgres>,
    value: &String,
    tipo_identificacao: EntidadeId,
) -> Result<EntidadeIdentificacao> {
    
    let entidade = abrir_identificacao(pool, value).await;

    if let Some(entidade) = entidade {
        Ok(entidade)
    } else
    {   
        let novo_identificacao = inserir_identificacao(pool, value, tipo_identificacao).await;
        Ok(novo_identificacao.unwrap())
    }
}