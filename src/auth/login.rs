use crate::infra::error::Error;
use crate::services as service;
use crate::{app::AppState, auth::model::LoginForm, infra::strings::anonimizar};
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
async fn login_submit(form: web::Form<LoginForm>, data: web::Data<AppState>) -> impl Responder {
    info!("Tentativa de LOGIN: {:?}", anonimizar(form.email.as_ref()));
    let web::Form(form) = form;
    let pool = &data.database;
    let login_inspect = service::login(&pool, &form.email, &form.senha).await;

    if let Some(valid_login) = login_inspect {
        if valid_login {
            info!("🙎‍♂️ Acesso concedido ✔ ");

            HttpResponse::Ok()
                .content_type("application/json")
                .json(json!({
                "redirect": "/admin/painel"
                }))
        } else {
            info!("🙇‍♂️ Acesso negado ❌ ");
            Error::Detailed {
                code: StatusCode::UNAUTHORIZED,
                msg: "Usuário ou senha inválidos".to_owned(),
                description: "".to_owned(),
                how_to_solve: "Confira o usuário e a senha".into(),
            }
            .into()
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
