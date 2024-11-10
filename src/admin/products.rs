use actix_session::Session;
use actix_web::web::Path;
use actix_web::{get, put, patch, web, HttpResponse, Responder};
use log::info;
use minijinja::context;
use crate::app::AppState;
use crate::infra::jwt::jwt_secret;
use crate::infra::toast::Toast;
use crate::models::produto::FormProduto;
use crate::models::QueryFiltro;
use crate::repository::dashboard as repo_menus;
use crate::services::produto as service;
use crate::infra::sessao_usuario::Sessao;

/// Exibe uma lista de produtos
#[get("/produtos")]
async fn admin_products_index(
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
async fn admin_new_product(
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


#[get("/produto/{id}/editar")]
async fn admin_product_edit(
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

#[put("/produto/{id}/atualizar")]
async fn admin_product_update(
    form: web::Form<FormProduto>,
    data: web::Data<AppState>,

) -> impl Responder {
    
    info!("Recebido POST com dados: {:?}", form);

    let _tmpl = data.render.get_template("resources/components/ajaxToast.html").unwrap();

    Toast::created("Produto inserido com sucesso")

}

#[patch("/produto/{id}/atualiza/imagem")]
async fn admin_product_update_image(
    form: web::Form<FormProduto>,
    data: web::Data<AppState>,

) -> impl Responder {

    info!("Recebido PUT com dados: {:?}", form);

    let _tmpl = data.render.get_template("resources/components/ajaxToast.html").unwrap();

    Toast::created("Imagem atualizada com sucesso...")

}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(admin_products_index)
        .service(admin_new_product)
        .service(admin_product_edit)
        .service(admin_product_update)
        .service(admin_product_update_image);
}