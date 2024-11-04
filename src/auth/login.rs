use std::time::Duration;

use crate::infra::error::Error;
use crate::services::login::token;
use crate::services::usuario as service;
use crate::{app::AppState, auth::model::LoginForm, infra::strings::anonimizar};
use actix_session::Session;
use actix_web::{get, http::StatusCode, post, web, HttpResponse, Responder};
use log::info;
use minijinja::context;
use serde_json::json;

#[get("/entrar")]
async fn login(data: web::Data<AppState>) -> impl Responder {
    let tmpl = data.render.get_template("auth/login.html").unwrap();
    let rendered = tmpl.render(context! {title => "Login"}).unwrap();

    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[post("/entrar")]
async fn login_submit(
    session: Session,
    form: web::Form<LoginForm>,
    data: web::Data<AppState>,
) -> impl Responder {
    const HORAS_DIA: u64 = 24;
    const MINUTOS_HORA: u64 = 60;
    const SEGUNDOS_MINUTO: u64 = 60;

    info!("Tentativa de LOGIN: {:?}", anonimizar(form.email.as_ref()));
    let web::Form(form) = form;
    let pool = &data.database;
    let login_inspect = service::login(&pool, &form.email, &form.senha).await;

    if let Some(valid_login) = login_inspect {
        if valid_login {
            info!("🙎‍♂️ Acesso concedido ✔ ");
            let token = token(&form.email, Duration::from_secs(15 * HORAS_DIA * MINUTOS_HORA * SEGUNDOS_MINUTO))
                .expect("Erro ao desempactoar o token");

            session.insert("token", token.clone()).unwrap();
            session.insert("user_id", form.email.clone()).unwrap();

            //Vamos tentar pegar outros dados do usuario para incorporar na sessao
            let abrir_usuario = service::abrir_usuario(pool, form.email).await;
            if let Some(usuario) = abrir_usuario {
                session.insert("user_name", usuario.nome).unwrap();
                session.insert("user_level", usuario.nivel.clone()).unwrap();
                let is_admin = if usuario.nivel == "ADMIN".to_owned() { &"true" } else { &"false" };
                session.insert("is_admin", is_admin).unwrap();
            }

            HttpResponse::Ok()
                .content_type("application/json")
                .json(json!({
                "redirect": "/admin/painel"
                }))
        } else {
            info!("🙇‍♂️ Acesso negado ❌ ");

            let tmpl = data.render.get_template("components/ajaxToast.html").unwrap();
            let rendered = tmpl.render(context! {
                toast_type => "toast-error",
                toast_icon => "bi-exclamation-circle-fill",
                toast_text =>"Usuário ou senha inválidos"
            }).unwrap();

            HttpResponse::Unauthorized()
                .content_type("application/json")
                .json(json!({
                    "form":{
                        "email":"",
                        "password":""
                    },
                    "toast":rendered
                }))
            /*
            Error::Detailed {
                code: StatusCode::UNAUTHORIZED,
                msg: "Usuário ou senha inválidos".to_owned(),
                description: "".to_owned(),
                how_to_solve: "Confira o usuário e a senha".into(),
            }
            .into()
             */
        }
    } else {
        info!("🙇‍♂️ Acesso negado ❌ ");
        Error::Detailed {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            msg: "Erro interno do servidor".to_owned(),
            description: "Houve uma falha ao tentar fazer login".to_owned(),
            how_to_solve: format!(
                r#"Envie este relatório para o suporte ou tente novamente mais tarde \n {:?}"#,
                form.email
            ),
        }
            .into()
    }
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login).service(login_submit);
}
