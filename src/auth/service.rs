use std::net::IpAddr;

use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde_json::json;
use actix_web::http::header::LOCATION;

// use crate::admin::model::Empresa;
// use crate::admin::repo::atualizar_empresa;
use crate::infra::psw::genpassword;
use crate::infra::render::reject;
use crate::{app::AppState, infra::result::Result};
use crate::auth::model::*;

use crate::auth::repo::{self as repo, abrir_usuario, incluir_enviar_email_primeiro_acesso};

use super::repo::inserir_empresa_primeiro_acesso;

pub async fn registrar_tentativa_de_acesso(
    login_form: &LoginForm, 
    status: &String,
    origem: &String,
    id_usuario: &String,

) {
   repo::inserir_log_acesso(&login_form.username, status, origem, id_usuario).await;  
}
pub async fn registrar_consumo_rota(
    rota: &String, 
    status: &String,
    origem: &String, //ip de origem
    id_usuario: &String,

) {
   repo::inserir_consumo_rota(rota, status, origem, id_usuario).await;  
}

pub async fn login_user(

    data: web::Data<AppState>, 
    login_form: &LoginForm,
    session: Session,
    ip: Option<IpAddr>,

) -> Result<HttpResponse> {
    

    let password_hash = format!("{:x}", md5::compute(&login_form.password));
    let user = abrir_usuario(&data.database.conn, &login_form.username, &password_hash).await;
    dbg!(&password_hash);
    dbg!(login_form);

    let redirect = |id_usuario, id_empresa| -> HttpResponse {
        let _ = session.insert("user_id", id_usuario);
        let _ = session.insert("empresa_id", id_empresa);
        
        // let url_for = format!("{}/", std::env::var("SITE").unwrap());
        HttpResponse::SeeOther()
            .insert_header((LOCATION, "/"))
            .finish()
    };

    let error = || -> HttpResponse {let error_response = json!({
                "code": 403,
                "error": "Forbidden",
                "message": "O usuário não possui permissão", 
                "required": "e-mail, senha" 
            });
            let status_code = actix_web::http::StatusCode::FORBIDDEN;
            HttpResponse::build(status_code).json(error_response)};

    

    if let Some(user) = user {
        dbg!(user.clone());
        registrar_tentativa_de_acesso(
            login_form, 
            &"SUCESSO".to_owned(), 
            &format!("{:?}", ip), 
            &user.id.clone()).await;
            let id_empresa = match user.id_empresa {
                Some(id) => id,
                None => "0".to_owned(),
            };
        Ok(redirect(user.id, id_empresa))
    } else
    {
        registrar_tentativa_de_acesso(
            login_form, 
            &"FAILED".to_owned(), 
            &format!("{:?}", ip), 
            &"INDEFINIDO".to_owned()).await;
        Ok(error())
    }
    
}

pub async fn logout_user(
    _data: &web::Data<AppState>, 
    session: &Session,
    ) -> Result<HttpResponse> {
        session.remove("user_id");
        
        // let url_for = format!("{}/login", std::env::var("SITE").unwrap());

        let redirect = || -> HttpResponse {
            session.clear();
            HttpResponse::SeeOther()
                .insert_header((LOCATION, "/login"))
                .finish()
    };    

    Ok(redirect())

    }

pub async fn subscribe(
    data: web::Data<AppState>, 
    form: &SubscribeForm, 

) -> Result<HttpResponse> {
        _ = repo::inserir_subscriber(&data.database.conn, form.clone()).await;
        
        // let url_for = format!("{}/", std::env::var("SITE").unwrap());
        Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, "/"))
                .finish())


}

pub async fn enviar_email(_nome: String, _email: String, _senhaprovisoria: String) {
    //do nothing
  
}

/// {
///    nome,
///    instituicao
///    e-mail
///    telefone
/// }
/// # Procedimento
/// Um usuário se inscreve, 
/// Então será mostrado no painel de supervisores para liberar o acesso
/// De acordo com o plano escolhido, o supervisor irá configurar quais permissões serão liberadas
/// Isso irá criar os menus que o usuário tem direito

pub async fn register(
    data: web::Data<AppState>, 
    form: &RegisterUser, 

) -> Result<HttpResponse> {
        let pool = &data.database.conn;
        let encontrou = repo::abrir_usuario_from_email(pool, &form.email).await;
        if let Some(_usuario) = encontrou {
            return reject("Este e-mail já foi cadastrado. Solicite uma nova senha")};

        let password_hash = if let Some(password) = &form.password {
            println!("Senha informada {}", password.clone());
            format!("{:x}", md5::compute(password))
        } 
        else
        {
            // gera uma senha provisória para o usuário
            let senha_provisoria = genpassword(6);
            println!("Senha gerada {}", senha_provisoria.clone());
            incluir_enviar_email_primeiro_acesso(pool, form.email.clone(), senha_provisoria.clone()).await;
            format!("{:x}", md5::compute(senha_provisoria))
        
        };
        let id_admin = &"d47e184c-8118-554e-a11c-97c308ad7669"; //Ricardo
        let novo_usuario = repo::incluir_user(pool, &form, password_hash).await.unwrap();
        let _perfil = repo::atribuir_perfil_usuario(pool, &novo_usuario.clone(), &"USER", id_admin).await;
        let instituicao = repo::incluir_instituicao(pool, novo_usuario.id.clone(), &form.instituicao).await.unwrap();
        let _vinculo = repo::vincular_empresa_usuario(pool, &novo_usuario, &instituicao).await;
        //permissoes basicas INCLUIR PEDIDO, READ DASHBOARD
        let _permissoes = repo::incluir_permissoes(pool, &novo_usuario).await;


        // let url_for = format!("{}/login", std::env::var("SITE").unwrap());
        Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, "/login"))
                .finish())

}
pub async fn primeiro_acesso(
    data: web::Data<AppState>, 
    form: &PrimeiroAcesso, 

) -> Result<HttpResponse> {
        let pool = &data.database.conn;
        let encontrou = repo::abrir_usuario_from_email(pool, &form.email).await;
        let novo_usuario = 
        if let Some(usuario) = encontrou {
            usuario
            }
        else {
            let _ = register(data.clone(), &RegisterUser::from(form.clone())).await;
            repo::abrir_usuario_from_email(pool, &form.email).await.unwrap()
        };

        
        let _id_admin = &"d47e184c-8118-554e-a11c-97c308ad7669"; //Ricardo
        inserir_empresa_primeiro_acesso(pool, novo_usuario.id, form.clone()).await;
        
        // let url_for = format!("{}/login", std::env::var("SITE").unwrap());

        Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, "/login"))
                .finish())

}

// Adiciona uma permissao a um determinado usuario
// pub async fn allow_user(req: &RegisterRequest) -> Result<HttpResponse> {
    // Pseudocódigo para registro de usuário
    // 1. Valide as credenciais do admin
    // 2. Valide as credenciais do usuario, se esta ativo
    // 3. Adicione a permissao. Coloque a assinatura do admin
    // revoke_user();
    // insert into user_permissions set permission = $1 AND operation = $2
    //         WHERE user_id = $3 
// }

// Remove uma permissao de um determinado usuario
// pub async fn revoke_user(req: &RegisterRequest) -> Result<HttpResponse> {
    // Pseudocódigo para remover a permissao
    // 1. Valide os dados do admin
    // 2. Remova as credenciais especificadas
    // delete FROM user_permissions 
    //         WHERE user_id = $1 AND permission = $2 AND operation = $3
// }

