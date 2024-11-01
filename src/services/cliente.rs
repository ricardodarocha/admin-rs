use actix_web::{get, post, web, HttpResponse, Responder};
use log::info;
use minijinja::context;
use crate::models::cliente::FormCliente;
use crate::models as model;
use crate::app::AppState;
use crate::services::abrir_cliente;


#[get("/cliente/edit")]
async fn web_cliente(
        data: web::Data<AppState>,
        query: web::Query<model::QueryId>, 

    ) -> impl Responder {
        
    let pool = &data.database;
    let tmpl = data.render.get_template("web/cliente.html").unwrap();
    let web::Query(entidade_cliente) = query;
    let cliente = abrir_cliente(pool, entidade_cliente.id).await;

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

#[post("/cliente/edit")]
async fn web_cliente_submit(
    form: web::Form<FormCliente>,
    data: web::Data<AppState>,

) -> impl Responder {
    
    info!("Recebido POST com dados: {:?}", form);

    let tmpl = data.render.get_template("shared/views/ajaxToast.html").unwrap();
    let rendered = tmpl.render(context! {
        toast_icon => "bi-check-circle",
        toast_class => "toast-success",
        toast_text => "Dados salvos com sucesso!",
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