pub mod controller {
    use actix_session::Session;
    use actix_web::{web, HttpRequest, HttpResponse, Responder};
    use actix_web::{post, get,  put, delete};
    use log::info;
    use minijinja::context;
    // use serde_json::json;
    use crate::app::AppState;
    // use actix_web::http::header::LOCATION;
    use crate::auth::session::{get_user, has_logged};
    use crate::land::model::Menu;
    use crate::land::repo::get_menus;
    use actix_web::http::header::LOCATION;

    ///Login Form
    // #[utoipa::path(
    //     responses(
    //         (status = 200, description = "Login Form")
    // ))]
    #[get("/")]
    pub async fn login_form(
        req: &dev::ServiceRequest,
        session: Session,
        data: web::Data<AppState>,
    ) -> impl Responder {
        let current_uri = &req.uri();
        info!("{:?}", current_uri);

        // let current_url = req.path().to_string();
        let pool = &data.database.conn;
    
       if has_logged(pool, &session, &"USER").await {
           let current_url = session.remove("current_url").unwrap_or("".to_string());
           return Ok(HttpResponse::SeeOther()
           .insert_header((LOCATION, current_url))
           .finish())
       };
       
    // Extrects comments, error alerts or info notifications 
       let flash = session.remove("flash").unwrap_or("".to_string()); // ℹ
       let alert = format!("{}", session.remove("alert").unwrap_or("".to_string())); // ⚠
    
        // Remover comentários se precisa listar as actions deste usuário para o contexto de entidade
    //    let menus: Vec<Menu> =
    //        match usuario.clone() {
    //            Some(usuario) => get_menus(pool, usuario.id,
    //            "entidade").await.unwrap(),
    //            None => vec![],
    //        };
    
        if let Some(form) = found_entidade {
            crate::infra::render::render_minijinja(
            "entidade/form_entidade.html", context!(
                menus,
                estados,
                usuario,
                current_id,
                form,
                flash,
                msg_error))
        }
    }
    ///Login Form
    // #[utoipa::path(
    //     responses(
    //         (status = 200, description = "Login Form")
    // ))]
    #[get("/")]
    pub async fn login_form(
        req: &dev::ServiceRequest,
        session: Session,
        path: web::Path<String>,
        data: web::Data<AppState>,
    ) -> impl Responder {
        let current_uri = &req.uri();
        info!("{:?}", current_uri);

        let current_url = req.path().to_string();
        let pool = &data.database.conn;
        let current_id = path.into_inner();
    //    let (current_tenant, current_id) = path.into_inner();
    
    // Remove comments when requires auth
    //    if !has_logged(pool, &session, &"USER").await {
    //        session.insert("current_url", current_url.clone()).ok();
    //        return Ok(HttpResponse::SeeOther()
    //        .insert_header((LOCATION, "/login"))
    //        .finish())
    //    };

    
    // Remove comments when requires session data
    //    let username = get_user(pool, &session).await;
    
    let found_entidade: Option<entidade> = sqlx::query_as!(
    
        entidade, r#"--sql
        select * from entidade
        where id <> '0'
    //    and id_empresa = entidade
        "#,
        current_id,
    //    current_tenant,
        )
        .fetch_optional(pool).await;
    
        if let Ok(value) = result {
            info!("entidade localizado");
            value
        } else {
            info!("entidade não encontrado");
            None
        }
    }
    
    // let found_entidade: Option<entidade> = repo::abrir_entidade(pool, id_empresa.clone(), &current_id.clone()).await;
    // let other_ent = open_other(pool).await.unwrap();
    
        // Remover comentários se exige flash e msg error
    //    let flash = session.remove("flash").unwrap_or("".to_string());
    //    let msg_error = format!("{}", session.remove("msg_error").unwrap_or("".to_string()));
    
        // Remover comentários se precisa listar as actions deste usuário para o contexto de entidade
    //    let menus: Vec<Menu> =
    //        match usuario.clone() {
    //            Some(usuario) => get_menus(pool, usuario.id,
    //            "entidade").await.unwrap(),
    //            None => vec![],
    //        };
    
        if let Some(form) = found_entidade {
            crate::infra::render::render_minijinja(
            "entidade/form_entidade.html", context!(
                menus,
                estados,
                usuario,
                current_id,
                form,
                flash,
                msg_error))
        }
    }
}

use controller::*;

pub fn routes(cfg: &mut crate::web::ServiceConfig) {
cfg.service(
    crate::web::scope("/login")
        .service(login_form)
    );
}