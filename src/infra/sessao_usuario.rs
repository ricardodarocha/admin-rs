// session.rs

use serde::{Deserialize, Serialize};
use actix_web::{Error, HttpResponse, http::header};
use actix_session::{Session, SessionExt};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
use actix_web::body::BoxBody;
use crate::infra::jwt::validate_jwt;

use super::jwt::jwt_secret;


/// Estrutura para armazenar dados da sessão do usuário
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sessao {
    pub user_id: String,
    pub jwt_token: String,
    pub user_name: Option<String>,
    pub user_level: Option<String>,
    pub is_admin: bool,
}

impl Sessao {
    /// Carrega a `Sessao` a partir da sessão do Actix
    /// Verifica se `user_id` e `jwt_token` existem e são válidos
    pub fn from_session(session: &Session, secret: &[u8]) -> Result<Option<Self>, Error> {
        // Verifica a presença de `user_id` e `jwt_token`
        let user_id = match session.get::<String>("user_id")? {
            Some(id) => id,
            None => return Ok(None), // Retorna None se `user_id` não estiver na sessão
        };

        let jwt_token = match session.get::<String>("token")? {
            Some(token) => token,
            None => return Ok(None), // Retorna None se `jwt_token` não estiver na sessão
        };

        // Verifica se o token é válido para o `user_id`
        if !crate::infra::jwt::validate_jwt(&jwt_token, secret, &user_id)? {
            return Ok(None); // Retorna None se o token não for válido
        }

        // Carrega outros dados opcionais da sessão
        let user_name = session.get::<String>("user_name")?;
        let user_level = session.get::<String>("user_level")?;
        let is_admin = session.get::<String>("is_admin")?.map(|val| val == "true");
        
        let is_admin = if let Some(is_admin) = is_admin {
            is_admin
        } else {false};

        Ok(Some(Sessao {
            user_id,
            jwt_token,
            user_name,
            user_level,
            is_admin,
        }))
    }
}

// .wrap_fn(auth_middleware) // Middleware de autenticação com JWT
        
pub async fn auth_middleware(
    req: ServiceRequest,
    next: impl Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    let session = req.get_session();
    let token: Option<String> = session.get("token").unwrap_or(None);

    if let Some(jwt_token) = token {
        let secret = jwt_secret();
        if let Some(user_id) = session.get::<String>("user_id").unwrap_or(None) {
            match validate_jwt(&jwt_token, secret.as_slice(), &user_id) {
                Ok(true) => next.call(req).await, // Token válido, prossegue
                _ => Ok(req.into_response(
                    HttpResponse::Found()
                        .append_header((header::LOCATION, "/entrar"))
                        .finish()
                        .map_into_boxed_body(),
                )),
            }
        } else {
            // Usuário não autenticado
            Ok(req.into_response(
                HttpResponse::Found()
                    .append_header((header::LOCATION, "/entrar"))
                    .finish()
                    .map_into_boxed_body(),
            ))
        }
    } else {
        // Sem token JWT
        Ok(req.into_response(
            HttpResponse::Found()
                .append_header((header::LOCATION, "/entrar"))
                .finish()
                .map_into_boxed_body(),
        ))
    }
}

// use actix_web::{dev, web, Appstate, HttpResponse};
// use actix_web::web::Data;
// use futures::future::{ok, Ready};

// async fn auth_middleware(req: dev::ServiceRequest, srv: web::Data<actix_web::AppState>) -> Result<dev::ServiceResponse, actix_web::Error> {
//     let cookie_value = req
//         .cookie("name-of-cookie")
//         .map(|cookie| cookie.value().to_owned())
//         .ok_or_else(|| HttpResponse::Unauthorized().finish())?;

//     if !is_authorized(&cookie_value) {
//         return Err(HttpResponse::Unauthorized().finish().into());
//     }

//     // authorized ? continue to the next middleware/handler
//     let fut = srv
//         .get_ref()
//         .clone()
//         .call(req)
//         .await
//         .map_err(|e| HttpResponse::InternalServerError().body(e.to_string()))?;
//     Ok(fut)
// }