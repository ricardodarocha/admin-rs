
use log::{error, info};
use sqlx::{Pool, Sqlite, SqlitePool};

use crate::auth::model::{Registrar, Usuario};
use crate::infra::strings::anonimizar;
use crate::repository::usuario as repo;

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