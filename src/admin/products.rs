use actix_web::{get, web, HttpResponse, Responder};
use minijinja::context;
use crate::app::AppState;
use crate::models::QueryFiltro;
use crate::repository::dashboard as repo_menus;
use crate::services::produto as service;

#[get("/produtos")]
async fn products_index(
    data: web::Data<AppState>,
    filtro: web::Query<QueryFiltro>,

) -> impl Responder {

    let pool = &data.database;
    let find_menus = repo_menus::carregar_menus(&pool).await;
    let menus = match find_menus {
        Ok(menus) => {
            menus
        }
        Err(_) => {
            vec!()
        }
    };
    let tmpl = data.render.get_template("admin/products/index.html").unwrap();

    let filtro = filtro.into_inner();
    let produtos = service::abrir_lista_produtos(pool, filtro).await;

    let rendered = tmpl.render(context! {
        title => "Produtos",
        active_menu => "produtos",
        menus,
        produtos
    }).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(products_index);
}