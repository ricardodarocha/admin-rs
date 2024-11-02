pub mod testes;
pub mod services;
pub mod models;
pub mod repository;
pub mod product;
pub mod infra;
pub mod app;
mod auth;
mod site;
mod admin;

use std::sync::Arc;
use actix_files::Files;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use env_logger::Env;
// use infra::sessao_usuario::auth_middleware;
use minijinja::Environment;
use reqwest;
use services::{
    cliente::{json_all_cliente, json_cliente, web_cliente, web_cliente_submit},
    pedido::{json_all_pedido, json_pedido},
    produto::{json_all_produto, json_produto, web_produto, web_produto_submit},
};
use crate::app::AppState;

use crate::infra::minijinja_utils;

use actix_web::{cookie::Key, middleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};

async fn configure_minijinja() -> Arc<Environment<'static>> {
    let mut env = Environment::new();

    env.add_filter("fmtdate", minijinja_utils::fmtdate);
    env.add_filter("fmtdateopt", minijinja_utils::fmtdateopt);
    env.add_filter("fmttime", minijinja_utils::fmttime);
    env.add_filter("fmttimeopt", minijinja_utils::fmttimeopt);
    env.add_filter("fmt", minijinja_utils::fmt);
    env.add_filter("fmt3", minijinja_utils::fmt3);
    env.add_filter("format", minijinja_utils::format_filter);

    env.set_loader(minijinja::path_loader("resources/views"));
    Arc::new(env)
}


async fn not_found() -> impl Responder {
    HttpResponse::Found()
        .append_header(("Location", "/ops"))
        .finish()
}

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

        let state = web::Data::new(AppState {
            database: database.clone(),
            client: client.clone(),
            render: render.clone(),
        });
        let cookies_secret_key = Key::generate();

        App::new()
            .app_data(state.clone())
            // .wrap_fn(auth_middleware)
            .wrap(cors)
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                cookies_secret_key.clone(),
            ))
            .service(Files::new("/node_modules", "node_modules").show_files_listing())
            .service(Files::new("/resources", "resources").show_files_listing())

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

            // Rotas que não precisam de login

            .configure(site::routes)
            .configure(auth::routes)
            .configure(admin::routes)
            .configure(testes::routes)
            // rotas que exigem login
            .service(web_produto)
            .service(web_produto_submit)
            .service(web_cliente)
            .service(web_cliente_submit)
            .service(json_cliente)
            .service(json_produto)
            .service(json_pedido)
            .service(json_all_cliente)
            .service(json_all_produto)
            .service(json_all_pedido)
            .default_service(web::to(not_found))
    })
        .bind(("localhost", 8080))?
        .run()
        .await
}
