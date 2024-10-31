use std::sync::Arc;
use actix_files::Files;
use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder};
use minijinja::{Environment, context};
use serde_json::json;
use std::collections::HashMap;

async fn configure_minijinja() -> Arc<Environment<'static>> {
    let mut env = Environment::new();
    env.set_loader(minijinja::path_loader("themes"));
    Arc::new(env)
}

#[get("/")]
async fn web_home(env: web::Data<Arc<Environment<'static>>>) -> impl Responder {
    let tmpl = env.get_template("web/home.html").unwrap();
    let rendered = tmpl.render(context! {title => "Página Inicial"}).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

#[get("/sobre")]
async fn web_about(env: web::Data<Arc<Environment<'static>>>) -> impl Responder {
    let tmpl = env.get_template("web/about.html").unwrap();
    let rendered = tmpl.render(context! {title => "Sobre"}).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

#[get("/contato")]
async fn web_contact(env: web::Data<Arc<Environment<'static>>>) -> impl Responder {
    let tmpl = env.get_template("web/contact.html").unwrap();
    let rendered = tmpl.render(context! {title => "Contato"}).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

#[post("/contato")]
async fn web_contact_submit(
    form: web::Form<HashMap<String, String>>,
    env: web::Data<Arc<Environment<'static>>>,
) -> impl Responder {
    println!("Recebido POST com dados: {:?}", form);

    let tmpl = env.get_template("shared/views/ajaxToast.html").unwrap();
    let rendered = tmpl.render(context! {
        toast_icon => "bi-check-circle",
        toast_class => "toast-success",
        toast_text => "Mensagem enviada com sucesso!",
    }).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({
            "toast": "teste"
        }))
}

#[get("/termos")]
async fn web_terms(env: web::Data<Arc<Environment<'static>>>) -> impl Responder {
    let tmpl = env.get_template("web/terms.html").unwrap();
    let rendered = tmpl.render(context! {title => "Termos e Condições de Uso"}).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

#[get("/politica-de-privacidade")]
async fn web_policy(env: web::Data<Arc<Environment<'static>>>) -> impl Responder {
    let tmpl = env.get_template("web/policy.html").unwrap();
    let rendered = tmpl.render(context! {title => "Política de Privacidade"}).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

#[get("/ops")]
async fn web_ops(env: web::Data<Arc<Environment<'static>>>) -> impl Responder {
    let tmpl = env.get_template("web/error.html").unwrap();
    let rendered = tmpl.render(context! {title => "Ops"}).unwrap();

    HttpResponse::NotFound()
        .content_type("text/html")
        .body(rendered)
}

async fn not_found() -> impl Responder {
    HttpResponse::Found()
        .append_header(("Location", "/ops"))
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = configure_minijinja().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(env.clone()))
            .service(Files::new("/shared", "shared").show_files_listing())
            .service(Files::new("/node_modules", "node_modules").show_files_listing())
            .service(web_home)
            .service(web_about)
            .service(web_contact)
            .service(web_contact_submit)
            .service(web_terms)
            .service(web_policy)
            .service(web_ops)
            .default_service(web::to(not_found))
    })
        .bind(("localhost", 8080))?
        .run()
        .await
}
