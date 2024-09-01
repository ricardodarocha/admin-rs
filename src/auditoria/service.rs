use crate::auditoria::repo as repo;
use crate::auditoria::model::*;
use actix_web::HttpRequest;
use sqlx::PgPool;
use crate::infra::result::Result;


pub async fn inserir_auditoria(pool: PgPool, auditoria: &PostAuditoria) -> Result<()> {
    repo::inserir_auditoria(&pool, auditoria).await
    // .map_err(|e| e.to_string() )  
}

pub async fn auditar_requisicao(
    pool: &PgPool, 
    requisicao: HttpRequest, 
    // body: Bytes,
    scope: String,
    id_empresa: &String, 
    id_usuario: &String,

) -> Result<()> {
    repo::auditar_requisicao(&pool, &requisicao, 
        // body, 
        id_usuario, id_empresa, scope).await
}   