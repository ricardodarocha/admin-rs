pub mod api;
pub mod views;
pub mod core;
pub mod application;
pub mod handlers;
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
mod helpers;

use std::sync::Arc;
use actix_files::Files;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use env_logger::Env;
// use infra::sessao_usuario::auth_middleware;
use minijinja::Environment;
use reqwest;

//todo! refactory all services routes to handler/route
use handlers::cliente::{json_all_cliente, json_cliente, web_cliente, web_cliente_submit};
use handlers::grafico::{json_all_grafico, json_grafico};
use handlers::produto::{json_all_produto, json_produto};
use application::controller::pedido::consultas as consultas_pedido;
use application::controller::pedido::acoes as acoes_pedido;
use handlers::relatorio::vendas_por_mes;
use crate::app::AppState;

use actix_web::{cookie::Key, middleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};

fn get_host_port() -> (String, String) {
    
    let host = std::env::var("HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    (host, port)
}

async fn configure_minijinja() -> Arc<Environment<'static>> {
    let mut env = Environment::new();
    env.add_function("url", helpers::url);
    env.add_function("mascara", helpers::mascara);
    env.add_function("anonimizar", helpers::anonimizar);
    env.add_function("fmt_decimal", helpers::fmt_decimal);
    env.add_function("fmt_cpf", helpers::fmt_cpf);
    env.add_function("fmt_cnpj", helpers::fmt_cnpj);
    env.add_function("fmt_cep", helpers::fmt_cep);
    env.add_function("numero_por_extenso", helpers::por_extenso::numero_por_extenso);
    env.add_function("multimidia", helpers::img_src::multimidia);


    env.add_filter("fmtdate", helpers::filter::fmtdate);    
    env.add_filter("fmtdateopt", helpers::filter::fmtdateopt);
    env.add_filter("fmttime", helpers::filter::fmttime);
    env.add_filter("fmttimeopt", helpers::filter::fmttimeopt);
    env.add_filter("fmt", helpers::filter::fmt);
    env.add_filter("fmt3", helpers::filter::fmt3);

    env.add_filter("mascara", helpers::mascara);
    env.add_filter("anonimizar", helpers::anonimizar);
    env.add_filter("fmt_decimal", helpers::fmt_decimal);
    env.add_filter("fmt_cpf", helpers::fmt_cpf);
    env.add_filter("fmt_cnpj", helpers::fmt_cnpj);
    env.add_filter("fmt_cep", helpers::fmt_cep);
    env.add_filter("numero_por_extenso", helpers::por_extenso::numero_por_extenso);

    env.add_function("format", helpers::filter::format_filter);
    // env.add_function("url_for", |route: String| minijinja_utils::url_for(&route));

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
    dotenv::dotenv().ok();

    let database = sqlx::sqlite::SqlitePool::connect("sqlite://my_database.db").await.unwrap();
    let client = reqwest::Client::new();
    let render = configure_minijinja().await;
    let (host, port) = get_host_port();

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
             

            //Resources
            .service(actix_files::Files::new("/storage","./storage")
            .show_files_listing()
            .use_last_modified(true))

            // .service(actix_files::Files::new("/static","./static")
            // .show_files_listing()
            // .use_last_modified(true))

            // Rotas que não precisam de login

            .configure(site::routes)
            .configure(auth::routes)

            .service(vendas_por_mes)
            .configure(admin::routes)
            .configure(testes::routes)
            .configure(acoes_pedido::routes)
            .configure(consultas_pedido::routes)
            .configure(api::routes)
            // rotas que exigem login
            .service(web_cliente)
            .service(web_cliente_submit)
            .service(json_cliente)
            .service(json_produto)
            .service(json_grafico)
            .service(json_all_cliente)
            .service(json_all_produto)
            .service(json_all_grafico)
            .default_service(web::to(not_found))
    })
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
