pub mod service;
pub mod repo;
pub mod model;
pub mod view;
// pub mod mock;

pub mod controller {
    use actix_session::Session;
    use actix_web::{web, HttpRequest, Responder}; //HttpResponse, 
    use actix_web::get;
    use minijinja::context;
    use crate::admin::repo::abrir_empresa_one;
    // use actix_web::http::header::LOCATION;
    use crate::app::AppState;
    use crate::auth::repo::recados_do_usuario;
    use crate::auth::session::{get_user, has_logged};
    use crate::dashboard::repo::{clientes_for_user, dash_for_user};
    use crate::land::model::Menu;
    use crate::land::repo;
    // use crate::land::mock::pedido::create_sample_order;
    // use crate::land::repo::listar_pedidos_all;
    // use crate::infra::result::Result;

    #[get("/")]
    pub async fn get_landing(
        data: web::Data<AppState>,
            session: Session,
            _req: HttpRequest,
        // path: web::Path<(String, String)>,
    ) ->impl Responder {


        dbg!("/root landingpage -> ");
        let pool = &data.database.conn;

        let logged = has_logged(&pool, &session).await;
        // let url_for = format!("{}/login", std::env::var("SITE").unwrap());
        
        // if !logged {             
        //     return HttpResponse::SeeOther()
        //     .insert_header((LOCATION, "/login"))
        //     .finish();
        // };

        let usuario = get_user(pool, &session).await;
        let empresa = match usuario.clone() {
            Some(usuario) => {
                  let id_empresa = usuario.clone().id_empresa;
                  match id_empresa {
                    Some(id) => {
                        if let Ok(empresa) = abrir_empresa_one(pool, &id).await {
                            Some(empresa)
                        }
                        else {
                        None
                        }
                    },
                    None => None,
                  }
            },
            None => None ,
        };
        let recados = match usuario.clone() {
            Some(usuario) => {
                  let recados = recados_do_usuario(pool, &usuario.id).await.unwrap();
                  Some(recados)
            },
            None => None ,
        };
       
        let menus: Vec<Menu> = 
            match usuario.clone() {
                Some(usuario) => repo::get_menus(pool, usuario.id, "").await.unwrap(),
                None => vec![],
        };

        let flash = session.remove("flash").unwrap_or("".to_string()); 
        let msg_error = format!("{}", session.remove("msg_error").unwrap_or("".to_string()));

        let (indicadores, clientes) =
        match usuario.clone() {
            Some(user) => {

                let indicadores = match dash_for_user(pool, user.clone().id).await { Ok(value) => Some(value), Err(_) => None };
                let clientes = match clientes_for_user(pool, user.id).await { Ok(value) => Some(value), Err(_) => None };

                ( Some(indicadores), Some(clientes) )
            },
            None => {
                (None, None)

            }
        };
        
        if logged {
            println!("Logged");
            crate::infra::render::render_minijinja("dash/dash_usuario.html", context!(
                menus, 
                usuario, 
                empresa,
                indicadores,
                recados,
                clientes,
                flash,
                msg_error)).unwrap() 
        } else
        {   let navbar = true;
            println!("Not Logged");
            crate::infra::render::render_minijinja("land/land.html", context!(navbar, usuario, empresa, flash, msg_error)).unwrap() 
        }
    
    }  

    #[get("/landingpage")]
    pub async fn get_landing_page(
        data: web::Data<AppState>,
            session: Session,
            _req: HttpRequest,
        // path: web::Path<(String, String)>,
    ) ->impl Responder {

        dbg!("/landingpage -> ");
        let pool = &data.database.conn;

        let usuario = get_user(pool, &session).await;
        let id_empresa = usuario.clone().unwrap().id_empresa;
        let empresa = abrir_empresa_one(pool, &id_empresa.clone().unwrap()).await.unwrap();

        let flash = session.remove("flash").unwrap_or("".to_string()); 
        let msg_error = format!("{}", session.remove("msg_error").unwrap_or("".to_string()));   
        let navbar = true;

        crate::infra::render::render_minijinja("land/land.html", context!(navbar, usuario, empresa, flash, msg_error)) 
    
    }  

}

use controller::*;

     
    pub fn routes(cfg: &mut crate::web::ServiceConfig) {
    cfg.service(crate::web::scope("")
            .service(get_landing)
            .service(get_landing_page)
        );
    }
