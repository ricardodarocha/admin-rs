
use log::{error, info};
use sqlx::{Pool, Sqlite};
use crate::models::cliente::{Cliente, ClienteNovo, FormCliente};
use crate::models::QueryFiltro;
use crate::repository;
use crate::repository::api::clientes::sqlite as api;
use crate::repository::admin::clientes::sqlite as admin;
use actix_web::web;

pub async fn inserir_cliente_json(pool: &Pool<Sqlite>, form: web::Json<ClienteNovo>) -> Option<Cliente> {

    let id_cliente = api::inserir_cliente_json(pool, form.into_inner()).await.unwrap_or("-1".to_owned());
    abrir_cliente(pool, &id_cliente).await

}

pub async fn inserir_cliente_form(pool: &Pool<Sqlite>, form: web::Form<FormCliente>) -> Option<Cliente> {

    let id_cliente = admin::inserir_cliente_form(pool, form.into_inner()).await;

    match id_cliente {
        Ok(id) => {
            abrir_cliente(pool, &id).await
        }
        Err(err) => {
            error!("❌ {}", err);
            None
        }
    }

}

pub async fn atualizar_cliente(pool: &Pool<Sqlite>, id: String, form: FormCliente) -> Option<String> {

    let cliente = repository::admin::clientes::sqlite::atualizar_cliente(pool, &id, form).await;

    match cliente {
        Ok(id) => {
            info!("Cliente atualizado {}", id);
            Some(id)
        }
        Err(err) => {
            error!("❌ {}", err);
            None
        }
    }
}

pub async fn abrir_cliente(pool: &Pool<Sqlite>, id: &str) -> Option<Cliente> {
    match repository::admin::clientes::sqlite::abrir_cliente(pool, &id.to_string()).await {
        Ok(cliente) => {
            info!("🙋‍♂️ cliente {}", id);
            Some(cliente)},
        Err(err) => {
            error!("🤷‍♂️ Erro ao abrir cliente. {}", err);
            None
        }
    }
}

pub async fn inserir_ou_alterar_cliente(pool: &Pool<Sqlite>, id: String, form: FormCliente) -> Option<Cliente> {
    let id = 
    match id.as_ref() {
        "0" => repository::admin::clientes::sqlite::inserir_cliente_form(pool, form).await,
        id => Ok(atualizar_cliente (pool, id.to_string(), form).await.unwrap()),
    };

    if let Ok(id) = id {
        abrir_cliente(pool, &id).await
    } else {None}
    
}

pub async fn abrir_lista_clientes(pool: &Pool<Sqlite>, filtro: &QueryFiltro) -> Vec<Cliente> {
     
    let lista = repository::api::clientes::sqlite::abrir_lista_clientes(pool, filtro).await;
    match lista {
        Ok(value) => {
            info!("👥👤 {} clientes... ", value.len());
            value
        },
        Err(err) => {
            error!("👨‍🚒 erro ao carregar lista de clientes. {}", err);
            vec!()
        }
    }
}