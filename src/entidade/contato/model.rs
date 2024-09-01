use serde::{Serialize, Deserialize};
use sqlx::FromRow;

use crate::entidade::EntidadeId;

#[derive(Deserialize, Serialize, FromRow)]
pub struct EntidadeContato {
    pub id: String,
    pub descricao: Option<String>,
    pub tipo_contato: Option<String>,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct EntidadeTipoContato {
    pub id: String,
    pub nome: String,
}

impl From<EntidadeTipoContato> for EntidadeId {
    fn from(value: EntidadeTipoContato) -> Self {
        EntidadeId {
            id: value.id
        }
    }
}