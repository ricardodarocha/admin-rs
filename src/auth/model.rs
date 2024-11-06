// use actix_session::Session;
// use serde::de::Deserializer;
// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use time::serde::iso8601::option;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Serialize, FromRow, Clone)]
pub struct Usuario {
    pub login: String,
    pub nome: String,   
    pub nivel: String,
    //nunca retornar a senha do usuário
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub senha: String, 
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct Mensagem {
    pub codigo: i32,
    pub titulo: Option<String>,
    pub mensagem: Option<String>,
    pub url: Option<String>,
        
    #[serde(with = "time::serde::iso8601")]
    pub criado_em: OffsetDateTime,

    #[serde(with = "option")]
    pub lido_em: Option<OffsetDateTime>,

    pub usuario: String,
    
}
#[derive(FromRow, Serialize, Deserialize)]
pub struct Conversa {
    pub codigo: i32,
    pub mensagem: Option<String>,
        
    #[serde(with = "time::serde::iso8601")]
    pub criado_em: OffsetDateTime,

    #[serde(with = "option")]
    pub lido_em: Option<OffsetDateTime>,

    /// id do usuário que vai receber a mensagem
    pub usuario: String,

    /// id do usuario que enviou a mensagem
    pub remetente: String,
    
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, Validate)]
#[validate(schema(function = "validate_passwords", skip_on_field_errors = false))]
pub struct Registrar {
    pub nome: String,
    #[validate(email(message = "Email inválido"))]
    pub email: String,
    pub senha: String,
    pub repetir_senha: String,
    
}

fn validate_passwords(form: &Registrar) -> Result<(), ValidationError> {
    if form.senha != form.repetir_senha {
        let mut error = ValidationError::new("password");
        error.message = Some("As senhas não correspondem".into());
        return Err(error);
    }
    Ok(())
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct RecuperaSenha {
    pub usuario: String,
    pub email: String,
    pub nova_senha: String,
    pub repetir_senha: String,
}

    // #[serde(with = "time::serde::iso8601")]