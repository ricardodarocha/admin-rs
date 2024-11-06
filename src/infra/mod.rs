pub mod sessao_usuario;
pub mod jwt;
pub mod toast;
pub mod strings;
pub mod repo;
pub mod email;
pub mod pagination;
pub mod psw;
pub mod render;
pub mod result;
pub mod error;
pub mod uuid;
pub mod models;

pub mod controller {
    use actix_web::{get, HttpResponse, Responder};

    #[get("/api/ping")]
    pub async fn ping() -> impl Responder {
    const MESSAGE: &str = "Running";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
    }

}

pub mod minijinja_utils;
