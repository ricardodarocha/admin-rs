pub mod model;
pub mod repo;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
#[derive(Default, Debug, Serialize, sqlx::Type, Deserialize, FromRow)]
// #[sqlx(type_name = "Produto")]
pub struct Produto {
    pub id: String,
    pub nome: String,
    pub preco: String,
}

#[derive(Default, Debug, Serialize, Deserialize, FromRow)]
// #[sqlx(type_name = "Produto")]
pub struct CadastroProduto {
    pub id: String,
    pub nome: Option<String>,
    pub preco: Option<String>,
}

// impl FromRow<'_, PgRow> for Produto {
//     fn from_row(row: &PgRow) -> sqlx::Result<Self> {
//         let user = Produto {
//             id: row.get("id"),
//             nome: row.get("nome"),
//             preco: row.get("preco"),
//         };
//         Ok(user)
//     }
// }

pub mod controller {
    use actix_session::Session;
    // use actix_web::http::header::ContentType;
    use actix_web::{get, patch, post, put};
    use actix_web::{web, HttpRequest, HttpResponse, Responder};
    use log::info;
    //, http::StatusCode
    use actix_web::http::header::LOCATION;
    use minijinja::context;
    use serde::Deserialize;
    // use serde_json::Value;
    use crate::app::AppState;
    use crate::auth::model::SessionParser;
    use crate::infra::result::Result;
    use sqlx::Postgres;

    use crate::auth::session::{get_user, has_logged};
    use crate::land::model::Menu;
    use crate::land::repo::get_menus;
    use crate::pedido::model::{EntidadeItemPedido, FormItem, PedidoForm};
    use crate::pedido::repo::{abrir_item_pedido_one, get_galeria};

    #[derive(Deserialize)]
    pub struct FiltroCliente {
        pessoa: String,
    }

    #[get("/add")] // /add?pessoa=
    pub async fn pedido_add(
        // _req: HttpRequest,
        session: Session,
        // path: web::Path<(String, String)>,
        data: web::Data<AppState>,
        arg: web::Query<FiltroCliente>,
    ) -> Result<HttpResponse> {
        // let url_for = format!("{}/", std::env::var("SITE").unwrap());::model::PedidoForm;

        if !has_logged(&data.database.conn, &session).await {
            return Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, "/"))
                .finish());
        };

        let pool = &data.database.conn;
        let usuario = get_user(pool, &session).await.unwrap();
        let id_usuario = usuario.id;
        let id_empresa = usuario.id_empresa;
        let id_cliente = &arg.pessoa;

        let ultimo_pedido =
            super::repo::ultimo_pedido_editavel(pool, id_usuario.clone(), id_cliente.clone()).await;
        let ultimo_pedido = if let Some(id_pedido) = ultimo_pedido {
            id_pedido
        } else {
            super::repo::criar_pedido_editavel(
                pool,
                id_usuario,
                id_empresa.clone(),
                Some(id_cliente.clone()),
            )
            .await
            .unwrap()
        };

        let novo_pedido = ultimo_pedido.id;

        let url = format!("{}", novo_pedido);

        Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, url))
            .finish())
    }

    #[get("/{id}")]
    pub async fn pedido_form(
        _req: HttpRequest,
        session: Session,
        path: web::Path<String>,
        data: web::Data<AppState>,
        // filtro_data: web::Query<FiltroData>,
    ) -> Result<HttpResponse> {
        // let url_for = format!("{}/", std::env::var("SITE").unwrap());

        if !has_logged(&data.database.conn, &session).await {
            return Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, "/"))
                .finish());
        };

        let pool = &data.database.conn;
        let sessiondata = SessionParser::from(session.clone());
        info!(
            "usuario {id_usuario}",
            id_usuario = &sessiondata.id_usuario.clone()
        );

        let usuario = get_user(pool, &session).await.unwrap();
        // let id_usuario = usuario.id;
        // let id_empresa = usuario.id_empresa;
        let id_pedido = path.into_inner();
        let id_empresa = usuario.id_empresa.clone().unwrap();
        let id_usuario = usuario.id.clone();

        let form = PedidoForm::abrir(pool, id_pedido.clone(), id_empresa.clone()).await;
        let usuario = get_user(pool, &session).await;
        let menus: Vec<Menu> = match usuario.clone() {
            Some(usuario) => get_menus(pool, usuario.id, "pedido").await.unwrap(),
            None => vec![],
        };
        let galeria = get_galeria(
            pool,
            id_pedido.clone(),
            id_empresa.clone(),
            id_usuario.clone(),
        )
        .await;

        let flash = session.remove("flash").unwrap_or("".to_string());
        let msg_error = format!("{}", session.remove("msg_error").unwrap_or("".to_string()));

        crate::infra::render::render_minijinja(
            "pedido/form_pedido.html",
            context!(usuario, menus, galeria, form, flash, msg_error),
        )
    }

    #[post("up/{id_pedido}/{id_produto}/{id_item}")]
    pub async fn update_item(
        // body: String,
        session: Session,
        path: web::Path<(String, String, i32)>,
        data: web::Data<AppState>,
        form: web::Query<FormItem>,
    ) -> impl Responder {
        // let payload: Value = serde_json::from_str(body.as_str()).unwrap();
        // info!("{x}",x=payload.clone());
        // let current = payload["selected"].as_str().unwrap();
        // let value = payload[current].as_f64().unwrap() as f32;
        let pool = &data.database.conn;
        let sessiondata = SessionParser::from(session.clone());
        info!(
            "usuario {id_usuario}",
            id_usuario = &sessiondata.id_usuario.clone()
        );

        let _usuario = get_user(pool, &session).await.unwrap();
        // let id_usuario = usuario.id;
        // let id_empresa = usuario.id_empresa;
        let (id_pedido, id_produto, id_item) = path.into_inner();

        let item = crate::pedido::repo::get_item_pedido(
            pool,
            id_pedido.clone(),
            id_produto.clone(),
            id_item,
        )
        .await;

        let web::Query(form) = form;
        // let form = FormItem {
        //     quantidade: value,
        // };

        async fn update_item_pedido(
            pool: &sqlx::Pool<Postgres>,
            id_pedido: String,
            id_produto: String,
            id_item: i32,
            form: FormItem,
        ) -> EntidadeItemPedido {
            crate::pedido::repo::update_item_pedido(pool, id_pedido, id_produto, id_item, form)
                .await
        }

        async fn create_item_pedido(
            pool: &sqlx::Pool<Postgres>,
            id_pedido: String,
            id_produto: String,
            form: FormItem,
        ) -> EntidadeItemPedido {
            crate::pedido::repo::create_item_pedido(pool, id_pedido, id_produto, form).await
        }

        let item = match item {
            Some(value) => {
                update_item_pedido(pool, value.id_pedido, value.id_produto, value.id_item, form)
                    .await
            }
            None => create_item_pedido(pool, id_pedido, id_produto, form).await,
        };

        // response
        let item = abrir_item_pedido_one(pool, item.id_pedido, item.id_produto, item.id_item).await;
        crate::infra::render::render_minijinja(
            "comp/pedido/view_total_pedido_item.html",
            context!(item),
        )

        // crate::infra::render::render_minijinja("comp/pedido/pedido_item.html", context!(item))
    }

    #[get("up/{id_pedido}/{id_produto}/{id_item}")]
    pub async fn get_update_item(
        // body: String,
        // session: Session,
        path: web::Path<(String, String, i32)>,
        data: web::Data<AppState>,
        form: web::Query<FormItem>,
    ) -> impl Responder {
        // let payload: Value = serde_json::from_str(body.as_str()).unwrap();
        // info!("{x}",x=payload.clone());
        // let current = payload["selected"].as_str().unwrap();
        // let value = payload[current].as_f64().unwrap() as f32;
        let pool = &data.database.conn;
        // let sessiondata = SessionParser::from(session.clone());
        // info!(
        //     "usuario {id_usuario}",
        //     id_usuario = &sessiondata.id_usuario.clone()
        // );

        // let _usuario = get_user(pool, &session).await.unwrap();
        // let id_usuario = usuario.id;
        // let id_empresa = usuario.id_empresa;
        let (id_pedido, id_produto, id_item) = path.into_inner();

        let item = crate::pedido::repo::get_item_pedido(
            pool,
            id_pedido.clone(),
            id_produto.clone(),
            id_item,
        )
        .await;

        let web::Query(form) = form;
        // let form = FormItem {
        //     quantidade: value,
        // };

        async fn update_item_pedido(
            pool: &sqlx::Pool<Postgres>,
            id_pedido: String,
            id_produto: String,
            id_item: i32,
            form: FormItem,
        ) -> EntidadeItemPedido {
            crate::pedido::repo::update_item_pedido(pool, id_pedido, id_produto, id_item, form)
                .await
        }

        async fn create_item_pedido(
            pool: &sqlx::Pool<Postgres>,
            id_pedido: String,
            id_produto: String,
            form: FormItem,
        ) -> EntidadeItemPedido {
            crate::pedido::repo::create_item_pedido(pool, id_pedido, id_produto, form).await
        }

        let item = match item {
            Some(value) => {
                update_item_pedido(pool, value.id_pedido, value.id_produto, value.id_item, form)
                    .await
            }
            None => create_item_pedido(pool, id_pedido, id_produto, form).await,
        };

        // response
        let item = abrir_item_pedido_one(pool, item.id_pedido, item.id_produto, item.id_item).await;
        crate::infra::render::render_minijinja(
            "comp/pedido/view_total_pedido_item.html",
            context!(item),
        )

        // crate::infra::render::render_minijinja("comp/pedido/pedido_item.html", context!(item))
    }

    #[put("up/{id_pedido}/{id_produto}/{id_item}")]
    pub async fn put_update_item(
        // body: String,
        session: Session,
        path: web::Path<(String, String, i32)>,
        data: web::Data<AppState>,
        form: web::Query<FormItem>,
    ) -> impl Responder {
        // let payload: Value = serde_json::from_str(body.as_str()).unwrap();
        // info!("{x}",x=payload.clone());
        // let current = payload["selected"].as_str().unwrap();
        // let value = payload[current].as_f64().unwrap() as f32;
        let pool = &data.database.conn;
        let sessiondata = SessionParser::from(session.clone());
        info!(
            "usuario {id_usuario}",
            id_usuario = &sessiondata.id_usuario.clone()
        );

        let _usuario = get_user(pool, &session).await.unwrap();
        // let id_usuario = usuario.id;
        // let id_empresa = usuario.id_empresa;
        let (id_pedido, id_produto, id_item) = path.into_inner();

        let item = crate::pedido::repo::get_item_pedido(
            pool,
            id_pedido.clone(),
            id_produto.clone(),
            id_item,
        )
        .await;

        let web::Query(form) = form;
        // let form = FormItem {
        //     quantidade: value,
        // };

        async fn update_item_pedido(
            pool: &sqlx::Pool<Postgres>,
            id_pedido: String,
            id_produto: String,
            id_item: i32,
            form: FormItem,
        ) -> EntidadeItemPedido {
            crate::pedido::repo::update_item_pedido(pool, id_pedido, id_produto, id_item, form)
                .await
        }

        async fn create_item_pedido(
            pool: &sqlx::Pool<Postgres>,
            id_pedido: String,
            id_produto: String,
            form: FormItem,
        ) -> EntidadeItemPedido {
            crate::pedido::repo::create_item_pedido(pool, id_pedido, id_produto, form).await
        }

        let item = match item {
            Some(value) => {
                update_item_pedido(pool, value.id_pedido, value.id_produto, value.id_item, form)
                    .await
            }
            None => create_item_pedido(pool, id_pedido, id_produto, form).await,
        };

        // response
        let item = abrir_item_pedido_one(pool, item.id_pedido, item.id_produto, item.id_item).await;
        crate::infra::render::render_minijinja(
            "comp/pedido/view_total_pedido_item.html",
            context!(item),
        )

        // crate::infra::render::render_minijinja("comp/pedido/pedido_item.html", context!(item))
    }
    #[patch("up/{id_pedido}/{id_produto}/{id_item}")]
    pub async fn patch_update_item(
        // body: String,
        session: Session,
        path: web::Path<(String, String, i32)>,
        data: web::Data<AppState>,
        form: web::Query<FormItem>,
    ) -> impl Responder {
        // let payload: Value = serde_json::from_str(body.as_str()).unwrap();
        // info!("{x}",x=payload.clone());
        // let current = payload["selected"].as_str().unwrap();
        // let value = payload[current].as_f64().unwrap() as f32;
        let pool = &data.database.conn;
        let sessiondata = SessionParser::from(session.clone());
        info!(
            "usuario {id_usuario}",
            id_usuario = &sessiondata.id_usuario.clone()
        );

        let _usuario = get_user(pool, &session).await.unwrap();
        // let id_usuario = usuario.id;
        // let id_empresa = usuario.id_empresa;
        let (id_pedido, id_produto, id_item) = path.into_inner();

        let item = crate::pedido::repo::get_item_pedido(
            pool,
            id_pedido.clone(),
            id_produto.clone(),
            id_item,
        )
        .await;

        let web::Query(form) = form;
        // let form = FormItem {
        //     quantidade: value,
        // };

        async fn update_item_pedido(
            pool: &sqlx::Pool<Postgres>,
            id_pedido: String,
            id_produto: String,
            id_item: i32,
            form: FormItem,
        ) -> EntidadeItemPedido {
            crate::pedido::repo::update_item_pedido(pool, id_pedido, id_produto, id_item, form)
                .await
        }

        async fn create_item_pedido(
            pool: &sqlx::Pool<Postgres>,
            id_pedido: String,
            id_produto: String,
            form: FormItem,
        ) -> EntidadeItemPedido {
            crate::pedido::repo::create_item_pedido(pool, id_pedido, id_produto, form).await
        }

        let item = match item {
            Some(value) => {
                update_item_pedido(pool, value.id_pedido, value.id_produto, value.id_item, form)
                    .await
            }
            None => create_item_pedido(pool, id_pedido, id_produto, form).await,
        };

        // response
        let item = abrir_item_pedido_one(pool, item.id_pedido, item.id_produto, item.id_item).await;
        crate::infra::render::render_minijinja(
            "comp/pedido/view_total_pedido_item.html",
            context!(item),
        )

        // crate::infra::render::render_minijinja("comp/pedido/pedido_item.html", context!(item))
    }
}

use controller::*;

// Define as rotas para o controlador de autenticação
pub fn routes(cfg: &mut crate::web::ServiceConfig) {
    cfg.service(
        crate::web::scope("/pedido")
            .service(pedido_add)
            .service(pedido_form)
            .service(update_item)
            .service(get_update_item)
            .service(put_update_item)
            .service(patch_update_item),
    );
}
