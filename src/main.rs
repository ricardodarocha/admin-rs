pub mod services;
pub mod models;
pub mod repository;
pub mod product;
pub mod infra;
pub mod app;
pub mod login;

use std::sync::Arc;
use actix_files::Files;
use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder};
use env_logger::Env;
use log::info;
use minijinja::{Environment, context};
use reqwest;
use crate::app::AppState;
use crate::login::*;

use std::collections::HashMap;
use crate::infra::minijinja_utils;

async fn configure_minijinja() -> Arc<Environment<'static>> {
    let mut env = Environment::new();

    env.add_filter("fmtdate", minijinja_utils::fmtdate);    
    env.add_filter("fmtdateopt", minijinja_utils::fmtdateopt);    
    env.add_filter("fmttime", minijinja_utils::fmttime);    
    env.add_filter("fmttimeopt", minijinja_utils::fmttimeopt);    
    env.add_filter("fmt", minijinja_utils::fmt);
    env.add_filter("fmt3", minijinja_utils::fmt3);
    env.add_filter("format", minijinja_utils::format_filter);

    env.set_loader(minijinja::path_loader("themes"));
    Arc::new(env)
}

#[get("/")]
async fn web_home(data: web::Data<AppState>) -> impl Responder {
    let tmpl = data.render.get_template("web/home.html").unwrap();
    let rendered = tmpl.render(context! {title => "Página Inicial"}).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

#[get("/sobre")]
async fn web_about(data: web::Data<AppState>) -> impl Responder {
    let tmpl = data.render.get_template("web/about.html").unwrap();
    let rendered = tmpl.render(context! {title => "Sobre"}).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

#[get("/contato")]
async fn web_contact(data: web::Data<AppState>) -> impl Responder {
    let tmpl = data.render.get_template("web/contact.html").unwrap();
    let rendered = tmpl.render(context! {title => "Contato"}).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

#[post("/contato")]
async fn web_contact_submit(
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

#[get("/termos")]
async fn web_terms(data: web::Data<AppState>) -> impl Responder {
    let tmpl = data.render.get_template("web/terms.html").unwrap();
    let rendered = tmpl.render(context! {title => "Termos e Condições de Uso"}).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

#[get("/politica-de-privacidade")]
async fn web_policy(data: web::Data<AppState>) -> impl Responder {
    let tmpl = data.render.get_template("web/policy.html").unwrap();
    let rendered = tmpl.render(context! {title => "Política de Privacidade"}).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

#[get("/ops")]
async fn web_ops(data: web::Data<AppState>) -> impl Responder {
    let tmpl = data.render.get_template("web/error.html").unwrap();
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

use actix_web::{cookie::Key, middleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    // dotenv::dotenv().ok();

    let database = sqlx::sqlite::SqlitePool::connect("sqlite://my_database.db").await.unwrap();
    let client = reqwest::Client::new();
    let render = configure_minijinja().await;

    let _ = sqlx::migrate!().run(&database.clone()).await.map_err(|e| format!("Erro na migração do banco de dados {e}"));

    HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            // .allowed_origin(format!("http://{}:{port}", host1 ).as_str())
            // .allowed_origin(format!("http://localhost:{port}" ).as_str())
            // .allowed_origin(format!("http://27.0.0.1:{port}" ).as_str())
            // .allowed_origin(format!("http://www.pedidonanuvem.com.br:{port}" ).as_str())
            .allow_any_header()
            .allow_any_origin()
            .allow_any_method()
            .expose_any_header()
            .supports_credentials();

        let state = web::Data::new(AppState{
        database: database.clone(),
        client: client.clone(),
        render: render.clone(),
    });
        let secret_key = Key::generate(); 

        App::new()
            .app_data(state.clone())
            .wrap(cors)
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .service(Files::new("/shared", "shared").show_files_listing())
            .service(Files::new("/node_modules", "node_modules").show_files_listing())

  
            .wrap(middleware::Logger::new(
                "%{r}a %r %s %b %{Referer}i %{User-Agent}i %T",
            )) // enable logger
            
            // .service(
            //     SwaggerUi::new("/swagger-ui/{_:.*}")
            //         .url("/api-docs/openapi.json", ApiDoc::openapi()),
            // )
            
            // .service(web::resource("/api/ping").route(web::get().to(ping)))
            // .service(actix_files::Files::new("/static","./static")
                // .show_files_listing()
                // .use_last_modified(true)
                // .index_file("index.html")
            // )            
  
            .service(web_home)
            .service(web_about)
            .service(web_contact)
            .service(web_contact_submit)
            .service(web_login)
            .service(web_register)
            .service(web_login_submit)
            .service(web_register_submit)
            .service(web_terms)
            .service(web_policy)
            .service(web_ops)
            .default_service(web::to(not_found))
    })
        .bind(("localhost", 8080))?
        .run()
        .await
}
