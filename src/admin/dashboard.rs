use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};
use minijinja::context;
use crate::api::kpis::get_kpis;
use crate::app::AppState;
use crate::infra::jwt::jwt_secret;
use crate::infra::sessao_usuario::Sessao;
use crate::repository::dashboard as repo;

#[get("/painel")]
async fn dashboard(
    data: web::Data<AppState>,
    session: Session,

) -> impl Responder {

    let sessao_usuario = Sessao::from_session(&session, &jwt_secret()).unwrap();

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

    let kpis = get_kpis(sessao_usuario).await;
    let tmpl = data.render.get_template("admin/dashboard.html").unwrap();
    let rendered = tmpl.render(context! {
        title => "Dashboard",
        active_menu => "painel",
        menus,
        kpis,
    }).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(dashboard);
}