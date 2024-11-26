use actix_web::web::Path;
use actix_web::{get, web, HttpResponse, Responder};
use minijinja::context;
// use crate::models::QueryFiltro;
use crate::app::AppState;
use crate::repository::api::produtos::sqlite as repo;
use crate::repository::dashboard as repo_menus;

#[get("/cardapio/{nome}")]
async fn web_cardapio(
    
        data: web::Data<AppState>,
        path: Path<String>,

    ) -> impl Responder {
        
    let nome = path.into_inner();
    let pool = &data.database;
    let find_menus = repo_menus::carregar_menus(&pool).await;
    let menus = match find_menus {
        Ok(menus) => {
            menus
        }
        Err(_) => {
            vec!()
        }
    };
    let tmpl = data.render.get_template("admin/cardapio.html").unwrap();
    let cardapio = repo::abrir_cardapio(pool, &nome).await;

    if let Ok(cardapio) = cardapio {

        let rendered = tmpl.render(context! {
        title => "Cardápio",
        active_menu => "Editor de Cardápio",
        menus,
        cardapio
    }).unwrap();


        HttpResponse::Ok()
            .content_type("text/html")
            .body(rendered)
    } 
    else {  
          HttpResponse::NotFound()
        .finish()   
    }
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web_cardapio);
}