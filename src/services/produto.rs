use log::{error, info};
use sqlx::{Pool, Sqlite};
use crate::models::produto::Produto;
use crate::repository::produto as repo;

pub async fn abrir_produto(pool: &Pool<Sqlite>, id: String) -> Option<Produto> {
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

