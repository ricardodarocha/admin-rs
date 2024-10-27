use std::sync::Arc;
use actix_web::{get, post, put, patch, delete, web, App, HttpServer, HttpResponse, Responder};
use minijinja::{Environment, context};

async fn configure_minijinja() -> Arc<Environment<'static>>{
    let mut env = Environment::new();
    env.set_loader(minijinja::path_loader("themes"));
    Arc::new(env)
}

#[get("/")]
async fn web_home(env: web::Data<Arc<Environment<'static>>>) -> impl Responder {
    let tmpl = env.get_template("web/home.html").unwrap();
    let rendered = tmpl.render(context!{title => "Página Inicial"}).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = configure_minijinja().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(env.clone()))
            .service(web_home)
    })
        .bind(("localhost", 8080))?
        .run()
        .await
}
