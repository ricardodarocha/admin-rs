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
pub mod helpers;
mod auth;
mod site;
mod admin;

use actix_files::Files;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use env_logger::Env;

//todo! refactory all services routes to handler/route
use handlers::cliente::{json_all_cliente, json_cliente, web_cliente, web_cliente_submit};
use handlers::grafico::{json_all_grafico, json_grafico};
use handlers::produto::{json_all_produto, json_produto};
use application::controller::pedido::consultas as consultas_pedido;
use application::controller::pedido::acoes as acoes_pedido;
use handlers::relatorio::vendas_por_mes;
use log::info;
use crate::app::AppState;
use crate::infra::minijinja::configure_minijinja;

use actix_web::{cookie::Key, middleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};

fn get_host_port() -> (String, String) {
    
    let host = std::env::var("HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    (host, port)
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

    let threads: usize = std::env::var("THREADS")
    .unwrap_or_else(|_| "4".to_string())
    .parse()
    .expect("configure o número de THREADS nas variáveis de ambiente THREADS=");

    HttpServer::new(move || {

        ///////////////////////////////////////////////
        ///////// Cors   ///////////////////// ////////
        ///////////////////////////////////////////////
        ///////////////////////////////////////////////
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
        ///////////////////////////////////////////////
        ///////////////////////////////////////////////
         
        

        ///////////////////////////////////////////////
        ///////// Setup de variáveis de estado ////////
        ///////////////////////////////////////////////
        ///////////////////////////////////////////////
        let state = web::Data::new(AppState {
            database: database.clone(),
            client: client.clone(),
            render: render.clone(),
        });
        ///////////////////////////////////////////////
        ///////////////////////////////////////////////
        ///////////////////////////////////////////////
        ///////////////////////////////////////////////

        let cookies_secret_key = Key::generate();
        
        let (host_info, port_info) = get_host_port();
        info!("🟢 rodando http://{}:{} [{:?}]" , host_info, port_info, std::thread::current().id() );
        App::new()
            .app_data(state.clone())
            .wrap(cors)
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                cookies_secret_key.clone(),
            ))
            .service(Files::new("/node_modules", "node_modules").show_files_listing())
            .service(Files::new("/resources", "resources").show_files_listing())

            .wrap(middleware::Logger::new(
                "%{r}a %r %s %b %{Referer}i %{User-Agent}i %T",
            )) 
            
            //todo! Condigurar Swagger
            // .service(
            //     SwaggerUi::new("/swagger-ui/{_:.*}")
            //         .url("/api-docs/openapi.json", ApiDoc::openapi()),
            // )

            // .service(web::resource("/api/ping").route(web::get().to(ping)))
             
            .service(actix_files::Files::new("/storage","./storage")
            .show_files_listing()
            .use_last_modified(true))

            // .service(actix_files::Files::new("/static","./static")
            // .show_files_listing()
            // .use_last_modified(true))

            .configure(site::routes)
            .configure(auth::routes)
            .configure(admin::routes)
            .configure(testes::routes)
            .configure(acoes_pedido::routes)
            .configure(consultas_pedido::routes)
            .configure(api::routes)
            .service(vendas_por_mes)
            .service(web_cliente)
            .service(web_cliente_submit)
            .service(json_cliente)
            .service(json_produto)
            .service(json_grafico)
            .service(json_all_cliente)
            .service(json_all_produto)
            .service(json_all_grafico)

            .default_service(web::to(not_found))
    })  .workers(threads)
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
