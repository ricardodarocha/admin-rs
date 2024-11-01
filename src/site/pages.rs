use std::collections::HashMap;
use actix_web::{get, post, web, HttpResponse, Responder};
use log::info;
use minijinja::{context};
use crate::app::AppState;

#[get("/")]
async fn home(data: web::Data<AppState>) -> impl Responder {
    let tmpl = data.render.get_template("site/home.html").unwrap();
    let rendered = tmpl.render(context! {title => "Página Inicial"}).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

#[get("/sobre")]
async fn about(data: web::Data<AppState>) -> impl Responder {
    let tmpl = data.render.get_template("site/about.html").unwrap();
    let rendered = tmpl.render(context! {title => "Sobre"}).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

#[get("/contato")]
async fn contact(data: web::Data<AppState>) -> impl Responder {
    let tmpl = data.render.get_template("site/contact.html").unwrap();
    let rendered = tmpl.render(context! {title => "Contato"}).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

#[post("/contato")]
async fn contact_submit(
    form: web::Form<HashMap<String, String>>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("Recebido POST com dados: {:?}", form);

    let tmpl = data.render.get_template("components/ajaxToast.html").unwrap();
    let rendered = tmpl.render(context! {
        toast_icon => "bi-check-circle",
        toast_class => "toast-success",
        toast_text => "Mensagem enviada com sucesso!",
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

#[get("/termos")]
async fn terms(data: web::Data<AppState>) -> impl Responder {
    let tmpl = data.render.get_template("site/terms.html").unwrap();
    let rendered = tmpl.render(context! {title => "Termos e Condições de Uso"}).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

#[get("/politica-de-privacidade")]
async fn policy(data: web::Data<AppState>) -> impl Responder {
    let tmpl = data.render.get_template("site/policy.html").unwrap();
    let rendered = tmpl.render(context! {title => "Política de Privacidade"}).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

#[get("/ops")]
async fn error(data: web::Data<AppState>) -> impl Responder {
    let tmpl = data.render.get_template("site/error.html").unwrap();
    let rendered = tmpl.render(context! {title => "Ops"}).unwrap();

    HttpResponse::NotFound()
        .content_type("text/html")
        .body(rendered)
}

pub fn routes(cfg: &mut web::ServiceConfig){
    cfg.service(home)
        .service(about)
        .service(contact)
        .service(contact_submit)
        .service(terms)
        .service(policy)
        .service(error);
}