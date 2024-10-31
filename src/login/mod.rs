use std::collections::HashMap;

use actix_web::{get, post, web, HttpResponse, Responder};
use log::info;
use minijinja::context;

use crate::app::AppState;

#[get("/entrar")]
async fn web_login(data: web::Data<AppState>) -> impl Responder {
    let tmpl = data.render.get_template("web/login.html").unwrap();
    let rendered = tmpl.render(context! {title => "Login"}).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

#[post("/entrar")]
async fn web_login_submit(
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

    // HttpResponse::Ok()
    //     .content_type("application/json")
    //     .json(json!({
    //         "toast": "teste"
    //     }))
}

#[get("/registrar")]
async fn web_register(data: web::Data<AppState>) -> impl Responder {
    let tmpl = data.render.get_template("web/register.html").unwrap();
    let rendered = tmpl.render(context! {title => "Register"}).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
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