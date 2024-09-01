pub mod contato;
pub mod identificacao;

use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
pub struct EntidadeId {
    pub id: String,
}
#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
pub struct EntidadeIdOpt {
    pub id: Option<String>,
}