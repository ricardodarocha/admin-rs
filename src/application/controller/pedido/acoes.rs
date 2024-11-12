use actix_web::{post, web, HttpResponse, Responder};
use actix_session::Session;
use log::{error, info};
use serde_json::json;
use crate::app::AppState;
use crate::core::tratados::Repository;
use crate::infra::error::Error;
use crate::infra::toast::{ApiResponse, Toast};
use crate::infra::validation::export_validations;
use crate::models::pedido::{EntidadePedido, NovoPedido, PayloadPedido};
use validator::Validate;

    #[post("/pedido")]
    async fn inserir_pedido_from_form(
        
        pool: web::Data<AppState>, 
        payload: web::Form<NovoPedido>, 
        _session: Session) -> impl Responder {
        
        // 1. Verifica a validação do formulário ver crates.io/crates/validator
        info!("⏳ Valiando dados do formulário de pedidos");
        if let Err(errors) = payload.validate() {
            let toast = export_validations(&errors, "Preencha os campos corretamente");
            let erro = Error::Form(toast);
            error!("{:?}", erro);
            return erro.into();
        }; 

        // 3. Usando ORM ou Repository
        let result = EntidadePedido::create(&pool.database, &payload.into_inner()).await; 
        match result{  
            Ok(pedido) => ApiResponse::new().with_data(json!(pedido)).send(),
            Err(err) => HttpResponse::InternalServerError().content_type("application/json")
                        .json(Toast::from(err))
        }

        // Response nativo
        // match result{  
        //     Ok(pedido) => HttpResponse::Created().json(pedido),
        //     Err(err) => HttpResponse::InternalServerError().body(err),
        // }
    }

    #[post("/pedido")]
    async fn inserir_pedido_from_json(
        pool: web::Data<AppState>, 
        payload: web::Json<NovoPedido>,
        _session: Session,
    
    ) -> impl Responder {
        let payload = payload.into_inner();
        
        // 1. Verifica a validação do formulário ver crates.io/crates/validator
        info!("⏳ Validando dados do json de pedidos");
        if let Err(errors) = payload.validate() {
            let toast = export_validations(&errors, "Preencha os campos corretamente");
            let erro = Error::Form(toast);
            error!("{:?}", erro);
            return erro.into();
        }; 

        // 2. Usando ORM ou Repository
        let result = EntidadePedido::create(&pool.database, &payload).await; 
        match result{  
            Ok(pedido) => ApiResponse::new().with_data(json!(pedido)).send(),
            Err(err) => HttpResponse::InternalServerError().content_type("application/json")
                        .json(Toast::from(err))
        }
    }


    #[post("/pedido/{id}")]
    async fn atualizar_pedido_from_form(
        data: web::Data<AppState>, 
        payload: web::Form<PayloadPedido>,
        path: web::Path<i32>

        ) -> impl Responder {
        let payload = payload.into_inner();
        let result = EntidadePedido::update(&data.database, path.into_inner(), &payload).await;
 
        match result{  
            Ok(pedido) => ApiResponse::created("pedido alterado com sucesso", json!(pedido)).send(),
            Err(err) => HttpResponse::InternalServerError().content_type("application/json")
                        .json(Toast::from(err))
        }
    }

    #[post("/pedido/{id}")]
    async fn atualizar_pedido_from_json(
        data: web::Data<AppState>, 
        payload: web::Json<PayloadPedido>,
        path: web::Path<i32>,
        
    ) -> impl Responder {
       
        let payload = payload.into_inner();
        let result = EntidadePedido::update(&data.database, path.into_inner(), &payload).await;
 
        match result{  
            Ok(pedido) => ApiResponse::created("pedido alterado com sucesso", json!(pedido)).send(),
            Err(err) => HttpResponse::InternalServerError().content_type("application/json")
                        .json(Toast::from(err))
    }
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(inserir_pedido_from_form)
        .service(inserir_pedido_from_json)
        .service(atualizar_pedido_from_form)
        .service(atualizar_pedido_from_json)
        ;
}