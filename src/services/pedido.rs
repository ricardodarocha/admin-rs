use log::{error, info};
// use minijinja::context;
use sqlx::{Pool, Sqlite};
use crate::models::pedido::{PedidoModel, PostItem};

use crate::models::QueryFiltroPedido;
use crate::repository::pedido as repo;

pub async fn abrir_pedido(pool: &Pool<Sqlite>, id: i64) -> Option<PedidoModel> {
    let pedido = repo::abrir_pedido(pool, id).await;

    match pedido {
        Ok(value) => {
            info!("📋 pedido localizado {}", id);
            Some(value)
        }
        Err(err) => {
            error!("👩‍🚒 {}", err);
            None
        }
    }
}

pub async fn inserir_pedido(pool: &Pool<Sqlite>, cliente: String) -> Option<PedidoModel> {
    let pedido_inserido = repo::inserir_pedido(pool, &cliente).await;
    
    match pedido_inserido {
        Ok(novo_id) => { 
            
            let pedido = repo::abrir_pedido(pool, novo_id).await;
            match pedido {
                Ok(value) => {
                info!("➕ pedido inserido com sucesso {}", novo_id);
                Some(value)
                },
            Err(err) => {
                error!("♨ Erro ao inserir pedido{}", err);
                None
                }  

            }
        },
        Err(err) => {
            error!("♨ Erro ao inserir pedido{}", err);
            None
        }
    }
    
}

pub async fn inserir_item(pool: &Pool<Sqlite>, pedido: i64, item: PostItem) -> Option<PedidoModel> {
    let item_inserido = repo::inserir_item_pedido(pool, pedido, &item).await;
    
    match item_inserido {
        Ok(item_inserido) if item_inserido == true => { 
            
            let pedido = repo::abrir_pedido(pool, pedido).await;
            match pedido {
                Ok(pedido) => {
                info!("➕ item inserido)");
                println!("________________________________________");
                
                for item in pedido.clone().itens.into_iter() {
                    println!("{:>30} ..... {}", item.produto.descricao, item.quant)
                }

                return Some(pedido);
                },
            Err(err) => {
                error!("♨ Erro ao inserir pedido{}", err);
                return None;
                }  

            }
        },
        Ok(_) => { 
            
                error!("♨ Erro ao inserir item. Pedido não encontrado [{:?}]", pedido);
                return None;
            },
        Err(err) => {
            error!("♨ Erro ao inserir pedido{}", err);
            None
        }
    }
}


pub async fn abrir_lista_pedidos(pool: &Pool<Sqlite>, cliente: &String, filtro: &QueryFiltroPedido) -> Vec<PedidoModel> {
    let pedido = repo::abrir_lista_pedidos(pool, &cliente, &filtro).await;

    let pagina = filtro.page;
    let ini = (pagina-1) * filtro.size;
    let fim = (pagina) * filtro.size;

    match pedido {
        Ok(value) => {
            info!("📋 pedidos listados ");
            info!("🙎‍♂️ cliente {}", cliente);
            info!("🗃 página {}, {}..{} ", pagina, ini, fim);
            value
        }
        Err(err) => {
            error!("👩‍🚒 erro ao listar pedidos {}", err);
            vec!()
        }
    }
}