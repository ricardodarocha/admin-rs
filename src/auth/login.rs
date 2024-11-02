use std::collections::HashMap;
use actix_web::{get, post, web, HttpResponse, Responder};
use log::info;
use minijinja::{context};
use serde_json::json;
use crate::app::AppState;

#[get("/entrar")]
async fn login(data: web::Data<AppState>) -> impl Responder {
    let tmpl = data.render.get_template("auth/login.html").unwrap();
    let rendered = tmpl.render(context! {title => "Login"}).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

#[post("/entrar")]
async fn login_submit(
    form: web::Form<HashMap<String, String>>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("Recebido POST com dados: {:?}", form);

    let tmpl = data.render.get_template("components/ajaxToast.html").unwrap();
    /*
    let rendered = tmpl.render(context! {
        toast_icon => "bi-check-circle",
        toast_class => "toast-success",
        toast_text => "Mensagem enviada com sucesso!",
    }).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
*/
    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({
            "redirect": "admin/painel"
        }))
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login)
        .service(login_submit);
}
