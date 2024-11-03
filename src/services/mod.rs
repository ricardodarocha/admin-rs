pub mod relatorio;
pub mod grafico;
pub mod cliente;
pub mod produto;
pub mod pedido;
pub mod login;

use actix_web::HttpResponse;
use log::{error, info};
use serde_json::json;
use sqlx::{Pool, Sqlite, SqlitePool};

use crate::auth::model::{Registrar, Usuario};
use crate::infra::strings::anonimizar;
use crate::models::produto::Produto;
use crate::models::cliente::Cliente;
// use crate::models::pedido::PedidoModel;
// use crate::models::pedido::ItemModel;
use crate::repository as repo;

pub async fn abrir_produto(pool: &SqlitePool, id: String) -> Option<Produto> {
    let produto = repo::abrir_produto(pool, &id).await;

    match produto {
        Ok(value) => {
            info!("📦 Produto localizado {}", id);
            Some(value)
        }
        Err(err) => {
            error!("📦 {}", err);
            None
        }
    }
}

pub async fn abrir_cliente(pool: &SqlitePool, id: String) -> Option<Cliente> {
    let produto = repo::abrir_cliente(pool, &id).await;

    match produto {
        Ok(value) => {
            info!("🧑 Cliente localizado {}", id);
            Some(value)
        }
        Err(err) => {
            error!("👩‍🚒 {}", err);
            None
        }
    }
}

pub async fn registrar_usuario(pool: &Pool<Sqlite>, register_form: &Registrar, level: &str) -> Option<Usuario> {
    let usuario = repo::registrar_usuario(pool, register_form.clone(), level).await;  
    match usuario {
        Ok(value) => {
            info!("🧑 Usuário inserido {}", anonimizar(value.login.as_ref()) );
            Some(value)
        }
        Err(err) => {
            error!("👩‍🚒 Erro ao inserir usuário {}", err);
            None
        }
    }
}

pub async fn login(pool: &Pool<Sqlite>, email: &String, senha: &String, ) -> Option<bool> {
    
    let result = repo::login(pool, email, senha).await;  
    match result {
        Ok(value) => {
            Some(value)
        }
        Err(err) => {
            error!("👩‍🚒 Erro na tentativa de login {}", err);
            None
        }
    }
}

pub async fn abrir_usuario(pool: &SqlitePool, email: String) -> Option<Usuario> {
    let usuario = repo::abrir_usuario(pool, &email.as_ref()).await;

    match usuario {
        Ok(value) => {
            info!("🧑 Usuário localizado {}", anonimizar(&email));
            Some(value)
        }
        Err(err) => {
            error!("👩‍🚒 {}", err);
            None
        }
    }
}

pub fn redireciona_login() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({
             "redirect": "/entrar"
         }))
}