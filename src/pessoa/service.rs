use crate::pessoa::repo as repo;
use crate::pessoa::model::*;
use crate::app::AppState;
use actix_web::web;
use crate::infra::result::Result;


pub async fn inserir_pessoa(data: web::Data<AppState>, pessoa: &PostPessoa) -> Result<Pessoa> {
    repo::inserir_pessoa(&data.database.conn, pessoa).await
    // .map_err(|e| e.to_string() )  
}

// pub async fn atualizar_pessoa(data: web::Data<AppState>, pessoa: &PutPessoa) -> Result<Pessoa> {
//     repo::atualizar_pessoa(&data.database.conn, pessoa).await
//     // .map_err(|e| e.to_string() )  
// }

// pub async fn excluir_pessoa(data: web::Data<AppState>, pessoa_id: String) -> Result<bool> {
//     repo::excluir_pessoa(&data.database.conn, pessoa_id).await;
//     // .map_err(|e| e.to_string() )  
//     Ok(true)
// }

pub async fn abrir_pessoa(data: web::Data<AppState>, id_empresa: String, pessoa_id: String) -> Result<Pessoa> {
    Ok(repo::abrir_pessoa(&data.database.conn, id_empresa, &pessoa_id).await.unwrap())
    // .map_err(|e| e.to_string() )  
}

pub async fn listar_pessoas(data: web::Data<AppState>, id_empresa: String, args: PessoaPagination) -> Result<Vec<PessoaList>> {
    Ok(repo::listar_pessoas_all(&data.database.conn, id_empresa, args).await.unwrap())
    // .map_err(|e| e.to_string() )  
}


