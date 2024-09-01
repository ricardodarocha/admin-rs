use serde::{Serialize, Deserialize};
use time::{OffsetDateTime, PrimitiveDateTime};
use crate::infra::pagination::Pagination;
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct PessoaPagination {
    #[serde(flatten)]
    pub pagination: Pagination
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct PessoaList {  
    pub id: String, 
   	pub nome: Option<String>,
   	pub razao_social: Option<String>,
   	pub tipo_pessoa: Option<String>,
   	pub identificacao: Option<String>,
   	pub tipo_identificacao: Option<String>,
   	pub status: Option<String>,
   	pub telefone: Option<String>,
   	pub email: Option<String>,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct Pessoa {    
    pub id: String,
    pub cliente: Option<bool>,
    pub fornecedor: Option<bool>,
    pub funcionario: Option<bool>,
    pub motorista: Option<bool>,
    pub id_tipo_pessoa: Option<String>,
    pub id_tipo_identificacao: Option<String>,
    pub id_telefone: Option<String>,
    pub id_email: Option<String>,
    pub id_identificacao: Option<String>,
    pub transportador: Option<bool>,
    pub transportadora: Option<bool>,
    pub contador: Option<bool>,
    pub medico: Option<bool>,
    pub paciente: Option<bool>,
    pub vendedor: Option<bool>,
    pub observacoes: Option<i32>,
    pub data: PrimitiveDateTime,
    pub created: OffsetDateTime,
    pub alterado: OffsetDateTime,
    pub dataalteracao: Option<PrimitiveDateTime>,
    pub grupo: Option<i32>,
    pub perfil: Option<i32>,
    pub cidade: Option<i32>,
    pub credito: Option<f32>,
    pub ativo: bool,
    pub nacionalidade: Option<i32>,
    pub empresa: Option<i32>,
    pub codigo: i32,
    pub id_celular: Option<String>,
    pub id_endereco_financeiro: Option<String>,
    pub id_endereco_comercial: Option<String>,
    pub id_endereco_cobranca: Option<String>,
    pub nome: Option<String>,
    pub razao_social: Option<String>,
    pub tipopessoa: Option<String>,
    pub telefone: Option<String>,
    pub email: Option<String>,
    pub url: Option<String>,
    pub endereco: Option<String>,
    pub uf: Option<String>,
    pub cep: Option<String>,
    pub credencial: Option<String>,
    pub senha: Option<String>,
    pub inscricao_federal: Option<String>,
    pub inscricao_estadual: Option<String>,
    pub abc: Option<String>,
    pub bairro: Option<String>,
    pub avatar: Option<String>,
    pub id_empresa: Option<String>,
    pub id_grupo_pessoa: String,
    pub id_regiao_pessoa: String,
    pub id_perfil_pessoa: String,
    pub id_nacionalidade_pessoa: String,
    pub id_status: String,
  }

#[derive(FromRow)]
pub struct EntityId{
    pub id: String,
}  

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct PostPessoa {
    pub id: Option<String>,
    pub razao_social: String,
    pub nome: Option<String>, 
    pub tipo_pessoa: Option<String>, //PF, PJ
    pub cpf: Option<String>,
    pub cnpj: Option<String>,
    pub email: String,
    pub telefone: String,
    pub endereco: String,
    pub bairro: String,
    pub cidade: String,
    pub estado: String,

}

#[derive(Deserialize)]
pub struct PutPessoaForm {
    pub id: String,
    // pub razao_social: Option<String>, //can not change
    pub nome: Option<String>, 
    pub tipo_pessoa: Option<String>, //can not change
    pub identificacao: Option<String>,  //can change
    pub tipo_identificacao: Option<String>, //CPF, CNPJ //can change
    pub email: Option<String>,
    pub telefone: Option<String>,
}
