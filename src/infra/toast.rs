
use actix_web::HttpResponse;
use serde::Serialize;
use serde_json::Value;
use actix_web::http::StatusCode;
use crate::infra::error::Error;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TipoToast {
    Success,
    Info,
    Warn,
    Error   
}

#[derive(Debug, Clone, Serialize)]
pub struct Toast {
    #[serde(rename="type")]
    pub tipo: TipoToast,
    pub icon: String,
    pub text: String,

}

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
    
    pub fn created(mensagem: &str) -> ApiResponse {

        let toast = Toast {
            tipo: TipoToast::Success,
            icon: "bi-check-circle-fill".to_owned(),
            text: mensagem.to_owned()
        };

       ApiResponse::new()
        .with_toast(toast)
    }

    pub fn accepted(mensagem: &str) -> ApiResponse {

        let toast = Toast {
            tipo: TipoToast::Success,
            icon: "bi-check-circle-fill".to_owned(),
            text: mensagem.to_owned()
        };

       ApiResponse::new()
        .with_toast(toast)
    }    
    
    pub fn with_status(status_code: StatusCode, mensagem: &str) -> Self {
        Toast {
            tipo: if status_code.is_success() { TipoToast::Success } else { TipoToast::Error },
            icon: if status_code.is_success() { "bi-check-circle-fill".to_owned() } else { "bi-exclamation-triangle-fill".to_owned() },
            text: mensagem.to_owned(),
        }
    }
}

// Estrutura ApiResponse para padronizar o retorno da API
#[derive(Serialize)]
pub struct ApiResponse {
    pub toast: Option<Toast>,
    pub form: Option<Value>,
    pub data: Option<Value>,
    #[serde(skip_serializing)]
    pub status_code: StatusCode,
    pub redirect: Option<&'static str>,
}

impl ApiResponse {
    pub fn new() -> Self {
        Self {
            toast: None,
            form: None,
            data: None,
            status_code: StatusCode::OK, // Status padrão
            redirect: None,
        }
    }

    pub fn with_status(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }

    pub fn with_toast(mut self, toast: Toast) -> Self {
        self.toast = Some(toast);
        self
    }

    pub fn with_form(mut self, form: Value) -> Self {
        self.form = Some(form);
        self
    }

    pub fn with_data(mut self, data: Value) -> Self {
        self.data = Some(data);
        self
    }

    pub fn with_redirect(mut self, data: &'static str) -> Self {
        self.redirect = Some(data);
        self
    }

    pub fn send(self) -> HttpResponse {
        // Constrói o `ApiResponse` final e converte em JSON
        let api_response = ApiResponse {
            status_code: self.status_code,
            toast: self.toast,
            form: self.form,
            data: self.data,
            redirect: self.redirect,
        };

        HttpResponse::build(api_response.status_code)
            .content_type("application/json")
            .json(api_response)
    }

    pub fn render(self, rendered: String) -> HttpResponse {
        // Constrói o `ApiResponse` final e converte em HTML
        let api_response = ApiResponse {
            status_code: self.status_code,
            toast: self.toast,
            form: self.form,
            data: self.data,
            redirect: self.redirect,
        };

        HttpResponse::build(api_response.status_code)
            .content_type("text/html")
            .body(rendered)
    }

    pub fn created(mensagem: &str, value: Value) -> Self {
        let toast = Toast {
            tipo: TipoToast::Success,
            icon: "bi-check-circle-fill".to_owned(),
            text: mensagem.to_owned(),
        };

        ApiResponse {
            toast: Some(toast),
            form: None,
            data: Some(value),
            status_code: StatusCode::CREATED,
            redirect: None,
        }
    }

    pub fn from_error(err: Error) -> Self {
        let toast = Toast::from(err);
        ApiResponse::new().with_toast(toast) 
    }
}