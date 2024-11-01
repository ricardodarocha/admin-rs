use actix_web::web::Path;
use actix_web::{get, web, HttpResponse, Responder};
use log::{error, info};
// use minijinja::context;
use sqlx::{Pool, Sqlite};
use crate::models::pedido::{PedidoModel};

use crate::app::AppState;
use crate::models::QueryFiltroPedido;
use crate::repository::pedido as repo;

#[get("/pedido/json/{id}")]
async fn json_pedido(
        data: web::Data<AppState>,
        path: Path<i64>,

    ) -> impl Responder {
        
    let id = path.into_inner();
    let pool = &data.database;
    
    let pedido = abrir_pedido(pool, id).await;

    if let Some(pedido) = pedido {

        HttpResponse::Ok()
            .content_type("application/json")
            .json(pedido)
    } 
    else {     
        
    HttpResponse::NotFound()
        .finish()

    }
    
}

async fn abrir_pedido(pool: &Pool<Sqlite>, id: i64) -> Option<PedidoModel> {
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

async fn abrir_lista_pedidos(pool: &Pool<Sqlite>, cliente: &String, filtro: &QueryFiltroPedido) -> Vec<PedidoModel> {
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

/// exemplo http://localhost:8080/pedidos/json?cliente=00008756486
#[get("/pedidos/json")]
async fn json_all_pedido(
        data: web::Data<AppState>,
        // path: Path<String>,
        query: web::Query<QueryFiltroPedido>

    ) -> impl Responder {
        
    // let cliente = path.into_inner();
    let pool = &data.database;
    let web::Query(query) = query;
    let cliente = &query.cliente;
    
    let pedido = abrir_lista_pedidos(pool, cliente, &query).await;

    HttpResponse::Ok()
            .content_type("application/json")
            .json(pedido)
    } 
    
