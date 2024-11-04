// use std::collections::HashMap; //deprecated
use actix_web::{get, post, web, HttpResponse, Responder};
use log::info;
use minijinja::context;
use reqwest::StatusCode;
use crate::app::AppState;
use crate::auth::model::Registrar;
use crate::infra::error::Error;
use crate::services::redireciona_login;
use crate::services::usuario as service;

#[get("/registrar")]
async fn register(
    // session: Session, 
    data: web::Data<AppState>,
    
    ) -> impl Responder {
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
    let usuario_registrado = service::registrar_usuario(pool, &register_form, "USER").await;

    if let Some(_usuario) = usuario_registrado {
        redireciona_login()
    } else {
        Error::Detailed { code: StatusCode::INTERNAL_SERVER_ERROR, 
            msg: "Erro interno do servidor".to_owned(), 
            description: "Houve uma falha ao criar o usuário".to_owned(), 
            how_to_solve: format!(r#"Envie este relatório para o suporte \n {:?}"#, register_form)}.into()
    }

    
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register)
        .service(register_submit);
}