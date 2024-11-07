use actix_web::{get, web, HttpResponse, Responder};
use minijinja::context;
use crate::app::AppState;
use crate::models::QueryFiltroCliente;
use crate::repository::dashboard as repo_menus;
use crate::services::cliente as service;

#[get("/clientes")]
async fn customers_index(
    data: web::Data<AppState>,
    filtro: web::Query<QueryFiltroCliente>,

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
    let tmpl = data.render.get_template("admin/customers/index.html").unwrap();

    let filtro = filtro.into_inner();
    let clientes = service::abrir_lista_clientes(pool, &filtro).await;

    let rendered = tmpl.render(context! {
        title => "Clientes",
        active_menu => "Clientes",
        menus,
        clientes
    }).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(customers_index);
}