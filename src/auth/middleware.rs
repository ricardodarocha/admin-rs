use actix_web::{
    dev::{ServiceRequest, ServiceResponse, Service, Transform},
    http::header,
    web::{self, Data},
    App, Error, HttpResponse, HttpServer,
};
use actix_web::body::MessageBody;
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use futures::future::{ok, Either, Ready};
use futures::FutureExt;

async fn user_has_access(user_id: i32, path: &str) -> bool {
    // Implemente a lógica para verificar se o usuário tem acesso a `path`
    // Por exemplo, consulte uma base de dados ou verifique uma lista de permissões
    true // Apenas um placeholder; substitua pela lógica real
}

// Primeiro middleware: verifica se o usuário está logado
async fn auth_middleware(
    req: ServiceRequest,
    next: actix_web::dev::ServiceCall,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let session = req.get_session();
    let is_logged_in = session.get::<bool>("logged_in").unwrap_or(None).unwrap_or(false);

    if is_logged_in {
        // Se o usuário está logado, passa a requisição para o próximo middleware ou rota
        next.call(req).await
    } else {
        // Se o usuário não está logado, redireciona para a página de login
        Ok(req.into_response(
            HttpResponse::Found()
                .append_header((header::LOCATION, "/login"))
                .finish()
                .into_body(),
        ))
    }
}

// Segundo middleware: verifica o nível de acesso do usuário
async fn access_middleware(
    req: ServiceRequest,
    next: actix_web::dev::ServiceCall,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let session = req.get_session();
    if let Some(user_id) = session.get::<i32>("user_id").unwrap_or(None) {
        let path = req.path();

        if user_has_access(user_id, path).await {
            // Se o usuário tem permissão para acessar, passa a requisição adiante
            next.call(req).await
        } else {
            // Caso contrário, responde com um erro de acesso negado
            Ok(req.into_response(
                HttpResponse::Forbidden()
                    .body("Acesso negado")
                    .into_body(),
            ))
        }
    } else {
        // Se não há um `user_id` na sessão, redireciona para a página de login
        Ok(req.into_response(
            HttpResponse::Found()
                .append_header((header::LOCATION, "/login"))
                .finish()
                .into_body(),
        ))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                actix_web::cookie::Key::generate(),
            ))
            .wrap_fn(auth_middleware)
            .wrap_fn(access_middleware)
            .route("/", web::get().to(|| async { "Página inicial" }))
            .route("/login", web::get().to(|| async { "Página de login" }))
            .route("/restricted", web::get().to(|| async { "Página restrita" }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
