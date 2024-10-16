use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Estado {
    pub codigo: i32,
    pub id: String,
    pub nome: String,
    pub _nome: String,
    pub siglauf: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cidade {
    pub id: String,
    pub codigo: i32,
    pub codigoestado: Option<i32>,
    pub nome: String,   
    pub _nome: String,
    pub codigoibge: i32,
    pub uf: Option<String>,
    pub id_estado: String,
    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bairro {
    pub id: String,
    pub nome: String,   
    pub _nome: String,
}   

#[derive(Debug, Serialize, Deserialize)]
pub struct Rua {  
    pub id: String,
    pub nome: String,   
    pub _nome: String,
}     

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BuscaEndereco {    
    pub endereco: Option<String>,
    pub bairro: Option<String>,
    pub cidade: Option<String>,
    pub estado: Option<String>,
    pub cep: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Endereco {
    pub id            : String,
    pub id_logradouro : String,
    pub id_rua        : String,
    pub id_bairro     : String,
    pub numero        : Option<String>,
    pub cep           : Option<String>,
    pub complemento   : Option<String>,
    pub codigocidade  : Option<i32>,
    pub codigoestado  : Option<i32>,
    pub id_estado     : Option<String>,
    pub id_cidade     : Option<String>,
}  