use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
// use std::fmt;
use std::error::Error as StdError;

pub type Result<T, E = Error> = std::result::Result<T, E>;
use thiserror::Error;

pub struct Detailed {

}

#[derive(Error, Debug)]
pub enum Error {
    
    #[error("Erro do servidor `{0}`")]
    ActixWeb(actix_web::Error),
    
    #[error("Erro do banco de dados `{0}`")]
    Sqlx(sqlx::Error),
    
    #[error("Procedimento inválido `{0}`")]
    Other(Box<dyn StdError>),
    
    #[error("code={code:?};msg={msg:?};description={description:?};solution={how_to_solve:?}")]
    Detailed {
        code: String,
        msg: String,
        description: String,
        how_to_solve: String,
    },

    #[error("Procedimento inválido `{0}`")]
    Simple(String),
    
    #[error("Procedimento inválido `{0}`")]
    Str(&'static str),
}

// impl fmt::Display for Error {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Error::ActixWeb(err) => write!(f, "ActixWeb error: {}", err),
//             Error::Sqlx(err) => write!(f, "SQLx error: {}", err),
//             Error::Other(err) => write!(f, "Erro personalizado: {}", err),
//             Error::Simple(msg) => write!(f, "Erro: {}", msg),
//             Error::Str(msg) => write!(f, "Erro: {}", msg),
//         }
//     }
// }

// impl StdError for Error {
//     fn source(&self) -> Option<&(dyn StdError + 'static)> {
//         match self {
//             Error::ActixWeb(err) => Some(err),
//             Error::Sqlx(err) => Some(err),
//             Error::Other(err) => Some(&**err),
//             Error::Simple(msg) => None,
//             Error::Str(msg) => None,
//         }
//     }
// }

impl From<actix_web::Error> for Error {
    fn from(err: actix_web::Error) -> Self {
        Error::ActixWeb(err)
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::Sqlx(err)
    }
}

impl From<Box<dyn StdError>> for Error {
    fn from(error: Box<dyn StdError>) -> Error {
        Error::Other(error)
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::ActixWeb(err) => HttpResponse::InternalServerError().body(format!("Internal server error {}", err)),
            Error::Sqlx(err) => HttpResponse::InternalServerError().body(format!("Internal server error {}", err)),
            Error::Other(err) => HttpResponse::InternalServerError().body(format!("Internal server error {}", err)),
            Error::Simple(err) => HttpResponse::InternalServerError().body(format!("Internal server error {}", err)),
            Error::Str(str) => HttpResponse::InternalServerError().body(format!("Internal server error {}", str)),
            Error::Detailed {
                code,
                msg,
                description,
                how_to_solve,
            } => {
                let body = json!({
                    "code": code,
                    "msg": msg,
                    "description": description,
                    "how_to_solve": how_to_solve,
                });
                HttpResponse::InternalServerError().json(body)}
                ,
        }
    }
}

