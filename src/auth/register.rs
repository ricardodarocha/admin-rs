// use std::collections::HashMap; //deprecated
use actix_web::{get, post, web, HttpResponse, Responder};
use log::{info, error, warn};
use minijinja::context;
use crate::app::AppState;
use crate::auth::model::Registrar;
use crate::infra::error::Error;
use crate::infra::validation::export_validations;
use crate::services::redireciona_login;
use crate::services::usuario as service;
use validator::Validate;
// use actix_web_validator::Form;

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
    let register_form = form.into_inner();

    if let Err(errors) = register_form.validate() {
        warn!("Gerando resultado");
        let toast = export_validations(&errors, "Preencha os campos corretamente");
        let erro = Error::Form(toast);
        error!("{:?}", erro);
        return erro.into();
    }

    let pool = &data.database;
    let usuario_registrado = service::registrar_usuario(pool, &register_form, "USER").await;

    if let Some(_usuario) = usuario_registrado {
        redireciona_login()
    } else {
       HttpResponse::InternalServerError()
        .body("Erro ao registrar usuário") 
    }
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register)
        .service(register_submit);
}