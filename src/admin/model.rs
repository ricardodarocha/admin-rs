use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, FromRow)]
pub struct Empresa {
    pub id: String,
    pub nome: String, 
    pub id_cnpj: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PostEmpresa {
    pub id: Option<String>,
    pub nome: String,
    pub cnpj: Option<String>,
    pub email: Option<String>,
    pub telefone: Option<String>,
}

impl From<crate::auth::model::PrimeiroAcesso> for PostEmpresa {
    fn from(value: crate::auth::model::PrimeiroAcesso) -> Self {
        PostEmpresa {
            id: None,
            nome: value.nome,
            cnpj: Some(value.cnpj),
            email: Some(value.email),
            telefone: Some(value.telefone),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PutEmpresa {
    pub id: String,
    pub nome: Option<String>,
    pub cnpj: Option<String>,
    pub telefone: Option<String>,
    pub responsavel: Option<String>,
    pub cpf: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EmpresaAssociada {
    pub id: String,
    pub id_empresa_pessoa: Option<String>,
    pub id_empresa_produto: Option<String>,
}
// impl From<crate::auth::model::PrimeiroAcesso> for PutEmpresa {
//     fn from(value: crate::auth::model::PrimeiroAcesso) -> Self {
//         PutEmpresa {
//             id: None,
//             nome: Some(value.nome),
//             cnpj: Some(value.cnpj),
//             email: Some(value.email),
//             telefone: Some(value.telefone),
//             email: Some(value.email),
//             cpf: Some(value.cpf),
//             responsavel: Some(value.responsavel),
//         }
//     }
// }