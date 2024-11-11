use actix_session::Session;

use actix_web::web::Path;
use log::{debug, error, info};
use actix_web::{get, put, patch, web, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures_util::StreamExt;
use serde_json::json;
use minijinja::context;
use reqwest::StatusCode;
use crate::app::AppState;
use crate::infra::error::Error;
use crate::infra::multimidia::storage;
use crate::infra::toast::{ApiResponse, Toast};
use crate::models::produto::FormProduto;
use crate::models::QueryFiltro;
use crate::repository::{self, dashboard as repo_menus};
use crate::services::produto as service;

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
    _session: Session,
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
    path: web::Path<String>,

) -> impl Responder {
    let pool = &data.database;
    let id = path.into_inner();

    info!("Recebido PUT com dados: {:?}", form);
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


#[patch("/produto/{id}/atualizar/imagem")]
async fn admin_product_update_image(
    mut payload: Multipart,
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    info!("Recebido PATCH com multimídia");
    let id = path.into_inner();
    let pool = &data.database;

    while let Some(field) = payload.next().await {
        
        let field = match field {
            Ok(field) => field,
            Err(err) => return ApiResponse::from_error(Error::Simple(format!("{}", err))).send(),
        };

        if field.name() == Some(&"avatar") {
            let storage_info = storage::FileStorage::salvar_imagem(field).await.unwrap();
            info!("{:?}", storage_info.clone());
            let resource_name = format!("{}/{}_1024w.{}", storage_info.path, storage_info.avatar, storage_info.extension);
            let _ = sqlx::query!("update produto set avatar = $1 where id = $2", resource_name, id)
                .execute(pool).await;
            for media in storage_info.resources.iter() {
                info!("🖼 File saved {}", media)
            }
            // Write the file content to the file
            // while let Some(chunk) = field.next().await {
            //     let chunk = match chunk {
            //         Ok(chunk) => chunk,
            //         Err(e) => return Err(actix_web::error::ErrorBadRequest(e.to_string()))
            //     };

            //     let _ = file.write_all(&chunk).await?;
            // }
        }
    }

    let _tmpl = data.render.get_template("components/ajaxToast.html").unwrap();

    let rendered = _tmpl.render(context! {
                toast_type => "toast-success",
                toast_icon => "bi-check-circle-fill",
                toast_text =>"Imagem recebida"
            }).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({"reset":true, "toast": rendered}))
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(admin_products_index)
        .service(admin_new_product)
        .service(admin_product_edit)
        .service(admin_product_update)
        .service(admin_product_update_image);
}