
use crate::infra::{result::Result, strings::anonimizar};
use crate::auth::model::Usuario;
use crate::infra::error::Error;
use log::error;
use serde_json::json;
use sqlx::{self, Pool, Sqlite};
use actix_web::http::StatusCode;

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
        let json = json!({"form": {
                        "email": "Este e-mail já está sendo usado"
                    },
                    "toast": "teste"});
        return Err(Error::Form(json))
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
    
    // Ver funcao de login
    let chave = format!("{email}-{senha}", email = email.to_lowercase());
    let password_hash = format!("{:x}", md5::compute(&chave));
    
    let _ = sqlx::query!(
        r#" insert into usuarios
                 (login, nome, senha, nivel) values
                 (LOWER($1),
                  $2,
                  $3, 
                  $4)
                "#,
        email,
        register_form.nome,
        password_hash,
        level,
    )
    .execute(pool)
    .await?;
    
    abrir_usuario(pool, &email).await
    
}

pub async fn abrir_usuario(pool: &Pool<Sqlite>, email: &str) -> Result<Usuario> {
    sqlx::query_as!(
        Usuario,
        r#" select login, nome, nivel from usuarios where LOWER(login) = LOWER($1)
                "#,
        email,
    )
    .fetch_one(pool)
    .await
    .map_err(Into::into)   
}

async fn verifica_email_usado(pool: &Pool<Sqlite>, email: &str) -> Result<bool> {
    let result: Result<u32, sqlx::Error> = sqlx::query_scalar(
        "select count() from usuarios where UPPER(login) = UPPER($1)")
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

pub async fn login(
    pool: &Pool<Sqlite>, 
    email: &str, 
    senha: &str)
 -> Result<bool> {
        //Esta chave deve ser compativel com a funcao registrar usuario
        let chave = format!("{email}-{senha}", email = email.to_lowercase());
        let password_hash = format!("{:x}", md5::compute(&chave));
            
        let result: Result<u32, sqlx::Error> = sqlx::query_scalar(
        "select count() from usuarios where lower(login) = lower($1) and senha = $2")
        .bind(email)
        .bind(password_hash)
        .fetch_one(pool)
        .await;
    
    match result {

        // Erro de SQL
        Err(err) => return Err(Error::Database(err.into())),

        // Verifica se retornou algum registro
        Ok(value)  =>  Ok (value > 0),
    
    }
}