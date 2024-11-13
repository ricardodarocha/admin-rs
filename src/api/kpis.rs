use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::infra::jwt::jwt_secret;
use crate::infra::sessao_usuario::Sessao;

#[derive(Serialize, Deserialize)]
pub struct Kpi {
    valor: Option<String>,
    icone: String,
    titulo: String,
    variacao: Option<String>,
    decorator: String,
}

pub async fn get_kpis(_user: Option<Sessao>) -> Vec<Kpi> {
    vec![
        Kpi{ 
            valor: Some("3.456".to_string()), 
            icone: "bi bi-eye".to_string(), 
            titulo: "Visualizações".to_string(), 
            variacao: Some("0.95%".to_string()), 
            decorator: "bi bi-arrow-up text-green-600".to_string(),
        },
        Kpi{ 
            valor: Some("R$ 4520,00".to_string()), 
            icone: "bi bi-cart2".to_string(), 
            titulo: "Vendido".to_string(), 
            variacao: Some("6.90%".to_string()), 
            decorator: "text-green-600".to_string(),
        },
        Kpi{ 
            valor: Some("450".to_string()), 
            icone: "bi bi-bag".to_string(), 
            titulo: "Pedidos".to_string(), 
            variacao: Some("0.95%".to_string()), 
            decorator: "text-green-600".to_string(),
        },
        Kpi{ 
            valor: Some("370".to_string()), 
            icone: "bi bi-people".to_string(), 
            titulo: "Novos clientes".to_string(), 
            variacao: Some("-1.02%".to_string()), 
            decorator: "bi bi-arrow-down text-red-500".to_string(),
        },
    ]
}

#[get("/kpis/json")]
async fn json_all_kpis(
    // data: web::Data<AppState>,
    // query: web::Query<QueryFiltroKpi>,
    session: Session,
    
) -> impl Responder {

    let sessao_usuario = Sessao::from_session(&session, &jwt_secret()).unwrap();

    let kpis = get_kpis(sessao_usuario).await;

    HttpResponse::Ok()
            .content_type("application/json")
            .json(kpis)

    }


pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(json_all_kpis);
}