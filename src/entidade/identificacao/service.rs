use sqlx::PgPool;
use crate::entidade::identificacao::model::*;
use crate::entidade::identificacao::repo as repo;
use crate::entidade::EntidadeId;
use crate::infra::{result::Result, error::Error};

pub async fn armazenar_cpf ( 

    pool: &PgPool, 
    _id_empresa: String,
    cpf: &str,
    
) -> Result<EntidadeIdentificacao> {
  
    let id_tipo_identificacao = repo::abrir_tipo_identificacao(pool, &"CPF".to_owned()).await;
    match id_tipo_identificacao {
        Some(tipo_identificacao) => repo::upsert_identificacao(pool, &cpf.into(), EntidadeId{id: tipo_identificacao.id}).await,
        None => Err(Error::Simple(format!("Tipo de identificação CPF não encontrado")))
    }
}

pub async fn armazenar_cnpj ( 

    pool: &PgPool, 
    _id_empresa: String,
    cnpj: &str,
    
) -> Result<EntidadeIdentificacao> {
  
    let id_tipo_identificacao = repo::abrir_tipo_identificacao(pool, &"CNPJ".to_owned()).await;
    match id_tipo_identificacao {
        Some(tipo_identificacao) => repo::upsert_identificacao(pool, &cnpj.into(), EntidadeId{id: tipo_identificacao.id}).await,
        None => Err(Error::Simple(format!("Tipo de identificação CNPJ não encontrado")))
    }
}

pub async fn armazenar_rg ( 

    pool: &PgPool, 
    _id_empresa: String,
    rg: &str,
    
) -> Result<EntidadeIdentificacao> {
  
    let id_tipo_identificacao = repo::abrir_tipo_identificacao(pool, &"RG".to_owned()).await;
    match id_tipo_identificacao {
        Some(tipo_identificacao) => repo::upsert_identificacao(pool, &rg.into(), EntidadeId{id: tipo_identificacao.id}).await,
        None => Err(Error::Simple(format!("Tipo de identificação RG não encontrado")))
    }
}

pub async fn upsert_identificacao (

    pool: &PgPool, 
    _id_empresa: String,
    identificacao: &str,
    tipo_identificacao: &str,

    
) -> Result<EntidadeIdentificacao> {

    let id_tipo_identificacao = repo::abrir_tipo_identificacao(pool, &tipo_identificacao.into()).await;
    match id_tipo_identificacao {
        Some(tipo_identificacao) => repo::upsert_identificacao(pool, &identificacao.into(), EntidadeId{id: tipo_identificacao.id}).await,
        None => Err(Error::Simple(format!("Tipo de identificacao não encontrado {}", tipo_identificacao)))
    }
}