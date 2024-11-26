#[derive(serde::Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

use std::time::Duration;

use serde_json::json;
use crate::app::AppState;
use crate::services::usuario as service;
use actix_web::{post, web, cookie::Cookie, Responder, HttpResponse};
use log::info;
use crate::services::login::token;

#[post("/token")]
async fn generate_token(
    data: web::Data<AppState>,
    login: web::Json<LoginRequest>,
) -> impl Responder {
    const HORAS: u64 = 24;
    const MINUTOS: u64 = 60;
    const SEGUNDOS: u64 = 60;

    let pool = &data.database;
    let login_inspector = service::login(pool, &login.username, &login.password).await;

    if let Some(valid_login) = login_inspector {
        if valid_login {
            info!("🙎‍♂️ Acesso concedido ✔ ");

            let dias = HORAS * MINUTOS * SEGUNDOS;
            let token = token(&login.username, Duration::from_secs(15 * dias))
                .expect("Erro ao desempacotar o token");

            let cookie = Cookie::build("AUTHORIZATION", format!("Bearer {}", token))
                .http_only(true) // Evita acesso via JavaScript
                // .secure(true)    // Requer HTTPS
                .path("/")       // Disponível em toda a aplicação
                .finish();

            HttpResponse::Ok()
                .content_type("application/json")
                .cookie(cookie)
                .json(json!({
                    "token": token,
                    "message": "Token gerado com sucesso"
                }))
        } else {
            info!("🙇‍♂️ Acesso negado ❌ ");

            HttpResponse::Unauthorized()
                .content_type("application/json")
                .json(json!({
                    "form": {
                        "email": "",
                        "password": ""
                    },
                    "message": "Acesso inválido"
                }))
        }
    } else {
        HttpResponse::InternalServerError()
            .content_type("application/json")
            .json(json!({
                "message": "Erro ao verificar login"
            }))
    }
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(generate_token);
}
