use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

use crate::{admin::model::Empresa, auth::model::User};

#[derive(Deserialize, Serialize, FromRow)]
pub struct Auditoria {
    pub id: String,
    
    #[serde(with = "time::serde::iso8601")]
    pub created: OffsetDateTime,
    pub id_empresa: String,
    pub id_usuario: String,
    pub id_perfil_usuario: String,
    pub tabela: Option<String>,
    pub valor_antigo: Option<String>,
    pub valor_novo: Option<String>,
    pub operacao: Option<String>,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct PostAuditoria {

    pub id_empresa: String,
    pub id_usuario: String,
    pub id_perfil_usuario: String,
    pub tabela: String,
    pub valor_antigo: String,
    pub valor_novo: String,
    pub operacao: String,
}

pub type PutAuditoria = Auditoria;

#[derive(Deserialize, Serialize, FromRow)]
pub struct ViewAuditoria {
    pub id: String,
    
    #[serde(with = "time::serde::iso8601")]
    pub created: OffsetDateTime,
    pub empresa: Empresa,
    pub usuario: User,
    pub perfil_usuario: String,
    pub tabela: String,
    pub valor_antigo: String,
    pub valor_novo: String,
    pub operacao: String,
}
#[derive(Deserialize, Serialize, FromRow)]
pub struct Requisicao {
    pub id: String,
    
    #[serde(with = "time::serde::iso8601")]    
    pub created: time::OffsetDateTime,
    pub id_servico_api: String,
    pub header: Option<String>,
    pub body: Option<String>,
    pub escope: Option<String>,
    pub status: Option<String>,
    pub id_usuario: String,
    pub id_empresa: String,
    pub response: Option<String>,
    pub origim: Option<String>,
}