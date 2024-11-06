use actix_web::{get, web, HttpResponse, Responder};
use minijinja::context;
use crate::app::AppState;
use crate::repository::dashboard as repo;

#[get("/produtos")]
async fn products_index(data: web::Data<AppState>) -> impl Responder {
    let pool = &data.database;
    let find_menus = repo::carregar_menus(&pool).await;
    let menus = match find_menus {
        Ok(menus) => {
            menus
        }
        Err(_) => {
            vec!()
        }
    };
    let tmpl = data.render.get_template("admin/products/index.html").unwrap();
    let rendered = tmpl.render(context! {
        title => "Produtos",
        active_menu => "produtos",
        menus
    }).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(products_index);
}