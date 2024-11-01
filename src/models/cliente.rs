use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Cliente {
    pub id: String,
    pub nome: String,
    pub cidade: String,
    pub avatar: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormCliente {
    pub id: String,
    pub nome: String,
    pub cidade: String,
}