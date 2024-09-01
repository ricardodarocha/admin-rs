pub mod service;
pub mod repo;
pub mod model;
pub mod view;

pub mod controller {
    use actix_session::Session;
    use actix_web::{web, HttpRequest, HttpResponse, Responder};
    use actix_web::Error;
    use actix_web::{post, get,  put, delete};
    use crate::admin::model::Postentidade;
    use crate::app::AppState;


#[get("/{id}")]
pub async fn entidade_form(
    _req: HttpRequest,
    session: Session,
    path: web::Path<String>,
    data: web::Data<AppState>,

    ) -> impl Responder {

    dbg!("GET entidade/{id}   -> entidade_form");
    let pool = &data.database.conn;
    let current_id = path.into_inner();

    if !has_logged(pool, &session).await {
                        return Ok(HttpResponse::SeeOther()
                    .insert_header((LOCATION, "/login"))
                    .finish())
                };
    let (operation, permission) = (UserOperation::Edit, UserPermission::Entidade);
    if !has_permission(pool, &session, operation, permission).await {
        return user_has_not_permission(&"edit entidade")
    };
    let usuario = get_user(pool, &session).await;
    let id_empresa = usuario.clone().unwrap().id_empresa.clone().unwrap().to_string();
    let found_entidade: Option<Entidade> = repo::abrir_entidade(pool, id_empresa.clone(), &current_id.clone()).await;
    // < OTHER ENTITIES >
    let flash = session.remove("flash").unwrap_or("".to_string());
    let msg_error = format!("{}", session.remove("msg_error").unwrap_or("".to_string()));   
    let menus: Vec<Menu> =
    match usuario.clone() {
        Some(usuario) => get_menus(pool, usuario.id,
        "entidade").await.unwrap(),
        None => vec![],
    };

    if let Some(form) = found_entidade {
        crate::infra::render::render_minijinja(
        "entidade/form_entidade.html", context!(
            menus,
            // < OTHER ENTITIES >
            usuario,
            current_id,
            form,
            flash,
            msg_error))
        } else
        {    
            let form = PostEntidade::default();
            crate::infra::render::render_minijinja("entidade/form_entidade.html", context!(
                menus,
                // < OTHER ENTITIES >
                usuario,
                current_id,
                form,
                flash,
                msg_error))
        }
    }
    
    /// Insere uma entidade multi_tenant
    #[post("/entidade/{empresa_id}/{user_id}")]
    pub async fn post_entidade(
        app: web::Data<AppState>,
        session: Session,
        entidade: web::Json<PostEntidade>,
        path: web::Path<(String, String)>,
        req: HttpRequest,
    ) -> Result<impl Responder, Error> {
        let (empresa_id, usr_uuid) = path.into_inner();

        match crate::admin::service::salvar_entidade(app, &entidade).await {
            Ok(entidade) => Ok(HttpResponse::Ok().json(entidade)),
            Err(e) => Ok(HttpResponse::BadRequest().json(e.to_string())),
        }
    }
    
    #[get("/entidade/{entidade_id}/{user_id}")]
    pub async fn get_entidade(
        app: web::Data<AppState>,
        session: Session,
        // body: web::Json<String>,
        path: web::Path<(String, String)>,
        req: HttpRequest,
    ) -> Result<impl Responder, Error> {
        let (entidade_id, usr_id) = path.into_inner();

        match crate::admin::repo::select_entidade(app, &entidade_id).await {
            Ok(entidade) => Ok(HttpResponse::Ok().json(entidade)),
            Err(e) => Ok(HttpResponse::BadRequest().json(e.to_string())),
        }
    }
    
    #[post("/entidade/{entidade_id}/{user_id}")]
    pub async fn put_entidade(
        app: web::Data<AppState>,
        session: Session,
        entidade: web::Json<Putentidade>,
        path: web::Path<(String, String)>,
        req: HttpRequest,
    ) -> Result<impl Responder, Error> {
        let (entidade_id, usr_uuid) = path.into_inner();

        match crate::admin::service::update_entidade(app, &entidade).await {
            Ok(entidade) => Ok(HttpResponse::Ok().json(entidade)),
            Err(e) => Ok(HttpResponse::BadRequest().json(e.to_string())),
        }
    }
    
    #[post("/entidade/{entidade_id}/{user_id}")]
    pub async fn delete_entidade(
        app: web::Data<AppState>,
        session: Session,
        body: web::Bytes,
        path: web::Path<(String, String)>,
        req: HttpRequest,
    ) -> Result<impl Responder, Error> {

    }
    


        // Define as rotas para o controlador de autenticação
        
        pub fn routes(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/entidade")
                .service(get_entidade)
                .service(put_entidade)
                .service(post_entidade)
                .service(delete_entidade)
            )
        }

    }
