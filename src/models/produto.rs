use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Produto {
    pub id: String,
    pub descricao: String,
    pub preco: f32,
    pub avatar: String,
    
}