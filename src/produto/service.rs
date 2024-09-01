use actix_session::Session;
use actix_web::HttpResponse;
use sqlx::PgPool;

// use crate::infra::render::reject_found;
use crate::infra::render::reject_not_found;
// use crate::admin::model::Empresa;
// use crate::admin::repo::atualizar_empresa;
use crate::infra::result::Result;
use crate::produto::model::*;
use actix_web::http::header::LOCATION;

use crate::produto::repo as repo;

pub async fn inserir_produto(

    pool: &PgPool, 
    id: Option<String>,
    id_empresa: String,
    produto_form: &PostProduto,
    _session: Session,
    
) -> HttpResponse {

    // let encontrou = repo::abrir_produto(pool, id_empresa.clone(), identificador).await;
    // if let Some(produto) = encontrou {
    //     return reject_found::<Produto>("Um produto com esta chave já existe", produto)};

    if let Some(_found) = id {
        HttpResponse::BadRequest().json("O produto já existe".to_owned())
    }   
    else 
    {
        match repo::inserir_produto(pool, id_empresa, id.unwrap(), produto_form).await 
        {
                Ok(_produto) => HttpResponse::SeeOther()
            .insert_header((LOCATION, "/produto/all"))
            .finish(),
                Err(e) => HttpResponse::BadRequest().json(e.to_string()),
        }
    }
}

pub async fn atualizar_produto(

    pool: &PgPool, 
    id: Option<String>,
    id_empresa: String,
    produto_form: &PostProduto,
    _session: Session,
    
) -> HttpResponse {

    // 
    // if let Some(produto) = encontrou {
    //     return reject_found::<Produto>("Um produto com esta chave já existe", produto)};

    if let Some(identificador) = id {
        
        let encontrou = repo::abrir_produto(pool, id_empresa.clone(), &identificador.clone()).await;
        if let None = encontrou {
            panic!("Produto não encontrado");
        } else {};

        match repo::atualizar_produto(pool, id_empresa, identificador.clone(), produto_form).await   
        {
                Ok(_produto) => HttpResponse::SeeOther()
            .insert_header((LOCATION, "/produto/all"))
            .finish(),
                Err(e) => HttpResponse::BadRequest().json(e.to_string()),
        }
    }   
    else 
    {

        panic!("Tentando atualizar um produto que não existe");
    }
}

pub async fn alterar_produto (

    pool: &PgPool, 
    id_empresa: String,
    produto_form: &PutProduto,
    _session: Session,

    
) -> Result<HttpResponse> {

    let id = &produto_form.id;
    let encontrou = repo::abrir_produto(pool, id_empresa.clone(), &id.clone()).await;
    if let None = encontrou {
        // return reject_found::<Produto>("Um produto com esta chave já existe", produto)};
        return reject_not_found(&"Produto não encontrado", &id.clone(), "Produto") };

    match repo::alterar_produto(pool, id_empresa, id.clone(), produto_form, ).await 
    {
            Ok(produto) => Ok(HttpResponse::Ok().json(produto)),
            Err(e) => Ok(HttpResponse::BadRequest().json(e.to_string())),
    }

}

