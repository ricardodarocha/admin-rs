// use std::collections::HashMap; //deprecated
use actix_web::{get, post, web, HttpResponse, Responder};
use log::info;
use minijinja::context;
use crate::app::AppState;
use crate::auth::model::Registrar;
use crate::services as service;

#[get("/registrar")]
async fn register(data: web::Data<AppState>) -> impl Responder {
    let tmpl = data.render.get_template("auth/register.html").unwrap();
    let rendered = tmpl.render(context! {title => "Register"}).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

/// Registra o usuário USER
#[post("/registrar")]
async fn register_submit(
    form: web::Form<Registrar>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("Recebido POST /registrar com dados: {:?}", form);
    let web::Form(register_form) = form;
    let pool = &data.database;
    let usuario_registrado = service::registrar_usuario(pool, register_form, "USER").await;

    if let Some(_usuario) = usuario_registrado {
        let tmpl = data.render.get_template("components/ajaxToast.html").unwrap();
            let rendered = tmpl.render(context! {
                toast_icon => "bi-check-circle",
                toast_class => "toast-success",
                toast_text => "Mensagem enviada com sucesso!",
            }).unwrap();

            HttpResponse::Ok()
                .content_type("text/html")
                .body(rendered)
    } else {
        let tmpl = data.render.get_template("components/ajaxToast.html").unwrap();
            let rendered = tmpl.render(context! {
                toast_icon => "bi-x-circle-fill",
                toast_class => "toast-error",
                toast_text => "Erro ao registrar o usuário!",
            }).unwrap();

            HttpResponse::Ok()
                .content_type("text/html")
                .body(rendered)
    }

    
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register)
        .service(register_submit);
}