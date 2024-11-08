
use actix_web::{HttpResponse, Responder};
use serde::Serialize;
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TipoToast {
    Success,
    Info,
    Warn,
    Error   
}

#[derive(Serialize)]
pub struct Toast {
    #[serde(rename="type")]
    pub tipo: TipoToast,
    pub icon: String,
    pub text: String,

}

use crate::infra::error::Error;

impl From<Error> for Toast {
    fn from(value: Error) -> Self {

        let text = format!("{:?}", value); 

        Toast {
            tipo: TipoToast::Error,
            icon: "bi-x-circle-fill".to_owned(),
            text,
        }
    }
}

impl Toast {
    
    pub fn created(mensagem: &str) -> impl Responder {

        let toast = Toast {
            tipo: TipoToast::Success,
            icon: "bi-check-circle-fill".to_owned(),
            text: mensagem.to_owned(),
        };

       HttpResponse::Created()
        .content_type("application/json")
        .json(toast) 
    }

    pub fn accepted(mensagem: &str) -> impl Responder {

        let toast = Toast {
            tipo: TipoToast::Success,
            icon: "bi-check-circle-fill".to_owned(),
            text: mensagem.to_owned(),
        };

       HttpResponse::Accepted()
        .content_type("application/json")
        .json(toast) 
    }

    
}