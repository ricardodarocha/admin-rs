use actix_web::{get, web, HttpResponse, Responder};
use minijinja::context;
use crate::app::AppState;
use crate::repository::dashboard as repo;

#[get("/painel")]
async fn dashboard(data: web::Data<AppState>) -> impl Responder {
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
    let tmpl = data.render.get_template("admin/dashboard.html").unwrap();
    let rendered = tmpl.render(context! {
        title => "Dashboard",
        active_menu => "painel",
        menus
    }).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(dashboard);
}