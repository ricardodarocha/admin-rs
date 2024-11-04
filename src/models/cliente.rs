use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Cliente {
    pub id: String,
    pub nome: String,
    pub cidade: String,
    pub avatar: String,

}

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PostCliente {
    pub id: Option<String>,
    pub nome: String,
    pub cidade: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormCliente {
    pub id: String,
    pub nome: String,
    pub cidade: String,
}