use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Estado {
    pub codigo: i32,
    pub nome: String,
    pub siglauf: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cidade {
    pub codigo: i32,
    pub codigoestado: i32,
    pub nome: String,
    pub codigoibge: i32,
    pub estado: Estado,
}
