use actix_session::Session;
use actix_web::web::Path;
use actix_web::{get, post, web, HttpResponse, Responder};
use log::info;
use minijinja::context;
use crate::infra::sessao_usuario;
use crate::models::produto::FormProduto;
use crate::models::{self as model, QueryFiltro};
use crate::app::AppState;
use crate::services::produto as service;
use crate::repository::produto as repo;
use crate::infra::jwt::jwt_secret;
use crate::services::redireciona_login;

#[get("/produto/edit")]
async fn web_produto(
        data: web::Data<AppState>,
        query: web::Query<model::QueryId>, 
        session: Session,

    ) -> impl Responder {

    let sessao_usuario = sessao_usuario::Sessao::from_session(&session, &jwt_secret()).unwrap();
    
    // aqui voce faz todo tipo de verificação 
    if let Some(_usuario_logado) = sessao_usuario.clone() {
        
        // Usuário admin
        // if usuario_logado.is_admin {

        // }   


    } else {

        // Como esta rota requer login, então redireciona
        return redireciona_login();
    };
        
    let pool = &data.database;
    let tmpl = data.render.get_template("web/produto.html").unwrap();
    let web::Query(entidade_produto) = query;
    let produto = service::abrir_produto(pool, entidade_produto.id).await;

    if let Some(produto) = produto {
        let rendered = tmpl.render(context! {title => "produto", produto}).unwrap();

        HttpResponse::Ok()
            .content_type("text/html")
            .body(rendered)
    } 
    else {     
        
        let tmpl = data.render.get_template("shared/views/ajaxToast.html").unwrap();
        let rendered = tmpl.render(context! {
            toast_icon => "bi-x-circle-fill",
            toast_class => "toast-error",
            toast_text => "Produto não encontrado!",
    }).unwrap();
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)

    }
    
}

#[post("/produto/edit")]
async fn web_produto_submit(
    form: web::Form<FormProduto>,
    data: web::Data<AppState>,

) -> impl Responder {
    
    info!("Recebido POST com dados: {:?}", form);

    let tmpl = data.render.get_template("shared/views/ajaxToast.html").unwrap();
    let rendered = tmpl.render(context! {
        toast_icon => "bi-check-circle",
        toast_class => "toast-success",
        toast_text => "Dados salvos com sucesso!",
    }).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)

    // HttpResponse::Ok()
    //     .content_type("application/json")
    //     .json(json!({
    //         "toast": "teste"
    //     }))
}

#[get("/produto/json/{id}")]
async fn json_produto(
        data: web::Data<AppState>,
        path: Path<String>,

    ) -> impl Responder {
        
    let id = path.into_inner();
    let pool = &data.database;
    
    let produto = service::abrir_produto(pool, id).await;

    if let Some(produto) = produto {

        HttpResponse::Ok()
            .content_type("application/json")
            .json(produto)
    } 
    else {     
        
    HttpResponse::NotFound()
        .finish()

    }
    
}

#[get("/produtos/json")]
async fn json_all_produto(
        data: web::Data<AppState>,
        // path: Path<String>,
        query: web::Query<QueryFiltro>

    ) -> impl Responder {
        
    // let id = path.into_inner();
    let pool = &data.database;
    let web::Query(query) = query;
    
    let produto = repo::abrir_lista_produtos(pool, &query).await;

    if let Ok(produto) = produto {

        HttpResponse::Ok()
            .content_type("application/json")
            .json(produto)
    } 
    else {     
        
    HttpResponse::NotFound()
        .finish()

    }
    
}

