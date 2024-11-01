pub mod cliente;
pub mod produto;
pub mod pedido;

use log::{error, info};
use sqlx::SqlitePool;

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
