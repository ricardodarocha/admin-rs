use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};
use log::info;
use serde_json::json;
use crate::app::AppState;
use crate::infra::toast::{ApiResponse, Toast};
use crate::models::QueryFiltroPedido;
use crate::repository::pedidos::sqlite::{abrir_lista_pedidos, abrir_pedido};

#[get("/pedido/{id}")]
async fn consulta_pedido_por_id(
    data: web::Data<AppState>,
    _session: Session,
    path: web::Path<i64>,

) -> impl Responder {
            
    // 1. Valida Form
    // 2. Aciona Repository
    // 3. Retorna ApiResponse

    let result = abrir_pedido(&data.database, path.into_inner()).await; 
   
    match result{  
        Ok(pedido) => Toast::created("pedido inserido com sucesso")
                        .with_data(json!(pedido))
                        .send(),
        Err(err) => HttpResponse::InternalServerError().content_type("application/json")
                    .json(Toast::from(err))
    }

}

#[get("/json")]
async fn consulta_pedido_por_cliente(
    data: web::Data<AppState>,
    query: web::Query<QueryFiltroPedido>,

) -> impl Responder {
    
    let filtro = query.into_inner();
    info!("Busca pedidos do cliente: {:?}", filtro.cliente);

    let result = abrir_lista_pedidos(&data.database, &filtro.cliente, &filtro).await; 
   
    match result{  
        Ok(pedido) => ApiResponse::new().with_data(json!(pedido)).send(),
        Err(err) => HttpResponse::InternalServerError().content_type("application/json")
                    .json(Toast::from(err))
    }

}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(consulta_pedido_por_id)
        .service(consulta_pedido_por_cliente)
        ;
}

