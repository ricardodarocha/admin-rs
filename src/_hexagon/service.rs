use crate::entidade::repo as repo;
use crate::entidade::model::*;
use crate::app::AppState;
use actix_web::web;
use crate::infra::result::Result;


pub async fn inserir_entidade(data: web::Data<AppState>, entidade: &PostEntidade) -> Result<Entidade> {
    repo::inserir_entidade(&data.database.conn, entidade).await
    // .map_err(|e| e.to_string() )  
}

pub async fn atualizar_entidade(data: web::Data<AppState>, entidade: &PutEntidade) -> Result<Entidade> {
    update_entidade(&data.database.conn, entidade).await
    // .map_err(|e| e.to_string() )  
}

pub async fn excluir_entidade(data: web::Data<AppState>, entidade_id: String) -> Result<bool> {
    delete_entidade(&data.database.conn, entidade_id).await;
    // .map_err(|e| e.to_string() )  
    Ok(true)
}

pub async fn abrir_entidade(data: web::Data<AppState>, entidade_id: String) -> Result<Entidade> {
    select_entidade_one(&data.database.conn, &entidade_id).await
    // .map_err(|e| e.to_string() )  
}

pub async fn listar_entidades(data: web::Data<AppState>) -> Result<Vec<Entidade>> {
    select_entidades_all(&data.database.conn).await
    // .map_err(|e| e.to_string() )  
}


