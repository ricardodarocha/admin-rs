pub mod endereco;
pub mod service;
pub mod repo;
pub mod model;
pub mod view;

pub mod controller {
    use actix_session::Session;
    use actix_web::{web, HttpRequest, HttpResponse, Responder};
    // use actix_web::Error;
    use actix_web::{post, get,}; //put  delete
    use minijinja::context;
    use crate::admin::repo::abrir_empresa_one;
    use crate::auth::model::{UserOperation, UserPermission};
    use crate::auth::session::{get_user, has_logged, has_permission, user_has_not_permission};
    use crate::cidade::repo::lista_estados;
    use crate::infra::models::Colunas;
    use crate::land::model::Menu;
    use crate::land::repo::get_menus;
    use crate::pessoa::model::{Pessoa, PessoaPagination, PostPessoa};
    use crate::app::AppState;
    use actix_web::http::header::LOCATION;
    use crate::pessoa::repo::{self as repo, lista_grupos_pessoas};

    #[get("/{id}")]
    pub async fn pessoa_form(
        _req: HttpRequest,
        session: Session,
        path: web::Path<String>,
        data: web::Data<AppState>,

) -> impl Responder {

    dbg!("GET pessoa/{id} -> pessoa_form");
    let pool = &data.database.conn;
    let current_id = path.into_inner();

    if !has_logged(pool, &session).await {
        return Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/login"))
        .finish())
        };
    let (operation, permission) = (UserOperation::Edit, UserPermission::Contato);
        if !has_permission(pool, &session, operation, permission).await {
        return user_has_not_permission(&"edit contato")
        };
    let usuario = get_user(pool, &session).await;
    let id_empresa = usuario.clone().unwrap().id_empresa.clone().unwrap().to_string();
    let empresa = abrir_empresa_one(pool, &id_empresa.clone()).await.unwrap();

    let found_pessoa: Option<Pessoa> = repo::abrir_pessoa(pool, id_empresa.clone(), &current_id.clone()).await;
    let estados = lista_estados(pool).await.unwrap();
    let flash = session.remove("flash").unwrap_or("".to_string());
    let msg_error = format!("{}", session.remove("msg_error").unwrap_or("".to_string()));
    let menus: Vec<Menu> =
      match usuario.clone() {
        Some(usuario) => get_menus(pool, usuario.id,
        "pessoa").await.unwrap(),
        None => vec![],
    };

    if let Some(form) = found_pessoa {
      crate::infra::render::render_minijinja(
      "pessoa/form_pessoa.html", context!(
        menus,
        estados,
        usuario,
        current_id,
        form,
        flash,
        msg_error))
    } else
    {  
	    let form = PostPessoa::default();
      crate::infra::render::render_minijinja("pessoa/form_pessoa.html", context!(
        menus,
        // < OTHER ENTITIES >
        usuario,
        empresa,
        current_id,
        form,
        flash,
        msg_error))
    }
  }
    
    /// Insere uma pessoa
    #[post("/pessoa/{empresa_id}/{user_id}")]
    pub async fn post_pessoa(
        app: web::Data<AppState>,
        _session: Session,
        pessoa: web::Json<PostPessoa>,
        path: web::Path<(String, String)>,
        _req: HttpRequest,
    ) -> impl Responder {
        let (_empresa_id, _usr_uuid) = path.into_inner();

        match crate::pessoa::service::inserir_pessoa(app, &pessoa).await {
            Ok(pessoa) => HttpResponse::Ok().json(pessoa),
            Err(e) => HttpResponse::BadRequest().json(e.to_string()),
        }
    }
    
    #[get("/pessoa/json/{pessoa_id}/{user_id}")]
    pub async fn get_pessoa(
        app: web::Data<AppState>,
        session: Session,
        // body: web::Json<String>,
        path: web::Path<(String, String)>,
        _req: HttpRequest,
    ) -> impl Responder {
        let (pessoa_id, _usr_id) = path.into_inner();
        let pool = &app.database.conn;

        let user = get_user(pool, &session).await.unwrap();

        let _scope = "".to_string();
        let _id_usuario = user.clone().id;
        let id_empresa = user.clone().id_empresa.unwrap();


        let result = repo::abrir_pessoa(pool, id_empresa, &pessoa_id).await.unwrap();
        HttpResponse::Ok().json(result)
    }
    
    // #[put("/pessoa/{pessoa_id}/{user_id}")]
    // pub async fn put_pessoa(
    //     app: web::Data<AppState>,
    //     session: Session,
    //     pessoa: web::Json<PutPessoa>,
    //     path: web::Path<(String, String)>,
    //     req: HttpRequest,
    // ) -> Result<impl Responder, Error> {
    //     let (pessoa_id, usr_uuid) = path.into_inner();

    //     match crate::admin::service::update_pessoa(app, &pessoa).await {
    //         Ok(pessoa) => Ok(HttpResponse::Ok().json(pessoa)),
    //         Err(e) => Ok(HttpResponse::BadRequest().json(e.to_string())),
    //     }
    // }
    
//     #[delete("/pessoa/{pessoa_id}/{user_id}")]
//     pub async fn delete_pessoa(
//         app: web::Data<AppState>,
//         session: Session,
//         body: web::Bytes,
//         path: web::Path<(String, String)>,
//         req: HttpRequest,
//     ) -> Result<impl Responder, Error> {

    #[get("/all")]
    pub async fn list_pessoa(
        _req: HttpRequest,
        session: Session,
        // path: web::Path<(String, String)>,
        data: web::Data<AppState>,
        args: web::Query<PessoaPagination>,

        ) -> impl Responder {

        dbg!("GET /pessoa_list -> ");
        let pool = &data.database.conn;
        // let url_for = format!("{}/", std::env::var("SITE").unwrap());

        if !has_logged(pool, &session).await {
             return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/login"))
            .finish())
        };

        let (operation, permission) = (UserOperation::View, UserPermission::Contato);
        if !has_permission(pool, &session, operation, permission).await {
            return user_has_not_permission(&"view contato")
        };

        let web::Query(args) = args;
        let usuario = get_user(pool, &session).await;
        let id_empresa = usuario.clone().unwrap().id_empresa;
        let empresa = abrir_empresa_one(pool, &id_empresa.clone().unwrap()).await.unwrap();
        let categorias = match id_empresa.clone() {
            Some(empresa) => { lista_grupos_pessoas(pool, empresa).await.unwrap() },
            None => { vec!()},
        };
  
        let grade = repo::listar_pessoas_all(pool, id_empresa.clone().unwrap(), args).await.unwrap();
   
        let colunas = Colunas::new(vec!["id", "nome", "razão social", "telefone", "email"]);
        let flash = session.remove("flash").unwrap_or("".to_string()); 
        let msg_error = format!("{}", session.remove("msg_error").unwrap_or("".to_string()));  
        
        let menus: Vec<Menu> = 
            match usuario.clone() {
                Some(usuario) => get_menus(pool, usuario.id, "contato").await.unwrap(),
                None => vec![],
        }; 

        crate::infra::render::render_minijinja("pessoa/pessoa_lista.html", context!(
            menus, 
            usuario, 
            categorias,
            empresa,
            colunas, 
            grade, 
            flash, 
            msg_error)) 
    }
  
}

use controller::*;

    // Define as rotas para o controlador de autenticação
    
    pub fn routes(cfg: &mut crate::web::ServiceConfig) {
    cfg.service(
        crate::web::scope("/contato") 
            .service(list_pessoa)
            .service(pessoa_form)
            .service(get_pessoa)
            // .service(put_pessoa)
            .service(post_pessoa)
            // .service(delete_pessoa)
        );
    }


