use std::fmt;

use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Clone, Deserialize, Serialize, FromRow)]
pub struct Empresa {
    pub id: String,
    pub nome: String, 
    pub id_cnpj: Option<String>,
    pub cnpj: Option<String>,
}

impl fmt::Display for Empresa {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.nome)
    }
}

#[derive(Serialize, Deserialize)]
pub struct PostEmpresa {
    pub id: Option<String>,
    pub nome: String,
    pub cnpj: Option<String>,
    pub email: Option<String>,
    pub telefone: Option<String>,
}

impl fmt::Display for PostEmpresa {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.nome)
    }
}

impl From<crate::auth::model::PrimeiroAcesso> for PostEmpresa {
    fn from(value: crate::auth::model::PrimeiroAcesso) -> Self {
        PostEmpresa {
            id: None,
            nome: value.nome_responsavel,
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
    pub fantasia: Option<String>,
    pub nome_responsavel: Option<String>,
    pub cpf: Option<String>,
    pub email: Option<String>,
    pub telefone: Option<String>,        
    pub segmento: Option<String>, 
    pub endereco_principal: Option<String>,
    pub bairro_principal: Option<String>,
    pub cidade_principal: Option<String>,
    pub estado_principal: Option<String>,
    pub cep_principal: Option<String>,
    pub endereco_cobranca: Option<String>,
    pub bairro_cobranca: Option<String>,
    pub cidade_cobranca: Option<String>,
    pub estado_cobranca: Option<String>,
    pub cep_cobranca: Option<String>,
    pub endereco_entrega: Option<String>,
    pub bairro_entrega: Option<String>,
    pub cidade_entrega: Option<String>,
    pub estado_entrega: Option<String>,
    pub cep_entrega: Option<String>,
}

impl From<PostAccount> for PutEmpresa {
    fn from(value: PostAccount) -> Self {
        PutEmpresa {
            id: value.id,
            nome: Some(value.nome_usuario),
            fantasia: value.nome_fantasia,
            nome_responsavel: Some(value.nome_responsavel),
            cpf: Some(value.cpf_responsavel),
            email: Some(value.email),
            telefone: Some(value.telefone),
            segmento: Some(value.segmento),
            endereco_principal: Some(value.endereco_principal),
            bairro_principal: Some(value.bairro_principal),
            cidade_principal: Some(value.cidade_principal),
            estado_principal: Some(value.estado_principal),
            cep_principal: Some(value.cep_principal),
            endereco_cobranca: value.endereco_cobranca,
            bairro_cobranca: value.bairro_cobranca,
            cidade_cobranca: value.cidade_cobranca,
            estado_cobranca: value.estado_cobranca,
            cep_cobranca: value.cep_cobranca ,
            endereco_entrega: value.endereco_entrega,
            bairro_entrega: value.bairro_entrega,
            cep_entrega: value.cep_entrega ,
            cidade_entrega: value.cidade_entrega,
            estado_entrega: value.estado_entrega,
        }
    }
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

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct PostAccount {
    pub id: String,
    pub nome_usuario: String,
    pub email_usuario: String, 
    pub razao_social: String, 
    pub nome_fantasia: Option<String>, 
    pub tipo_identificacao: String,
    pub cnpj: String,
    pub segmento: String,
    pub email: String,
    pub telefone: String,
    pub nome_responsavel: String,
    pub cpf_responsavel: String,
    pub endereco_principal: String,
    pub bairro_principal: String,
    pub cidade_principal: String,
    pub estado_principal: String,
    pub cep_principal: String,
    pub endereco_cobranca: Option<String>,
    pub bairro_cobranca: Option<String>,
    pub cidade_cobranca: Option<String>,
    pub estado_cobranca: Option<String>,
    pub cep_cobranca: Option<String>,
    pub endereco_entrega: Option<String>,
    pub bairro_entrega: Option<String>,
    pub cidade_entrega: Option<String>,
    pub estado_entrega: Option<String>,
    pub cep_entrega: Option<String>,
    
}

impl fmt::Display for PostAccount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.nome_fantasia)
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct DadosAccount {
    pub id_usuario: String,
    pub id_empresa: String,
    pub nome_usuario: String,
    pub email_usuario: String, 
    pub razao_social: String, 
    pub nome_fantasia: Option<String>, 
    pub tipo_identificacao: String,
    pub cnpj: Option<String>,
    pub segmento: Option<String>,
    pub email: String,
    pub telefone: String,
    pub nome_responsavel: String,
    pub cpf_responsavel: Option<String>,
    pub endereco_principal: Option<String>,
    pub bairro_principal: Option<String>,
    pub cidade_principal: Option<String>,
    pub estado_principal: Option<String>,
    pub cep_principal: Option<String>,
    pub endereco_cobranca: Option<String>,
    pub bairro_cobranca: Option<String>,
    pub cidade_cobranca: Option<String>,
    pub estado_cobranca: Option<String>,
    pub cep_cobranca: Option<String>,
    pub endereco_entrega: Option<String>,
    pub bairro_entrega: Option<String>,
    pub cidade_entrega: Option<String>,
    pub estado_entrega: Option<String>,
    pub cep_entrega: Option<String>,
    
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Segmento
{
    pub id: String,
    pub nome: String,
    pub classe: String,
}