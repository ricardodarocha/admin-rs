use sqlx::PgPool;
use crate::entidade::contato::model::*;
use crate::entidade::contato::repo as repo;
use crate::entidade::EntidadeId;
use crate::infra::{result::Result, error::Error};

pub async fn armazenar_email ( 

    pool: &PgPool, 
    _id_empresa: String,
    email: &str,
    
) -> Result<EntidadeContato> {
  
    let id_tipo_contato = repo::abrir_tipo_contato(pool, &"EMAIL".to_owned()).await;
    match id_tipo_contato {
        Some(tipo_contato) => repo::upsert_contato(pool, &email.into(), EntidadeId{id: tipo_contato.id}).await,
        None => Err(Error::Simple(format!("Tipo de contato EMAIL não encontrado ")))
    }
}

pub async fn armazenar_telefone ( 

    pool: &PgPool, 
    _id_empresa: String,
    telefone: &str,
    
) -> Result<EntidadeContato> {
  
    let id_tipo_contato = repo::abrir_tipo_contato(pool, &"TELEFONE".to_owned()).await;
    match id_tipo_contato {
        Some(tipo_contato) => repo::upsert_contato(pool, &telefone.into(), EntidadeId{id: tipo_contato.id}).await,
        None => Err(Error::Simple(format!("Tipo de contato TELEFONE não encontrado ")))
    }
}

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