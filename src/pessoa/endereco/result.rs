use serde::{Serialize, Deserialize};
use crate::pessoa::endereco::model::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Endereco {
    pub id            : String,
    pub logradouro    : Rua,
    // pub numero        : Option<String>, 
    // pub complemento   : Option<String>, 
    pub bairro        : Bairro,
    pub cidade        : Option<Cidade>,
    pub estado        : Option<Estado>,
    pub cep           : Option<String>,
    pub complemento   : Option<String>,
}  