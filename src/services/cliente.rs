use actix_web::web::Path;
use actix_web::{get, post, web, HttpResponse, Responder};
use log::{error, info};
use minijinja::context;
use sqlx::{Pool, Sqlite};
use crate::models::cliente::{Cliente, FormCliente};

use crate::app::AppState;
use crate::models::QueryFiltroCliente;
use crate::repository as repo;
use crate::services::abrir_cliente;


#[get("/cliente/edit/{id}")]
async fn web_cliente(
        data: web::Data<AppState>,
        path: Path<String>,

    ) -> impl Responder {
        
    let id = path.into_inner();
    let pool = &data.database;
    let tmpl = data.render.get_template("web/cliente.html").unwrap();
    let cliente = abrir_cliente(pool, id).await;

    if let Some(cliente) = cliente {
        let rendered = tmpl.render(context! {title => "Cliente", cliente}).unwrap();

        HttpResponse::Ok()
            .content_type("text/html")
            .body(rendered)
    } 
    else {     
        
        let tmpl = data.render.get_template("shared/views/ajaxToast.html").unwrap();
        let rendered = tmpl.render(context! {
            toast_icon => "bi-check-circle",
            toast_class => "toast-error",
            toast_text => "Cliente não encontrado!",
    }).unwrap();
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)

    }
    
}

#[post("/cliente/edit/{id}")]
async fn web_cliente_submit(
    data: web::Data<AppState>,
    form: web::Form<FormCliente>,
    path: Path<String>,

    ) -> impl Responder {
        
    let id = path.into_inner();
    let pool = &data.database;    
    info!("Recebido POST com dados: {:?}", form.clone());
    let web::Form(form) = form;
    let _cliente = inserir_ou_alterar_cliente(pool, id, form).await;
    let tmpl = data.render.get_template("shared/views/ajaxToast.html").unwrap();
    let rendered = tmpl.render(context! {
        toast_icon => "bi-check-circle",
        toast_class => "toast-success",
        toast_text => "Dados salvos com sucesso! ",
    }).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)

    // HttpResponse::Ok()
    //     .content_type("application/json")
    //     .json(json!({
    //         "toast": "teste"
    //     }))
}

async fn inserir_ou_alterar_cliente(pool: &Pool<Sqlite>, id: String, form: FormCliente) -> Option<Cliente> {
    match id.as_ref() {
        "0" => inserir_cliente(pool, form).await,
        id => atualizar_cliente (pool, id.to_string(), form).await,
    } 
}

#[get("/cliente/json/{id}")]
async fn json_cliente(
        data: web::Data<AppState>,
        path: Path<String>,

    ) -> impl Responder {
        
    let id = path.into_inner();
    let pool = &data.database;
    
    let cliente = abrir_cliente(pool, id).await;

    if let Some(cliente) = cliente {

        HttpResponse::Ok()
            .content_type("application/json")
            .json(cliente)
    } 
    else {     
        
    HttpResponse::NotFound()
        .finish()

    }
    
}

#[get("/clientes/json")]
async fn json_all_cliente(
        data: web::Data<AppState>,
        // path: Path<String>,
        query: web::Query<QueryFiltroCliente>

    ) -> impl Responder {
        
    // let id = path.into_inner();
    let pool = &data.database;
    let web::Query(query) = query;
    
    let cliente = repo::cliente::abrir_lista_clientes(pool, &query).await;

    if let Ok(cliente) = cliente {

        HttpResponse::Ok()
            .content_type("application/json")
            .json(cliente)
    } 
    else {     
        
    HttpResponse::NotFound()
        .finish()

    }
    
}


pub async fn inserir_cliente(pool: &Pool<Sqlite>, form: FormCliente) -> Option<Cliente> {

    let cliente = repo::inserir_cliente(pool, form).await;

    match cliente {
        Ok(value) => {
            info!("Cliente inserido {}", value.id);
            Some(value)
        }
        Err(err) => {
            error!("❌{}", err);
            None
        }
    }

}

pub async fn atualizar_cliente(pool: &Pool<Sqlite>, id: String, form: FormCliente) -> Option<Cliente> {

    let cliente = repo::atualizar_cliente(pool, &id, form).await;

    match cliente {
        Ok(value) => {
            info!("Cliente atualizado {}", id);
            Some(value)
        }
        Err(err) => {
            error!("❌ {}", err);
            None
        }
    }
}