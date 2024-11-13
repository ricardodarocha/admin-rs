use actix_web::middleware::Next;
use actix_web::HttpMessage;
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

// user_id
// token
// user_name
// user_level
// is_admin


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



// Contribuicao do Alexandre 09/11/24

// pub async fn check_auth_web_middleware(
//     req: ServiceRequest,
//     next: Next<BoxBody>,
// ) -> Result<ServiceResponse<BoxBody>, Error> {
//     // Recupera o token do cabeçalho Authorization
//     let auth_token = req
//         .headers()
//         .get(AUTHORIZATION)
//         .and_then(|header| header.to_str().ok());

//     // Tenta recuperar o token do cookie caso o cabeçalho Authorization não esteja presente
//     let cookie_token = req.cookie("token").map(|cookie| cookie.value().to_string());

//     // Prioriza o token do cabeçalho Authorization, mas se não houver, usa o do cookie
//     let token = auth_token
//         .map(|header| header.replace("Bearer ", ""))
//         .or(cookie_token);

//     if let Some(token) = token {
//         // Decodifica o token JWT
//         match utils::decode_jwt(&token) {
//             Ok(claim) => {
//                 let api_state = req.app_data::<web::Data<ApiState>>().unwrap();
//                 let mut valid_token = false;

//                 {
//                     let cache = api_state.cache.lock().unwrap();

//                     // Imprimindo o conteúdo do cache para depuração
//                     // println!("Cache atual:");
//                     // for (user, cached_token) in cache.iter() {
//                     //     println!("Usuário: {}, Token: {}", user, cached_token);
//                     // }

//                     // Verifica se o token armazenado no cache corresponde ao token no cabeçalho ou cookie
//                     if let Some(cached_token) = cache.get(&claim.usuario) {
//                         if cached_token == &token {
//                             valid_token = true;
//                         }
//                     }
//                 }

//                 if valid_token {
//                     // Insere o claim nos dados da requisição para ser utilizado posteriormente
//                     req.extensions_mut().insert(claim);
//                     next.call(req).await
//                 } else {
//                     let api_response = ApiResponse::new(
//                         401,
//                         json!({"status": "error", "message": "Token inválido ou não encontrado no cache. Volte e efetue o login."}),
//                     );
//                     let mut response = api_response.error_response();
//                     response
//                         .headers_mut()
//                         .insert(LOCATION, "/login".parse().unwrap());
//                     Ok(req.into_response(response.map_into_boxed_body()))
//                 }
//             }
//             Err(_) => {
//                 let api_response = ApiResponse::new(
//                     401,
//                     json!({"status": "error", "message": "Token expirado ou inválido. Volte e efetue o login."}),
//                 );

//                 let mut response = api_response.error_response();
//                 response
//                     .headers_mut()
//                     .insert(LOCATION, "/login".parse().unwrap());

//                 Ok(req.into_response(response.map_into_boxed_body()))
//             }
//         }
//     } else {
//         // Caso nenhum token seja encontrado no cabeçalho ou cookie
//         let api_response =
//             ApiResponse::new(401, json!({"status": "fail", "message": "Não autorizado"}));
//         Err(api_response.into())
//     }
// }

pub async fn check_admin_auth(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {

    let session = req.extensions().get::<Session>().cloned().expect("Session não encontrada. Acesso restrito para administradores ");
    let user_session = Sessao::from_session(&session, &jwt_secret())?;
    let user_is_admin = if let Some(user_session) = user_session {
        user_session.is_admin  
    } else 
    {
        return Err(actix_web::error::ErrorForbidden("Acesso restrito para administradores"));
    };

    if user_is_admin {
        let res = next.call(req).await?;
        Ok(res)
    } else {
        Err(actix_web::error::ErrorForbidden("Acesso restrito para administradores"))
    }
}

pub async fn check_api_auth(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {

    let session = req.extensions().get::<Session>().cloned().expect("Session não encontrada. Obtenha um CLIENT-ID para acessar a API ");
    let user_session = Sessao::from_session(&session, &jwt_secret())?;
    let user_is_admin = if let Some(user_session) = user_session {
        user_session.is_admin  
    } else 
    {
        return Err(actix_web::error::ErrorForbidden("Obtenha um CLIENT-ID para acessar a API"));
    };

    if user_is_admin {
        let res = next.call(req).await?;
        Ok(res)
    } else {
        Err(actix_web::error::ErrorForbidden("Obtenha um CLIENT-ID para acessar a API"))
    }
}

// pub async fn check_user_auth(
//     req: ServiceRequest,
//     next: Next<BoxBody>,
// ) -> Result<ServiceResponse<BoxBody>, Error> {
    
//     let req2 = &req;
//     let req3 = &req;
//     let req4 = &req;
//     let state = req2.app_data::<web::Data<AppState>>()
//         .ok_or_else(|| actix_web::error::ErrorInternalServerError("Conexão não configurada"))?;
    
//     let session = req3.extensions().get::<Session>().cloned().expect("Session não encontrada. Requer autenticação ");
//     let user_session = Sessao::from_session(&session, &jwt_secret())?;
//     let user_level = if let Some(user_session) = user_session {
//         user_session.user_level  
//     } else 
//     {
//         return Err(actix_web::error::ErrorForbidden("Requer autenticação"));
//     };

//     if user_level == Some("USER".to_string()) {
//         // Se o usuário estiver autenticado, prossegue com a requisição
//         let res = next.call(req).await?;
//         Ok(res)
//     } else {
//         // Se não estiver autenticado, retorna um erro 401
//         Err(actix_web::error::ErrorUnauthorized("Usuário não autenticado"))
//     };

//     let auth_token = req3.headers()
//         .get("Authorization")
//         .and_then(|header| header.to_str().ok())
//         .map(|header| header.replace("Bearer ", ""));

//     let user_id = req.cookie("user_id")
//         .map(|cookie| cookie.value().to_string());

//     // Verificando se temos um usuário autenticado
//     let user_authenticated = auth_token.is_some() || user_id.is_some();

//     if user_authenticated {
//         // Se o usuário estiver autenticado, prossegue com a requisição
//         let res = next.call(req).await?;
//         Ok(res)
//     } else {
//         // Se não estiver autenticado, retorna um erro 401
//         Err(actix_web::error::ErrorUnauthorized("Usuário não autenticado"))
//     }
// }