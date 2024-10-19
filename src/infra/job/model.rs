use serde::{Serialize, Deserialize};
use serde_json::Value;
use sqlx::prelude::FromRow;
use time::OffsetDateTime;

/// Um job no banco de dados é representado pela tabela "job"
#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct Job {
    pub id: String,
    pub description: String,

    /// O horário agendado quando deve ser executado, no formato UTC Iso88591
    pub execute_at: OffsetDateTime,

    /// Um json que contém todos os parâmetros que serão enviados para o executor
    pub context: Value,

    /// processing... completed
    pub status: String,

    //pub id_usuario: String,
    //pub created_at: OffsetDateTime,
}

#[allow(async_fn_in_trait)]
pub trait Jober {
    async fn run(_job: Job) {}
}
