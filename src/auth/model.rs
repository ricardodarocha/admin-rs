use actix_session::Session;
// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

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
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct PrimeiroAcesso {
    pub nome: String, //Nome da instituição
    pub cnpj: String,
    pub segmento: String,
    pub email: String,
    pub telefone: String,
    pub responsavel: String,
    pub cpf: String,
    pub password: Option<String>, //senha sem criptografia
}

impl From<PrimeiroAcesso> for RegisterUser {
    fn from(value: PrimeiroAcesso) -> Self {
        RegisterUser {
            nome: value.responsavel,
            instituicao: value.nome,
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