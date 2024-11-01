use actix_web::{get, post, web, HttpResponse, Responder};
use log::info;
use minijinja::context;
use crate::models::produto::FormProduto;
use crate::models as model;
use crate::app::AppState;
use crate::services::abrir_produto;


#[get("/produto/edit")]
async fn web_produto(
        data: web::Data<AppState>,
        query: web::Query<model::QueryId>, 

    ) -> impl Responder {
        
    let pool = &data.database;
    let tmpl = data.render.get_template("web/produto.html").unwrap();
    let web::Query(entidade_produto) = query;
    let produto = abrir_produto(pool, entidade_produto.id).await;

    if let Some(produto) = produto {
        let rendered = tmpl.render(context! {title => "produto", produto}).unwrap();

        HttpResponse::Ok()
            .content_type("text/html")
            .body(rendered)
    } 
    else {     
        
        let tmpl = data.render.get_template("shared/views/ajaxToast.html").unwrap();
        let rendered = tmpl.render(context! {
            toast_icon => "bi-check-circle",
            toast_class => "toast-error",
            toast_text => "Produto não encontrado!",
    }).unwrap();
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)

    }
    
}

#[post("/produto/edit")]
async fn web_produto_submit(
    form: web::Form<FormProduto>,
    data: web::Data<AppState>,

) -> impl Responder {
    
    info!("Recebido POST com dados: {:?}", form);

    let tmpl = data.render.get_template("shared/views/ajaxToast.html").unwrap();
    let rendered = tmpl.render(context! {
        toast_icon => "bi-check-circle",
        toast_class => "toast-success",
        toast_text => "Dados salvos com sucesso!",
    }).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)

    // HttpResponse::Ok()
    //     .content_type("application/json")
    //     .json(json!({
    //         "toast": "teste"
    //     }))
}