use actix_session::Session;
use actix_web::web::Path;
use actix_web::{get, post, web, HttpResponse, Responder};
use log::info;
use crate::app::AppState;
use crate::models::QueryFiltroCliente;
use minijinja::context;
use crate::models::cliente::FormCliente;
use crate::services::cliente as service;
use crate::repository::cliente as repo;

// #[get("/clientes/lista")]
// pub async fn web_listar_clientes(
//     data: web::Data<AppState>,
//     query: web::Query<QueryFiltroCliente>,
// ) -> impl Responder {
//     let pool = &data.database;
//     let tmpl = data.render.get_template("web/listaClientes.html").unwrap();

//     // Obtenção dos clientes filtrados
//     let clientes = listar_clientes(pool, query).await;

//     if let Some(clientes) = clientes {
//         let rendered = tmpl.render(context! { title => "Lista de Clientes", clientes }).unwrap();
        
//         HttpResponse::Ok()
//             .content_type("text/html")
//             .body(rendered)
//     } else {
//         let error_tmpl = data.render.get_template("shared/views/ajaxToast.html").unwrap();
//         let rendered = error_tmpl.render(context! {
//             toast_icon => "bi-exclamation-circle",
//             toast_class => "toast-error",
//             toast_text => "Nenhum cliente encontrado!",
//         }).unwrap();

//         HttpResponse::Ok()
//             .content_type("text/html")
//             .body(rendered)
//     }
// }

#[get("/cliente/edit/{id}")]
pub async fn web_cliente(
        data: web::Data<AppState>,
        path: Path<String>,
        _session: Session,

    ) -> impl Responder {
        
    let id = path.into_inner();
    let pool = &data.database;
    let tmpl = data.render.get_template("site/cliente.html").unwrap();
    let cliente = service::abrir_cliente(pool, &id).await;

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
pub async fn web_cliente_submit(
    data: web::Data<AppState>,
    form: web::Form<FormCliente>,
    path: Path<String>,

    ) -> impl Responder {
        
    let id = path.into_inner();
    let pool = &data.database;    
    info!("Recebido POST com dados: {:?}", form.clone());
    let web::Form(form) = form;
    let _cliente = service::inserir_ou_alterar_cliente(pool, id, form).await;
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

#[get("/cliente/json/{id}")]
async fn json_cliente(
        data: web::Data<AppState>,
        path: Path<String>,

    ) -> impl Responder {
        
    let id = path.into_inner();
    let pool = &data.database;
    
    let cliente = service::abrir_cliente(pool, &id).await;

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
    
    let cliente = repo::abrir_lista_clientes(pool, &query).await;

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
