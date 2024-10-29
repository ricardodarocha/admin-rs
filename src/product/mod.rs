pub mod controller {
    use std::net::IpAddr;

    use actix_session::Session;
    use actix_web::{get, post, put, Responder};  
    use actix_web::{web, HttpRequest, HttpResponse}; 
    use minijinja::context;
    use crate::app::AppState;
    use crate::land::model::Menu;
    use crate::land::repo::get_menus;
    use crate::product::service as service;
    use crate::product::repo::{self as repo, lista_grupos_produtos};
    use crate::auth::model::{UserOperation, UserPermission};
    use crate::auth::session::{get_user, has_logged, has_permission, user_has_not_permission};
    use crate::infra::models::Colunas;
    use crate::product::model::*;
    use actix_web::http::header::LOCATION;
    use crate::auditoria::service::*;
    use crate::admin::repo::abrir_empresa_one;

    /// Formulário de cadastro de produtos
    #[utoipa::path(
        responses(
            (status = 200, description = "Produto")
    ))]
    #[get("/{id}")]
    pub async fn produto_form(
        _req: HttpRequest,
        session: Session,
        path: web::Path<String>,
        data: web::Data<AppState>,

        ) -> impl Responder {

        dbg!("GET produto/{id}  -> produto_form");
        let pool = &data.database.conn;
        let current_id = path.into_inner();
        // let url_for = format!("{}/", std::env::var("SITE").unwrap());

        if !has_logged(pool, &session).await {
             return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/login"))
            .finish())
        };

        let (operation, permission) = (UserOperation::Edit, UserPermission::Produto);
        if !has_permission(pool, &session, operation, permission).await {
            return user_has_not_permission(&"edit produto")
        };
        let usuario = get_user(pool, &session).await;
        let id_empresa = usuario.clone().unwrap().id_empresa.clone().unwrap().to_string();

        let found_produto: Option<Produto> = repo::abrir_produto(pool, id_empresa.clone(), &current_id.clone()).await;
        let grupos_produto = repo::lista_grupos_produtos(pool, id_empresa).await.unwrap();
        
        let flash = session.remove("flash").unwrap_or("".to_string()); 
        let msg_error = format!("{}", session.remove("msg_error").unwrap_or("".to_string()));  
        
        let menus: Vec<Menu> = 
            match usuario.clone() {
                Some(usuario) => get_menus(pool, usuario.id, "produto").await.unwrap(),
                None => vec![],
        }; 

        if let Some(form) = found_produto {
            crate::infra::render::render_minijinja("produto/form_produto.html", context!(
                menus,
                grupos_produto, 
                usuario, 
                current_id, 
                form, 
                flash, 
                msg_error)) 
        } else
        {   
            let form = PostProduto::default();
            crate::infra::render::render_minijinja("produto/form_produto.html", context!(
                menus,
                grupos_produto, 
                usuario, 
                current_id, 
                form, 
                flash, 
                msg_error))
        } 
    }

    /// Lista de todos os produtos
    #[utoipa::path(
        responses(
            (status = 200, description = "Lista de produtos")
    ))]
    #[get("/all")]
    pub async fn produto_list(
        _req: HttpRequest,
        session: Session,
        // path: web::Path<(String, String)>,
        data: web::Data<AppState>,
        pagination: web::Query<ProdutoPagination>,

        ) -> impl Responder {

        dbg!("GET /produto_list -> ");
        let pool = &data.database.conn;
        // let url_for = format!("{}/", std::env::var("SITE").unwrap());

        if !has_logged(pool, &session).await {
             return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/login"))
            .finish())
        };

        let (operation, permission) = (UserOperation::Edit, UserPermission::Produto);
        if !has_permission(pool, &session, operation, permission).await {
            return user_has_not_permission(&"edit produto")
        };

        let web::Query(pagination) = pagination;
        let usuario = get_user(pool, &session).await;
        let id_empresa = usuario.clone().unwrap().id_empresa;
        let empresa = abrir_empresa_one(pool, &id_empresa.clone()).await.unwrap();
        let categorias = match id_empresa {
            Some(empresa) => { lista_grupos_produtos(pool, empresa).await.unwrap() },
            None => { vec!()},
        };


        let grade = repo::lista_produtos(pool, usuario.clone().unwrap().id_empresa.clone().unwrap(), pagination).await.unwrap();
        let colunas = Colunas::new(vec!["id", "nome", "códiogo de barras", "preço", "unidade", "estoque"]);
        let flash = session.remove("flash").unwrap_or("".to_string()); 
        let msg_error = format!("{}", session.remove("msg_error").unwrap_or("".to_string()));  
        
        let menus: Vec<Menu> = 
            match usuario.clone() {
                Some(usuario) => get_menus(pool, usuario.id, "produto").await.unwrap(),
                None => vec![],
        }; 

        crate::infra::render::render_minijinja("produto/produto_lista.html", context!(
            menus, 
            usuario, 
            categorias,
            empresa,
            colunas, 
            grade, 
            flash, 
            msg_error)) 
    }

    /// Salvar o produto
    #[utoipa::path(
        responses(
            (status = 200, description = "Salvar produto")
    ))]
    #[post("/{id}")]
    pub async fn post_produto(
        req: HttpRequest,
        produto_body: web::Form<PostProduto>,
        path: web::Path<String>,
        data: web::Data<AppState>,
        session: Session,

        ) -> impl Responder {

        let pool = &data.database.conn;
        let id = path.into_inner();
        let user = get_user(pool, &session).await.unwrap();

        let scope = "".to_string();
        let id_usuario = user.clone().id;
        let id_empresa = user.clone().id_empresa.unwrap();

       //audita

        let _ = auditar_requisicao(
        pool,
        req.clone(),
        scope,
        &id_empresa,
        &id_usuario,
    ).await ;

       let ip: Option<IpAddr> = if let Some(val) = req.clone().peer_addr() {
            Some(val.ip())
        } else 
        {
            None
        };
        
        let _ = crate::auth::repo::inserir_consumo_rota(
            &"POST produto/".to_owned(), 
            &"SUCESSO".to_owned(), 
            &format!("{:?}", ip), 
            &user.id.clone())
        .await;

        let produto = repo::abrir_produto(pool, id_empresa, &id).await;
        match produto {
            Some(_value) => {
                 service::atualizar_produto(
                    pool, 
                    Some(id),
                    user.id_empresa.unwrap(),
                    &produto_body,
                    session     
                ).await
            },
            None => {
                service::inserir_produto(
                    pool, 
                    Some(id),
                    user.id_empresa.unwrap(),
                    &produto_body,
                    session
                ).await
            },
        }
    }
 
    /// Atualizar Produto
    #[utoipa::path(
        responses(
            (status = 200, description = "Atualizar produto")
    ))]
    #[put("/{id}")]
    pub async fn put_produto(
        req: HttpRequest,
        path: web::Path<String>,
        produto: web::Form<PutProduto>,
        data: web::Data<AppState>,
        session: Session, 

        ) -> HttpResponse {
        
        let pool = &data.database.conn;
        let id = path.into_inner();
        let user = get_user(pool, &session).await.unwrap();
        let id_usuario = user.clone().id;
        let id_empresa = user.clone().id_empresa.unwrap();

        //audita
        let scope = "".to_string();
        let _ = auditar_requisicao(
            pool,
            req.clone(),
            scope,
            &id_empresa,
            &id_usuario,
        ).await ;
        
        if id != "0".to_string() { 
            service::alterar_produto(pool, id_empresa, &produto, session).await.unwrap() }
        else {
           let status_code = actix_web::http::StatusCode::NOT_MODIFIED;            
           HttpResponse::build(status_code).body("Código INDEFINIDO não pode ser alterado") 
        }
    }

    /// Lista de produtos
    #[utoipa::path(
        responses(
            (status = 200, description = "Lista de produtos")
    ))]
    #[get("/json/{id}")]
    pub async fn json_produto(
        _req: HttpRequest,
        session: Session,
        path: web::Path<String>,
        data: web::Data<AppState>,
        // filtro_data: web::Query<FiltroData>,

        ) -> impl Responder {
        
        let pool = &data.database.conn;

        let id = path.into_inner();
        dbg!("GET /produto/{id} -> ");
        // let url_for = format!("{}/", std::env::var("SITE").unwrap());

        if !has_logged(&data.database.conn, &session).await {
             return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/login"))
            .finish())
        };
        
        let (operation, permission) = (UserOperation::Edit, UserPermission::Produto);
        if !has_permission(pool, &session, operation, permission).await {
            return user_has_not_permission(&"edit produto")
        };

        let usuario = get_user(pool, &session).await.unwrap();
        let id_empresa = usuario.id_empresa.unwrap();

        let result = repo::abrir_produto(pool, id_empresa, &id).await.unwrap();
        Ok(HttpResponse::Ok().json(result))
    }


    /// Lista de produtos
    #[utoipa::path(
        responses(
            (status = 200, description = "Lista de produtos")
    ))]
#[get("/json")]
    pub async fn json_lista_produtos(
        _req: HttpRequest,
        session: Session,
        // path: web::Path<(String, String)>,
        data: web::Data<AppState>,
        // filtro_data: web::Query<FiltroData>,

        ) -> impl Responder {

        let pool = &data.database.conn;

        dbg!("GET /produto/all -> ");
         
         if !has_logged(&data.database.conn, &session).await {
             return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/login"))
            .finish())
        };

        let (operation, permission) = (UserOperation::View, UserPermission::Produto);
        if !has_permission(pool, &session, operation, permission).await {
            return user_has_not_permission(&"view produto")
        };
        
        let usuario = get_user(pool, &session).await.unwrap();
        let id_empresa = usuario.id_empresa.unwrap();
        let result = repo::lista_produtos(pool, id_empresa, ProdutoPagination::setup(1, 9999999) ).await.unwrap();
        Ok(HttpResponse::Ok().json(result))
    }
}

use controller::*;

// Define as rotas para o controlador de autenticação
pub fn routes(cfg: &mut crate::web::ServiceConfig)  {
    cfg.service(
        crate::web::scope("/produto")
            .service(json_lista_produtos)
            .service(json_produto)
            .service(produto_list)
            .service(produto_form)
            .service(post_produto)
            // .service(put_produto)
    );
    
}