use actix_web::{get, web, HttpResponse, Responder};
use minijinja::{context};
use crate::app::AppState;

#[get("/painel")]
async fn dashboard(data: web::Data<AppState>) -> impl Responder {
    let tmpl = data.render.get_template("admin/dashboard.html").unwrap();
    let rendered = tmpl.render(context! {title=>"Painel"}).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(dashboard);
}