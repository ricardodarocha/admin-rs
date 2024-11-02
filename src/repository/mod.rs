pub mod cliente;
pub mod pedido;
pub mod produto;

use crate::auth::model::Usuario;
use crate::infra::error::Error;
use crate::infra::result::Result;
use crate::infra::strings::anonimizar;
use crate::infra::uuid::{generate_uuid, UuidKind};
use crate::models::produto::{Produto, FormProduto};
use crate::models::cliente::{Cliente, FormCliente};
use log::error;
// use minijinja::value;
use sqlx::{self, Pool, Sqlite};
use actix_web::{web, http::StatusCode};
// use crate::infra::error::Error;

pub async fn abrir_produto(pool: &Pool<Sqlite>, id: &String) -> Result<Produto> {
    sqlx::query_as!(
        Produto,
        r#" select
                 id,
                 descricao,
                 preco as "preco: f32",
                 avatar
            from produto
           where id = $1"#,
        id,
    )
    .fetch_one(pool)
    .await
    .map_err(Into::into)
}

pub async fn abrir_cliente(pool: &Pool<Sqlite>, id: &String) -> Result<Cliente> {
    sqlx::query_as!(
        Cliente,
        r#" select
                 id,
                 nome,
                 cidade,
                 avatar
            from cliente
           where id = $1"#,
        id,
    )
    .fetch_one(pool)
    .await
    .map_err(Into::into)
}

pub async fn atualizar_produto(
    pool: &Pool<Sqlite>, 
        id: &String,
        form: web::Form<FormProduto>,
            
    ) -> Result<Produto> {
    let _ = sqlx::query_as!(
        Produto,
        r#" update Produto set 
                 id = $1,
                 descricao = $2 ,
                 preco  = $3 
           where id = $1"#,
        id,
        form.descricao,
        form.preco,
    )
    .execute(pool)
    .await;

    abrir_produto(pool, id).await
}

pub async fn atualizar_cliente(
    pool: &Pool<Sqlite>, 
    id: &String,
    form: FormCliente,

    ) -> Result<Cliente> {
    
    let _ = sqlx::query_as!(
        Cliente,
        r#" update Cliente set 
                 id = $1,
                 nome = $2,
                 cidade = $3
           where id = $1"#,
        id,
        form.nome,
        form.cidade
    )
    .execute(pool)
    .await;

    abrir_cliente(pool, id).await
}
pub async fn inserir_produto(
    pool: &Pool<Sqlite>, 
    form: web::Form<FormProduto>

    ) -> Result<Produto> {

    let id = generate_uuid(UuidKind::V7);
    let _ = sqlx::query_as!(
        Produto,
        r#" insert into produto
                 (id,
                 descricao,
                 preco) values
                 ($1,
                 $2,
                 $3)
                "#,
        id,
        form.descricao,
        form.preco
    )
    .execute(pool)
    .await;
    // .map_err(Into::into)

   abrir_produto(pool, &id).await 

}

pub async fn inserir_cliente(
    pool: &Pool<Sqlite>, 
    form: FormCliente

    ) -> Result<Cliente> {
    
    let id = generate_uuid(UuidKind::V7);
    let _ = sqlx::query_as!(
        Cliente,
        r#" insert into cliente
                 (id,
                 nome,
                 cidade) values
                 ($1,
                 $2,
                 $3)
                "#,
        id,
        form.nome,
        form.cidade,
    )
    .execute(pool)
    .await;
    // .map_err(Into::into)
    abrir_cliente(pool, &id).await

}

pub async fn registrar_usuario(
    pool: &Pool<Sqlite>, 
    register_form: crate::auth::model::Registrar,
    level: &str,

) -> Result<Usuario> {

    // Certifica que este e-mail ainda não foi usado
    let email = &register_form.email;
    let email_usado = verifica_email_usado(pool, &email).await?;
    
    if email_usado == true  {
        
        error!("E-mail já está em uso {}", anonimizar(email.as_ref()));
        return Err(Error::Detailed { 
                code: StatusCode::CONFLICT,
                msg: "Este email já foi usado".to_string(),
                description: "O email fornecido já está associado a uma conta existente.".to_string(),
                how_to_solve: "Informe um outro email que ainda não tenha sido usado".to_string()
        })
    };

    //certifica que as senhas são iguais
    let (senha, confere) = (&register_form.senha, &register_form.repetir_senha);
    if senha != confere {
        error!("As duas senhas precisam ser iguais");
        return Err(Error::Detailed { 
                code: StatusCode::BAD_REQUEST,
                msg: "Senha inválida".to_string(),
                description: "".to_string(),
                how_to_solve: "As duas senhas precisam ser iguais.".to_string()
        })
    };
    
    let _ = sqlx::query!(
        r#" insert into usuarios
                 (login, email, nome, senha, nivel) values
                 ($1,
                  $1,
                  $2,
                  $3, 
                  $4)
                "#,
        email,
        register_form.nome,
        register_form.senha,
        level,
    )
    .execute(pool)
    .await?;
    
    abrir_usuario(pool, &email).await
    
}

async fn abrir_usuario(pool: &Pool<Sqlite>, email: &str) -> Result<Usuario> {
    sqlx::query_as!(
        Usuario,
        r#" select login, nome, nivel from usuarios where email = $1
                "#,
        email,
    )
    .fetch_one(pool)
    .await
    .map_err(Into::into)   
}

async fn verifica_email_usado(pool: &Pool<Sqlite>, email: &str) -> Result<bool> {
    let result: Result<u32, sqlx::Error> = sqlx::query_scalar(
        "select count() from usuarios where login = $1")
        .bind(email)
        .fetch_one(pool)
        .await;
    
    match result {

        // Erro de SQL
        Err(err) => return Err(Error::Database(err.into())),

        // Verifica se o email ja foi usado pelo menos uma vez
        Ok(value)  =>  Ok (value > 0),
    }
}