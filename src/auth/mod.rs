pub mod session;
pub mod service;
pub mod repo;
pub mod model;
pub mod view;

pub mod controller {
    use std::net::IpAddr;

    use actix_session::Session;
    // use actix_web::http::header::ContentType;
    use actix_web::{get, post, Responder};  
    use actix_web::{web, HttpRequest, HttpResponse}; //, http::StatusCode
    use minijinja::context;
    use crate::app::AppState;
    use crate::auth::model::*;
    use crate::auth::service as service;
    use actix_web::http::header::LOCATION;

    use super::session::has_logged;

    /// Login Form
    #[utoipa::path(context_path = "/login",
        responses(
            (status = 200, description = "Login")
    ))]
    #[get("")]
    pub async fn login_form(
        _req: HttpRequest,
        session: Session,
        // path: web::Path<(String, String)>,
        data: web::Data<AppState>,
        // filtro_data: web::Query<FiltroData>,

        ) -> impl Responder {

        dbg!("GET /login -> ");
        // let url_for = format!("{}/", std::env::var("SITE").unwrap());

        if has_logged(&data.database.conn, &session).await {
             return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/"))
            .finish())
        };

        let form = LoginForm::default();

        let flash = session.remove("flash").unwrap_or("".to_string()); 
        let msg_error = format!("{}", session.remove("msg_error").unwrap_or("".to_string()));   


        crate::infra::render::render_minijinja("login/login.html", context!(form, flash, msg_error)) 
}

    /// Subscribe Form
    #[utoipa::path(context_path = "/login",
        responses(
            (status = 200, description = "Just subscribe your e-mail")
    ))]
    #[get("/subscribe")]
    pub async fn subscribe_form(
        _req: HttpRequest,
        session: Session,
        // path: web::Path<(String, String)>,
        _data: web::Data<AppState>,
        // filtro_data: web::Query<FiltroData>,
        ) -> impl Responder {

        dbg!("GET /subscribe");
        let form = SubscribeForm::default();

        let flash = session.remove("flash").unwrap_or("".to_string()); 
        let msg_error = format!("{}", session.remove("msg_error").unwrap_or("".to_string()));   

        crate::infra::render::render_minijinja("login/subscribe.html", context!(form, flash, msg_error) )
}

    /// Login Form
    #[utoipa::path(context_path = "/login",
        responses(
            (status = 200, description = "Login")
    ))]
    #[post("")]
    pub async fn login(
        req: HttpRequest,
        login_body: web::Form<LoginForm>,
        // path: web::Path<(String, String)>,
        data: web::Data<AppState>,
        session: Session,
        ) -> impl Responder {

        let ip: Option<IpAddr> = if let Some(val) = req.peer_addr() {
            Some(val.ip())
        } else 
        {
            None
        };
        
        service::login_user(data, &login_body, session, ip).await.unwrap() 
         
    }

    /// Subscribe
    #[utoipa::path(context_path = "/login",
        responses(
            (status = 200, description = "Subscribe")
    ))] 
    #[post("/subscribe")]
    pub async fn subscribe(
        _req: HttpRequest,
        subscriber: web::Form<SubscribeForm>,
        // path: web::Path<(String, String)>,
        data: web::Data<AppState>,
        // _session: Session, //does not require login to subscribe
        ) -> HttpResponse {
        
        service::subscribe(data, &subscriber).await.unwrap()          
    }


    /// Logout
    #[utoipa::path(context_path = "login",
        responses(
            (status = 200, description = "Logout")
    ))]
    // Função para logout
    #[get("/logout")]
    pub async fn logout(        
        data: web::Data<AppState>,
        session: Session,
    ) -> impl Responder {
        match service::logout_user(&data, &session).await {
            Ok(response) => response,
            Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
        }
    }

    /// Register Form
    #[utoipa::path(context_path = "/login",
        responses(
            (status = 200, description = "Register")
    ))]
    #[get("/register")]
    pub async fn register_form(
        _req: HttpRequest,
        session: Session,
        // path: web::Path<(String, String)>,
        // data: web::Data<AppState>,
        // filtro_data: web::Query<FiltroData>,
        ) -> impl Responder {

        dbg!("GET /login/register ");
        let form = RegisterData::default();

        let flash = session.remove("flash").unwrap_or("".to_string()); 
        let msg_error = format!("{}", session.remove("msg_error").unwrap_or("".to_string()));   

        crate::infra::render::render_minijinja("login/register.html", context!(form, flash, msg_error) )
}

    /// Register
    #[utoipa::path(
        responses(
            (status = 200, description = "Register")
    ))]
    #[post("/register")]
    pub async fn register(
        data: web::Data<AppState>,
        form: web::Form<RegisterUser>
        
    ) -> impl Responder {

        match service::register(data, &form).await {
            Ok(response) => response,
            Err(e) => HttpResponse::BadRequest().json(e.to_string()),
        }
    }

    /// Primeiro Acesso Form
    #[utoipa::path(
        context_path = "/login",
        responses(
            (status = 200, description = "Formulário para cadastrar o primeiro acesso")
    ))]
    #[post("/primeiroacesso/form")]
    pub async fn primeiroacessoform(
        _req: HttpRequest,
        session: Session,
        // path: web::Path<(String, String)>,
        // data: web::Data<AppState>,
        form: web::Form<RegisterUser>
        // filtro_data: web::Query<FiltroData>,
        ) -> impl Responder {

        dbg!("POST /login/primeiroacesso/form ");

        let flash = session.remove("flash").unwrap_or("".to_string()); 
        let msg_error = format!("{}", session.remove("msg_error").unwrap_or("".to_string()));   

        crate::infra::render::render_minijinja("login/primeiro_acesso_form.html", context!(form, flash, msg_error) )
}

    /// Cadastro do primeiro acesso
    #[utoipa::path(
        context_path = "/login",
        responses(
            (status = 200, description = "Primeiro Acesso")
    ))]
    #[post("/primeiroacesso")]
    pub async fn primeiroacesso(
        data: web::Data<AppState>,
        form: web::Form<PrimeiroAcesso>
        
    ) -> impl Responder {

        match service::primeiro_acesso(data, &form).await {
            Ok(response) => response,
            Err(e) => HttpResponse::BadRequest().json(e.to_string()),
        }
    }
}
use controller::*;

// Define as rotas para o controlador de autenticação
pub fn routes(cfg: &mut crate::web::ServiceConfig)  {
    cfg.service(
        crate::web::scope("/login")
            .service(login_form)
            .service(login)
            .service(register_form)
            .service(register)
            .service(primeiroacessoform)
            .service(primeiroacesso)
    )
    
            .service(logout)
            ;
}