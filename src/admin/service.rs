use crate::admin::repo as repo;
use crate::admin::model::*;
use crate::app::AppState;
use crate::infra::error::Error;
use actix_web::web;
use sqlx::Pool;
use sqlx::Postgres;
use crate::infra::result::Result;


pub async fn inserir_empresa(
        pool: &Pool<Postgres>,
        id_usuario: String, 
        empresa: &PostEmpresa,

    ) -> Result<Empresa> {

    repo::inserir_empresa(
        pool, 
        id_usuario,
        empresa).await
    // .map_err(|e| e.to_string() )  
}

pub async fn atualizar_empresa(
    pool: &Pool<Postgres>, 
    empresa: &PutEmpresa,
    id_empresa: &String,
    ) -> Result<Empresa> {

    repo::atualizar_empresa(pool, empresa, id_empresa).await
    // .map_err(|e| e.to_string() )  
}

pub async fn inserir_account(
    pool: &Pool<Postgres>,
    id_usuario: String, 
    empresa: &PostAccount,

    ) -> Result<Empresa> { 

    repo::inserir_account(
        pool, 
        id_usuario,
        empresa).await
    // .map_err(|e| e.to_string() )  
}

pub async fn atualizar_account(
        pool: &Pool<Postgres>, 
        empresa: &PutEmpresa,
        id_empresa: &String,
    
    ) -> Result<Empresa> {
    
    repo::atualizar_empresa(pool, empresa, id_empresa).await
    // .map_err(|e| e.to_string() )  
}

pub async fn excluir_empresa(data: web::Data<AppState>, empresa_id: String) -> Result<bool> {
    let _ = repo::excluir_empresa(&data.database.conn, empresa_id).await
    .map_err(|e| e.to_string() );  
    Ok(true)
}

pub async fn abrir_empresa(data: web::Data<AppState>, empresa_id: String) -> Result<Empresa> {
    if let Some(value) = repo::abrir_empresa_one(&data.database.conn, &Some(empresa_id)).await? {
        Ok(value)
    } else {
        Err(Error::Str("Não encontrado"))
    }
    // .map_err(|e| e.to_string() )  
}

pub async fn listar_empresas(data: web::Data<AppState>, id_usuario: &String) -> Result<Vec<Empresa>> {
    repo::listar_empresas_all(&data.database.conn, id_usuario).await
    // .map_err(|e| e.to_string() )  
}


