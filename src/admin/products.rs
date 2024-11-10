use actix_session::Session;
use actix_web::web::Path;
use actix_web::{get, post, web, HttpResponse, Responder};
use log::{debug, error, info};
use minijinja::context;
use reqwest::StatusCode;
use serde_json::json;
use crate::app::AppState;
use crate::infra::jwt::jwt_secret;
use crate::infra::toast::{ApiResponse, Toast};
use crate::models::produto::FormProduto;
use crate::models::QueryFiltro;
use crate::repository::{self, dashboard as repo_menus};
use crate::services::produto as service;
use crate::infra::sessao_usuario::Sessao;

/// Exibe uma lista de produtos
#[get("/produtos")]
async fn products_index(
    data: web::Data<AppState>,
    filtro: web::Query<QueryFiltro>,
) -> impl Responder {
    let pool = &data.database;
    let find_menus = repo_menus::carregar_menus(&pool).await;
    let menus = match find_menus {
        Ok(menus) => {
            menus
        }
        Err(_) => {
            vec!()
        }
    };
    let tmpl = data.render.get_template("admin/products/index.html").unwrap();

    let filtro = filtro.into_inner();
    let produtos = service::abrir_lista_produtos(pool, filtro).await;

    let rendered = tmpl.render(context! {
        title => "Produtos",
        active_menu => "produtos",
        menus,
        produtos
    }).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

/// Abre formulário que insere um produto
#[get("/produtos/novo")]
async fn new_product(
    data: web::Data<AppState>
) -> impl Responder {
    let pool = &data.database;
    let find_menus = repo_menus::carregar_menus(&pool).await;
    let menus = match find_menus {
        Ok(menus) => {
            menus
        }
        Err(_) => {
            vec!()
        }
    };
    let tmpl = data.render.get_template("admin/products/create.html").unwrap();

    let rendered = tmpl.render(context! {
        title => "Produtos",
        active_menu => "produtos",
        menus
    }).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

#[get("/produto/editar/{id}")]
async fn product_edit(
    data: web::Data<AppState>,
    path: Path<String>,
    session: Session,

) -> impl Responder {

    let sessao_usuario = Sessao::from_session(&session, &jwt_secret()).unwrap();
    
    // aqui voce faz todo tipo de verificação 
    if let Some(usuario_logado) = sessao_usuario.clone() {
        
        // Usuário admin
        if usuario_logado.is_admin {

        } else {

        // Como esta rota requer nível admin, então redireciona
        // return redireciona_loja();
        };

    } else {

        // Como esta rota requer login, então redireciona
        // return redireciona_login();
    };
        
    let pool = &data.database;
    let find_menus = repo_menus::carregar_menus(&pool).await;
    let menus = match find_menus {
        Ok(menus) => {
            menus
        }
        Err(_) => {
            vec!()
        }
    };
    let tmpl = data.render.get_template("admin/products/edit.html").unwrap();
    let id = path.into_inner();
    let produto = service::abrir_produto(pool, id).await;

    let rendered = tmpl.render(context! {
        title => "Produtos",
        active_menu => "produtos",
        menus,
        produto
    }).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

#[post("/produto/editar/{id}")]
async fn web_produto_submit(
    form: web::Form<FormProduto>,
    data: web::Data<AppState>,
    path: web::Path<String>,

) -> impl Responder {
    let pool = &data.database;
    let id = path.into_inner();

    info!("Recebido POST com dados: {:?}", form);
    let produto = repository::produto::atualizar_produto(pool, &id, form).await;
    match produto {
        Ok(produto) => {
            let toast = Toast::with_status(StatusCode::ACCEPTED, "produto atualizado com sucesso");
            debug!("{:?}", toast);

            ApiResponse::new()
            .with_data(json!(produto))
            .with_toast(toast)
            .send()
        },
        Err(err) => {
            error!("{}", err);
            err.into()
        },
    }

}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(products_index)
        .service(new_product)
        .service(product_edit);
}