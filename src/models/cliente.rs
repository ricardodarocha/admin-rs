use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Cliente {
    pub id: String,
    pub nome: String,
    pub cidade: String,
    pub avatar: String,

}

///Formas de incluir um cliente
/// Cliente novo (Nome, cidade, foto)
/// Cliente existente (id)

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ClienteNovo {
    pub nome: String,
    pub cidade: Option<String>,
    // Dados minimos para inserir o cliente
    // ...
    pub cpf: String,
    pub email: Option<String>,
    pub telefone: Option<String>,
    pub companhia: Option<String>,
    pub cargo: Option<String>,
    // Dados opcionais
    pub avatar: Option<String>,

    #[serde(flatten)]
    outros_campos: HashMap<String, Value>,

} 

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ClienteExiste {
    pub id: String,
    pub nome: Option<String>,
    pub cidade: Option<String>,
    pub avatar: Option<String>,
} 

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ClienteId(
    pub String
);

#[derive(Clone, Serialize, Deserialize)]
pub enum PostCliente {
    IdCliente(ClienteId),
    NovoCliente(ClienteNovo),
    ClienteJaExiste(ClienteExiste),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormCliente {
    pub id: String,
    pub nome: String,
    pub cidade: String,
}