pub mod service;
pub mod repo;
pub mod model;
pub mod view;

pub mod controller {
    use actix_session::Session;
    use actix_web::{web, HttpRequest, HttpResponse, Responder};
    use actix_web::{post, get,  put, delete};
    // use serde_json::json;
    use crate::admin::model::{PostEmpresa, PutEmpresa};
    use crate::app::AppState;
    use crate::auth::model::{UserOperation, UserPermission};  
    // use actix_web::http::header::LOCATION;
    use crate::auth::session::{get_user, has_permission, user_has_not_permission};

    /// Insere uma empresa
    #[post("/empresa/{empresa_id}/{user_id}")]
    pub async fn post_empresa(
        app: web::Data<AppState>,
        session: Session,
        empresa: web::Json<PostEmpresa>,
        path: web::Path<(String, String)>,
        _req: HttpRequest,

    ) -> impl Responder {
        let pool = &app.database.conn;
        
        let (operation, permission) = (UserOperation::Edit, UserPermission::Empresa);
        if !has_permission(pool, &session, operation, permission).await {
            return user_has_not_permission(&"edit empresa")
        };

        let usuario = get_user(pool, &session).await.unwrap();

        let (_empresa_id, _usr_uuid) = path.into_inner();

        match crate::admin::service::inserir_empresa(pool, usuario.id, &empresa).await {
            Ok(empresa) => Ok(HttpResponse::Ok().json(empresa)),
            Err(e) => Ok(HttpResponse::BadRequest().json(e.to_string())),
        }
    }

    /// Exibe uma empresa pelo id
    #[get("/empresa/{empresa_id}/{user_id}")]
    pub async fn get_empresa(
        app: web::Data<AppState>,
        session: Session,
        // body: web::Json<String>,
        path: web::Path<(String, String)>,
        _req: HttpRequest,
    ) -> impl Responder {


        let (operation, permission) = (UserOperation::Edit, UserPermission::Empresa);
        if !has_permission(&app.database.conn, &session, operation, permission).await {
            return user_has_not_permission(&"edit empresa")
        };

        let (empresa_id, _usr_id) = path.into_inner();

        match crate::admin::repo::abrir_empresa_one(&app.database.conn, &empresa_id).await {
            Ok(empresa) => Ok(HttpResponse::Ok().json(empresa)),
            Err(e) => Ok(HttpResponse::BadRequest().json(e.to_string())),
        }
    }

    /// Exibe uma lista de empresas
    #[get("/empresa/{empresa_id}/{user_id}")]
    pub async fn get_empresas_all(
        app: web::Data<AppState>,
        session: Session,
        // body: web::Json<String>,
        path: web::Path<(String, String)>,
        _req: HttpRequest,
    ) -> impl Responder {


        let pool = &app.database.conn;
        let (operation, permission) = (UserOperation::View, UserPermission::Empresa);
        if !has_permission(pool, &session, operation, permission).await {
            return user_has_not_permission(&"view empresa")
        };

        let (_empresa_id, _usr_id) = path.into_inner();

        match crate::admin::repo::listar_empresas_all(&app.database.conn,).await {
            Ok(lista_empresas) => Ok(HttpResponse::Ok().json(lista_empresas)),
            Err(e) => Ok(HttpResponse::BadRequest().json(e.to_string())),
        }
    }
    
    #[put("/empresa/{empresa_id}/{user_id}")]
    pub async fn put_empresa(
        app: web::Data<AppState>,
        session: Session,
        empresa: web::Json<PutEmpresa>,
        path: web::Path<(String, String)>,
        _req: HttpRequest,
    ) -> impl Responder {

        let pool = &app.database.conn;
        let (operation, permission) = (UserOperation::Edit, UserPermission::Empresa);
        if !has_permission(pool, &session, operation, permission).await {
            return user_has_not_permission(&"edit empresa")
        };

        let (_empresa_id, _usr_uuid) = path.into_inner();

        match crate::admin::service::atualizar_empresa(pool, &empresa).await {
            Ok(empresa) => Ok(HttpResponse::Ok().json(empresa)),
            Err(e) => Ok(HttpResponse::BadRequest().json(e.to_string())),
        }
    }
    
    #[delete("/empresa/{empresa_id}/{user_id}")]
    pub async fn delete_empresa(
        app: web::Data<AppState>,
        session: Session,
        _body: web::Bytes,
        path: web::Path<(String, String)>,
        _req: HttpRequest,
    ) -> impl Responder {

        let pool = &app.database.conn;
        let (operation, permission) = (UserOperation::Delete, UserPermission::Empresa);
        if !has_permission(pool, &session, operation, permission).await {
            return user_has_not_permission(&"delete empresa")
        };

        let (empresa_id, _usr_uuid) = path.into_inner();

        match crate::admin::service::excluir_empresa(app, empresa_id).await {
            Ok(empresa) => Ok(HttpResponse::Ok().json(empresa)),
            Err(e) => Ok(HttpResponse::BadRequest().json(e.to_string())),
        }
    }
    
        // Define as rotas para o controlador de autenticação
}

use controller::{get_empresa, put_empresa, post_empresa, delete_empresa,};

pub fn routes(cfg: &mut crate::web::ServiceConfig) {
cfg.service(
    crate::web::scope("/admin")
        .service(get_empresa)
        .service(put_empresa)
        .service(post_empresa)
        .service(delete_empresa)
    );
}