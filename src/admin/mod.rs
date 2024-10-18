pub mod service;
pub mod repo;
pub mod model;
pub mod view;

pub mod controller {
    use actix_session::Session;
    use actix_web::{web, HttpRequest, HttpResponse, Responder};
    use actix_web::{post, get,  put, delete};
    use log::info;
    use minijinja::context;
    // use serde_json::json;
    use crate::admin::model::{DadosAccount, PostAccount, PostEmpresa, PutEmpresa};
    use crate::admin::{repo, service};
    use crate::app::AppState;
    use crate::auth::model::{UserOperation, UserPermission};  
    // use actix_web::http::header::LOCATION;
    use crate::auth::session::{get_user, has_logged, has_permission, user_has_not_permission};
    use crate::land::model::Menu;
    use crate::land::repo::get_menus;
    use actix_web::http::header::LOCATION;

    /// Empresa
    #[utoipa::path(
        responses(
            (status = 200, description = "Empresa")
    ))]
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

    /// Empresa
    #[utoipa::path(
        responses(
            (status = 200, description = "Empresa")
    ))]
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

        match crate::admin::repo::abrir_empresa_one(&app.database.conn, &Some(empresa_id)).await {
            Ok(empresa) => Ok(HttpResponse::Ok().json(empresa)),
            Err(e) => Ok(HttpResponse::BadRequest().json(e.to_string())),
        }
    }

    /// Lista de empresas
    #[utoipa::path(
        responses(
            (status = 200, description = "Lista de empresas")
    ))]
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

        let (_empresa_id, id_usuario) = path.into_inner();

        match crate::admin::repo::listar_empresas_all(&app.database.conn, &id_usuario,).await {
            Ok(lista_empresas) => Ok(HttpResponse::Ok().json(lista_empresas)),
            Err(e) => Ok(HttpResponse::BadRequest().json(e.to_string())),
        }
    }

    /// Alterar empresa
    #[utoipa::path(
        responses(
            (status = 200, description = "Alterar empresa")
    ))]
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

        let (id_empresa, _usr_uuid) = path.into_inner();

        match crate::admin::service::atualizar_empresa(&pool.clone(), &empresa, &id_empresa).await {
            Ok(empresa) => Ok(HttpResponse::Ok().json(empresa)),
            Err(e) => Ok(HttpResponse::BadRequest().json(e.to_string())),
        }
    }

    /// Excluir Empresa
    #[utoipa::path(
        responses(
            (status = 200, description = "Excluir empresa")
    ))]
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

    /// Alterar os dados do usuário
    #[utoipa::path(
        responses(
            (status = 200, description = "Usuário")
    ))]
    #[get("/account")]
    pub async fn usuario_form(
        _req: HttpRequest,
        session: Session,
        // path: web::Path<String>,
        data: web::Data<AppState>,

        ) -> impl Responder {

        dbg!("GET admin/account  -> form_usuario");
        let pool = &data.database.conn;
        // let current_id = path.into_inner();
        // let url_for = format!("{}/", std::env::var("SITE").unwrap());

        if !has_logged(pool, &session).await {
             return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/login"))
            .finish())
        };

        //permissao de cadastrar usuario, vai habilitar o botao cadastrar usuario
        // let (operation, permission) = (UserOperation::Edit, UserPermission::Produto);
        // if !has_permission(pool, &session, operation, permission).await {
        //     return user_has_not_permission(&"edit produto")
        // };
        let usuario = get_user(pool, &session).await;
        
        let flash = session.remove("flash").unwrap_or("".to_string()); 
        let msg_error = format!("{}", session.remove("msg_error").unwrap_or("".to_string()));  
        
        let menus: Vec<Menu> = 
            match usuario.clone() {
                Some(usuario) => get_menus(pool, usuario.id, "usuário").await.unwrap(),
                None => vec![],
        }; 

        let segmentos = crate::admin::repo::lista_segmentos(pool).await.unwrap();
        let estados = crate::cidade::repo::lista_estados(pool).await.unwrap();
   
        let id_usuario = usuario.clone().unwrap().id;
        info!("Buscando empresa do usuário {}", id_usuario.clone());
        let found_empresa =  crate::admin::repo::abrir_dados_empresa_principal(pool, id_usuario).await;
        info!("empresa encontrada {:?}", found_empresa.clone());
        let account_form = found_empresa;
        let form = if let Some(form) = account_form {
            form
        } else {
            DadosAccount::default()
                .with_user(usuario.clone())
        };

        // exemplo de menu 
        // usuario
        //     alterar senha
        //     excluir conta

        crate::infra::render::render_minijinja("admin/form_usuario.html", context!(
            menus,
            usuario, 
            segmentos,
            estados,
            form, 
            flash, 
            msg_error)) 
    }

    /// Salvar os dados da empresa
    #[utoipa::path(
        responses(
            (status = 200, description = "Salvar account")
    ))]
    #[post("/account/{id}")]
    pub async fn post_account(
        _req: HttpRequest,
        account_body: web::Form<PostAccount>,
        path: web::Path<String>,
        data: web::Data<AppState>,
        session: Session,
    ) -> HttpResponse {
    
        let pool = &data.database.conn;
        let id = path.into_inner();
        info!("{:?}", account_body.clone());
        let user = get_user(pool, &session).await.unwrap();
    
        let _scope = "".to_string();
        let _id_usuario = user.clone().id;
        let id_empresa = user.clone().id_empresa;

        let web::Form(account_body) = account_body;
    
        // Audita
        // let _ = auditar_requisicao(
        //     pool,
        //     req.clone(),
        //     scope,
        //     &id_empresa,
        //     &id_usuario,
        // ).await;
    
        // let ip: Option<IpAddr> = if let Some(val) = req.clone().peer_addr() {
        //     Some(val.ip())
        // } else {
        //     None
        // };
    
        // let _ = crate::auth::repo::inserir_consumo_rota(
        //     &"POST account/".to_owned(),
        //     &"SUCESSO".to_owned(),
        //     &format!("{:?}", ip),
        //     &user.id.clone()
        // ).await;
    
        let account = repo::abrir_empresa_one(pool, &id_empresa).await.unwrap();
        if let Some(_account) = account {
            let res = service::atualizar_account(
                pool,
                &account_body.into(),
                &id_empresa.unwrap(),
            ).await;
            match res {
                Ok(empresa) =>  { HttpResponse::Ok().json(empresa) 
            }, Err(err) => {
            HttpResponse::BadRequest().json(format!("A empresa já existe. Erro ao atualizar: {}", err)) }}
        
        } else {
            let res = service::inserir_account(
                    pool,
                    id,
                    &account_body,
                ).await; 
                
         match res {
                Ok(empresa) =>  { HttpResponse::Ok().json(empresa) 
            }, Err(err) => {
            HttpResponse::BadRequest().json(format!("Falha ao incluir a empresa {}", err)) }
            
        }
        
    }}
}

use controller::*;

pub fn routes(cfg: &mut crate::web::ServiceConfig) {
cfg.service(
    crate::web::scope("/admin")
        .service(usuario_form)
        .service(post_account)
        .service(get_empresa)
        .service(put_empresa)
        .service(post_empresa)
        .service(delete_empresa)
    );
}