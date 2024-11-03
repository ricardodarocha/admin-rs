use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};
// use log::info;
use minijinja::context;
use crate::app::AppState;

#[get("/testes")]
async fn testes(
    data: web::Data<AppState>,
    _session: Session ) 
    
-> impl Responder {
    
    let tmpl = data.render.get_template("site/testes.html").unwrap();
    let rendered = tmpl.render(context! {title => "Mapa de rotas para teste"}).unwrap();

    HttpResponse::NotFound()
        .content_type("text/html")
        .body(rendered)
}

pub fn routes(cfg: &mut web::ServiceConfig){
    cfg.service(testes);
}