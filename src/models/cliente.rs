use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Cliente {
    pub id: String,
    pub nome: String,
    pub cidade: String,
    pub avatar: String,
}