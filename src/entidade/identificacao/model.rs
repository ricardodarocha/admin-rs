use serde::{Serialize, Deserialize};
use sqlx::FromRow;

use crate::entidade::EntidadeId;

#[derive(Deserialize, Serialize, FromRow)]
pub struct EntidadeIdentificacao {
    pub id: String,
    pub descricao: Option<String>,
    pub tipo_identificacao: Option<String>,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct EntidadeTipoIdentificacao {
    pub id: String,
    pub simbolo: String,
    pub nome: String,
}

impl From<EntidadeTipoIdentificacao> for EntidadeId {
    fn from(value: EntidadeTipoIdentificacao) -> Self {
        EntidadeId {
            id: value.id
        }
    }
}