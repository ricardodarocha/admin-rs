pub mod service;
pub mod repo;
pub mod model;
pub mod view;
pub mod mock;

pub mod controller {
    use actix_session::Session;
    use actix_web::{web, HttpRequest, HttpResponse, Responder};
    use actix_web::get;
    use minijinja::context;
    use actix_web::http::header::LOCATION;
    use crate::admin::repo::abrir_empresa_one;
    use crate::app::AppState;
    use crate::auth::session::{get_user, has_logged}; //, has_permission
    use crate::dashboard::repo::dash_for_user;
    // use crate::infra::result::Result;

    
    #[get("")]
    pub async fn get_dashboard(
        data: web::Data<AppState>,
            session: Session,
            _req: HttpRequest,
        // path: web::Path<(String, String)>,
    ) ->impl Responder {

        dbg!("/dashboard/index -> ");
        let pool = &data.database.conn;
        // let url_for = format!("{}/login", std::env::var("SITE").unwrap());

        if !has_logged(&pool, &session).await {
             return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/login"))
            .finish())
        };

        let usuario = get_user(pool, &session).await;
        let id_empresa = usuario.clone().unwrap().id_empresa;
        let empresa = abrir_empresa_one(pool, &id_empresa.clone().unwrap()).await.unwrap();
        let dashboard = dash_for_user(pool, usuario.clone().unwrap().id).await.unwrap();
        // let form = LoginForm::default();

        let flash = session.remove("flash").unwrap_or("".to_string()); 
        let msg_error = format!("{}", session.remove("msg_error").unwrap_or("".to_string()));   


        crate::infra::render::render_minijinja("dash/dash_lista_pedidos.html", context!(usuario, empresa, dashboard, flash, msg_error)) 
    
    }  

    #[get("/pedido/{id}")]
    pub async fn get_pedido(
        data: web::Data<AppState>,
            session: Session,
            _req: HttpRequest,
        // path: web::Path<(String, String)>,
    ) ->impl Responder {


        dbg!("/dashboard/pedido/id -> ");
        let pool = &data.database.conn;
        // let url_for = format!("{}/login", std::env::var("SITE").unwrap());

        if !has_logged(&pool, &session).await {
             return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/login"))
            .finish())
        };

        let usuario = get_user(pool, &session).await;
        let id_empresa = usuario.clone().unwrap().id_empresa;
        let empresa = abrir_empresa_one(pool, &id_empresa.clone().unwrap()).await.unwrap();
        let dashboard = dash_for_user(pool, usuario.clone().unwrap().id).await.unwrap();
        // let form = LoginForm::default();

        let flash = session.remove("flash").unwrap_or("".to_string()); 
        let msg_error = format!("{}", session.remove("msg_error").unwrap_or("".to_string()));   
        crate::infra::render::render_minijinja("dash/dash_pedido.html", context!(usuario, dashboard,  empresa, flash, msg_error)) 
    }

    #[get("/pedido/new")]
    pub async fn get_form_pedido(
        data: web::Data<AppState>,
            session: Session,
            _req: HttpRequest,
        // path: web::Path<(String, String)>,
    ) ->impl Responder {


        dbg!("/pedido/new -> ");
        let pool = &data.database.conn;
        // let url_for = format!("{}/login", std::env::var("SITE").unwrap());

        if !has_logged(&pool, &session).await {
             return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/login"))
            .finish())
        };

        let usuario = get_user(pool, &session).await;
        let id_empresa = usuario.clone().unwrap().id_empresa;
        let dashboard = dash_for_user(pool, usuario.clone().unwrap().id).await.unwrap();
        let empresa = abrir_empresa_one(pool, &id_empresa.clone().unwrap()).await.unwrap();
        // let form = LoginForm::default();

        let flash = session.remove("flash").unwrap_or("".to_string()); 
        let msg_error = format!("{}", session.remove("msg_error").unwrap_or("".to_string()));   
        crate::infra::render::render_minijinja("dash/dash_pedido_form.html", context!(usuario, dashboard, empresa, flash, msg_error)) 
    }
}

use controller::*;

     
    pub fn routes(cfg: &mut crate::web::ServiceConfig) {
    cfg.service(crate::web::scope("/dashboard")
            .service(get_dashboard)
            .service(get_form_pedido)
            .service(get_pedido)
        );
    }
