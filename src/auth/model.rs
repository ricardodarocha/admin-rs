use actix_session::Session;
// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde::de::Deserializer;
use sqlx::FromRow;
use time::OffsetDateTime;
use time::serde::iso8601::option;

#[derive(Debug, Deserialize, Serialize, FromRow, Clone)]
pub struct User {
    pub id: String,
    pub nome: String, //nome para exibição
    pub email: Option<String>,
    pub photo: Option<String>,
    pub id_empresa: Option<String>,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String, // Senha em texto puro para comparação
}
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct SubscribeForm {
    pub nome: String,
    pub email: String, 
}
#[derive(FromRow, Serialize, Deserialize)]
pub struct Subscriber {
    pub id: String,
    pub nome: Option<String>,
        
    #[serde(with = "time::serde::iso8601")]
    pub created: OffsetDateTime,
    pub id_email: Option<String>,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct Recado {
    pub codigo: i32,
    pub id: Option<String>,
    pub mensagem: Option<String>,
    pub url: Option<String>,
        
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,

    #[serde(with = "option")]
    pub readed_at: Option<OffsetDateTime>,

    pub id_usuario: String,
    
}

    #[allow(unused)]
    #[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
    struct Profile {
        pub id: i64,

        #[serde(with = "time::serde::iso8601")]
        pub created_at: OffsetDateTime,

        #[serde(with = "time::serde::iso8601")]
        pub updated_at: OffsetDateTime,
        pub user_name: String,
        pub full_name: String
    }
    
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct RegisterData {
    pub cnpj: String,
    pub email: String,
    pub razao_social: String,
    pub telefone: String,
    pub user_id: String,
}
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct RegisterUser {
    pub nome: String, //Nome para exibição
    pub instituicao: String,
    pub email: String,
    pub telefone: String,
    pub password: Option<String>, //senha sem criptografia
}

fn lgpd<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    match s {
        "Aceito" => Ok(true),
        _ => Ok(false),
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct PrimeiroAcesso {
    pub nome_instituicao: String, //Nome da instituição
    pub cnpj: String, //Cnpj da instituição
    pub segmento: String, //Segmento da instituição
    pub email: String, //email do responsavel
    pub nome_responsavel: String, //nome do responsável
    pub cpf: String, //cpf do responsavel
    pub telefone: String, //telefone da empresa, ou telefone do responsável
    pub password: Option<String>, //senha sem criptografia; normalmente a primeira senha será gerada pelo programa
    
    #[serde(deserialize_with = "lgpd")]
    pub lgpd: bool,
}

impl From<PrimeiroAcesso> for RegisterUser {
    fn from(value: PrimeiroAcesso) -> Self {
        RegisterUser {
            nome: value.nome_responsavel,
            instituicao: value.nome_instituicao,
            email: value.email,
            telefone: value.telefone,
            password: value.password,
        }
    }
}

#[derive(FromRow, Debug, Deserialize, Serialize, Default, Clone)]
pub struct Tenant {
    pub id: String,
    pub cnpj: String,
    pub email: String,
    pub status: String,
    pub razao_social: String,
    pub contato: String,
    pub telefone: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Permissoes {
    pub id_perfil_usuario: String, //id do perfil
    pub nome: Option<String>, //nome do perfil
    pub permissoes: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct ResetPassword {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserId {
    pub id: String
}

// Estrutura de User, ServiceError, etc.

pub struct SessionParser {
    pub id_usuario: String,
    pub id_empresa: String,
}

impl From<Session> for SessionParser {
    fn from(value: Session) -> Self {
        SessionParser {
            id_usuario: value.get::<String>("user_id").unwrap().expect("usuário não fez login"),
            id_empresa: value.get::<String>("empresa_id").unwrap().expect("nenhuma empresa localizada"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "user_permission", rename_all = "lowercase")]
pub enum UserPermission {
    Produto,
    Empresa,
    Usuario,
    Pedido,
    Contato,
    Compra,
    Financeiro,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "user_operation", rename_all = "lowercase")]
pub enum UserOperation {
    Edit,
    View,
    Delete,
    
}

pub struct Permission {
    pub permission: UserPermission,
    pub operation: UserOperation,
}

//SELECT role as "role: Role" FROM users WHERE email = $1 AND password = $2