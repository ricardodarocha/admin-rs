pub mod usuario;
pub mod relatorio;
pub mod grafico;
pub mod cliente;
pub mod produto;
pub mod pedido;
pub mod login;

use actix_web::HttpResponse;
use serde_json::json;




pub fn redireciona_login() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({
             "redirect": "/entrar"
         }))
}