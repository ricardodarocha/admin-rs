pub mod aapp; //Abstract App - a camada semântica
pub mod app; //App State used by Actixweb
pub mod cidade;
pub mod produto;
pub mod land;
pub mod auditoria;
pub mod pedido;
pub mod entidade;
pub mod relatorios;
pub mod dashboard;
pub mod pessoa; //a classe mais abstrata de Pessoa
pub mod pessoas; //clientes, fornecedores etc
pub mod itens;
pub mod operacoes;
pub mod infra;
pub mod sentinel; //Sistema de notificações remotas
pub mod admin; //Sistema de cadastros em geral, empresas
pub mod auth; //Sistema de cadastro de usuários e permissões
pub mod config; //configuracoes do sistema 

use actix_web::{cookie::Key, middleware, web, App, HttpRequest, HttpServer, Responder};
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use app::AppState;
// use auth::{model::User, session::has_logged};
use config::database;
use env_logger::Env;
use infra::controller::ping;
use minijinja::context;
use utoipa::OpenApi;
// use utoipa::{openapi, Modify, OpenApi};
// use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa_swagger_ui::SwaggerUi;
use crate::infra::job::job_scheduler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    dotenv::dotenv().ok();
    let port = std::env::var("PORT_API").unwrap().parse::<u32>().unwrap();
    let host = std::env::var("SERVER_API").unwrap();
    let database = database::DbInstance::init().await;

    let pool_clone = database.conn.clone();
    actix_web::rt::spawn(async move {
        println!("🟢 job running... 🔨");
        job_scheduler(pool_clone).await;
    });


    let _ = sqlx::migrate!().run(&database.conn.clone()).await.map_err(|e| format!("Erro na migração do banco de dados {e}"));
    let app_data = web::Data::new(app::AppState {
                    client: reqwest::Client::new(),
                    database,
                });

                
    // minijinja_embed::load_templates!(&mut env);

    let _secret =
        std::env::var("SECRET").unwrap_or_else(|_| "935b43f5-4313-5f8b-8cfe-5e05692226dd".to_string());
    let _database_url = std::env::var("DATABASE_URL").unwrap();   
   
    let secret_key = Key::generate(); 

    #[derive(OpenApi)]
    #[openapi(
        paths(
            // auth::controller::login_form,
            // crate::produto::routes,
            // crate::pessoa::routes,
            // crate::dashboard::routes,
            // crate::pedido::routes,
            // crate::admin::routes,
            // crate::land::routes,
        )
    )]

    struct ApiDoc;
    // struct SecurityAddon;
    // impl Modify for SecurityAddon {
    //     fn modify (&self, openapi: &mut utoipa::openapi::OpenApi) {
    //         let components = openapi::Components.as_mut().unwrap();
    //         components.add_security_scheme(
    //             "bearer_auth",
    //             SecurityScheme::Http(HttpBuilder::new()
    //         )       
    //             .scheme(HttpAuthScheme::Bearer)
    //             .bearer_format("JWT")
    //             .build(),
    //         );
    //         components.add_security_scheme("basic_auth",
    //             SecurityScheme::Http(HttpBuilder::new()
    //         )       
    //             .scheme(HttpAuthScheme::Basic)
    //             .build(),
    //         );

    //     }
    // }

    println!("🌎 live server at {}:{}", host.clone(), port);
    // let host1 = host.clone();
    let host2 = host.clone();
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

        App::new()
            .app_data(app_data.clone())
            .wrap(cors)
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .wrap(middleware::Logger::new(
                "%{r}a %r %s %b %{Referer}i %{User-Agent}i %T",
            )) // enable logger
            
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            
            // .service(web::resource("/api/ping").route(web::get().to(ping)))
            .service(actix_files::Files::new("/static","./static")
                .show_files_listing()
                .use_last_modified(true)
                // .index_file("index.html")
            )            
            .route("/index", web::get().to(index))
            .service(ping)
            // .service(index)
            .service(actix_files::Files::new("/images", "./static/img"))
            .configure(auth::routes)
            .configure(produto::routes)
            .configure(pessoa::routes)
            .configure(dashboard::routes)
            .configure(pedido::routes)
            .configure(admin::routes)
            .configure(land::routes)
            // .configure(controller::vitrine::routes)
            // .configure(controller::compras::routes)
            // .configure(controller::estoque::routes)
            // .configure(controller::crm::routes)
            // .configure(controller::pedido::routes)
            // .configure(controller::vendas::routes)
            // .configure(controller::financeiro::routes)
            // .configure(controller::recursos_humanos::routes)
            // .configure(controller::projetos::routes)
            // .configure(controller::custos::routes)
            // .route("/", web::get().to(index))
            // .route("/login", web::post().to(login))
            // .route("/logout", web::post().to(logout))
    })
    .bind(format!("{}:{}", host2, port))?
    .run()
    .await
}

//Serving the Registration and sign-in page
async fn index(data:  web::Data<AppState>, session: Session, _req: HttpRequest) -> impl Responder {
    // let path: PathBuf = "./static/index.html".parse().unwrap();
    // Ok(NamedFile::open(path).unwrap())
    let usuario = auth::session::get_user(&data.database.conn, &session).await;
    infra::render::render_minijinja("index.html", context!(usuario) )
}