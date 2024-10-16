use sqlx::PgPool;
use crate::entidade::contato::model::*;
use crate::entidade::contato::repo as repo;
use crate::entidade::EntidadeId;
use crate::infra::{result::Result, error::Error};

pub async fn upsert_contato (

    pool: &PgPool, 
    _id_empresa: String,
    contato: &str,
    tipo_contato: &str,

    
) -> Result<EntidadeContato> {

    let id_tipo_contato = repo::abrir_tipo_contato(pool, &tipo_contato.into()).await;
    match id_tipo_contato {
        Some(tipo_contato) => repo::upsert_contato(pool, &contato.into(), EntidadeId{id: tipo_contato.id}).await,
        None => Err(Error::Simple(format!("Tipo de contato não encontrado {}", tipo_contato)))
    }
}