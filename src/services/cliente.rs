
use log::{error, info};
use sqlx::{Pool, Sqlite};
use crate::models::cliente::{Cliente, ClienteNovo, FormCliente};
use crate::repository::cliente as repo;

pub async fn inserir_cliente_json(pool: &Pool<Sqlite>, form: ClienteNovo) -> Option<Cliente> {

    let id_cliente = repo::inserir_cliente_json(pool, form).await.unwrap_or("-1".to_owned());
    abrir_cliente(pool, &id_cliente).await

}

pub async fn inserir_cliente_form(pool: &Pool<Sqlite>, form: FormCliente) -> Option<Cliente> {

    let id_cliente = repo::inserir_cliente(pool, form).await;

    match id_cliente {
        Ok(id) => {
            info!("Cliente inserido {}", id);
            abrir_cliente(pool, &id).await
        }
        Err(err) => {
            error!("❌{}", err);
            None
        }
    }

}

pub async fn atualizar_cliente(pool: &Pool<Sqlite>, id: String, form: FormCliente) -> Option<Cliente> {

    let id_cliente = repo::atualizar_cliente(pool, &id, form).await;

    match id_cliente {
        Ok(id) => {
            info!("Cliente atualizado {}", id);
            abrir_cliente(pool, &id).await
        }
        Err(err) => {
            error!("❌ {}", err);
            None
        }
    }
}

pub async fn abrir_cliente(pool: &Pool<Sqlite>, id: &str) -> Option<Cliente> {
    match repo::abrir_cliente(pool, &id.to_string()).await {
        Ok(cliente) => Some(cliente),
        Err(err) => {
            error!("❌ Erro ao abrir cliente {}", err);
            None
        }
    }
}

pub async fn inserir_ou_alterar_cliente(pool: &Pool<Sqlite>, id: String, form: FormCliente) -> Option<Cliente> {
    match id.as_ref() {
        "0" => inserir_cliente_form(pool, form).await,
        id => atualizar_cliente (pool, id.to_string(), form).await,
    } 
}