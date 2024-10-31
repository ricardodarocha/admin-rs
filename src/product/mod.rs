use std::collections::HashMap;

use actix_web::{get, post, web, HttpResponse, Responder};
use log::info;
use minijinja::context;

use crate::app::AppState;
use crate::models::QueryId;
// use crate::repository as repo;
// use crate::models as model;
use crate::services as service;
use serde_json::json;

#[get("/product/form")]
async fn web_product(
        data: web::Data<AppState>,
        id: web::Query<QueryId>,
        ) -> impl Responder {
    let pool = &data.database;
    let tmpl = data.render.get_template("web/product.html").unwrap();
    let web::Query(entidade_produto) = id;
    let cadastro = service::abrir_produto(pool, entidade_produto.id).await;

    //Se encontrou o produto, renderiza o formulario, senão, dá um erro
    if let Some(produto) = cadastro {
        let rendered = tmpl.render(context! {title => "Cadastro de produto", produto}).unwrap();

        HttpResponse::Ok()
            .content_type("text/html")
            .body(rendered)
    } else {
        HttpResponse::Ok()
            .content_type("application/json")
            .json(json!(
                {
                    "erro": ""
                }
            ))
    }
    
}

#[post("/registrar")]
async fn web_register_submit(
    form: web::Form<HashMap<String, String>>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("Recebido POST com dados: {:?}", form);

    let tmpl = data.render.get_template("shared/views/ajaxToast.html").unwrap();
    let rendered = tmpl.render(context! {
        toast_icon => "bi-check-circle",
        toast_class => "toast-success",
        toast_text => "Mensagem enviada com sucesso!",
    }).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)

}