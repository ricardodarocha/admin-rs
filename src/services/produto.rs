use log::{error, info};
use sqlx::{Pool, Sqlite};
use crate::models::produto::Produto;
use crate::repository::produto as repo;

pub async fn abrir_produto(pool: &Pool<Sqlite>, id: String) -> Option<Produto> {
    let produto = repo::abrir_produto(pool, &id).await;

    match produto {
        Ok(value) => {
            info!("📦 Produto {}", id);
            Some(value)
        }
        Err(err) => {
            error!("📦 {}", err);
            None
        }
    }
}

pub async fn inserir_produto_json(pool: &Pool<Sqlite>, form: crate::models::pedido::ProdutoNovo) -> Option<Produto> {
    let id_produto = repo::inserir_produto_json(pool, form).await;
    match id_produto {
        Ok(id) => {
            abrir_produto(pool, id).await
        }
        Err(err) => {
            error!("❌ {}", err);
            None
        }
    }
    
}
