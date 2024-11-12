use serde::{Serialize, Deserialize};
use crate::infra::decimal::decimal;

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Produto {
    pub id: String,
    pub nome: String,
    pub descricao: String,
    pub preco: Option<f64>,
    pub precofmt: Option<String>,
    pub avatar: String,
    
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormProduto {
    pub id: String,
    pub descricao: String,
    pub nome: String,
    
    #[serde(deserialize_with = "decimal")]
    pub preco: f32,
    
}