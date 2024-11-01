use actix_session::Session;
use actix_web::HttpResponse;
use serde_json::json;
use sqlx::{Pool, Postgres};

 use crate::infra::error::Result;

use super::{model::{User, UserPermission, UserOperation}, repo::abrir_usuario_from_id};

pub async fn has_permission(
    pool: &Pool<Postgres>,
    session: &Session,
    operation: UserOperation, //read
    permission: UserPermission, //book
) -> bool {
        
    if let Some(user_id) = session.get::<String>("user_id").unwrap() {
        // dbg!(operation, permission);
        
        let found = sqlx::query!(
            r#" 
          SELECT COUNT(*) as count 
            FROM permissao
            WHERE id_usuario = $1 AND 
            ( operation  =  $2  or  operation  = 'all') AND 
              permissao  =  $3 
            "#,
            user_id,
            operation as UserOperation,
            permission as UserPermission,
        )
    .fetch_one(pool)
    .await;

    
        found.unwrap().count.unwrap() >= 1 
    } 
    else {
        //session.inser(error, o usuariO nao possui a permissão {operation} {permission})
        false
    }
   
}

pub async fn get_user(    
    pool: &Pool<Postgres>,
    session: &Session,

) -> Option<User> {
    if let Some(user_id) = session.get::<String>("user_id").unwrap() {
        dbg!(user_id.clone());
        abrir_usuario_from_id(pool, &user_id).await 
    } else {
        print!("user_id nao informado");
        None
    }

}

pub async fn has_logged(
    _pool: &Pool<Postgres>,
    session: &Session,
) -> bool {
        
    if let Some(_user_id) = session.get::<String>("user_id").unwrap() {
        return true
    } else {return false}

}

pub fn user_has_not_permission(permission: &str) -> Result<HttpResponse> {
    let error_response = json!({
                "code": 403,
                "error": "Forbidden",
                "message": "O usuário não possui permissão",                
                "required": permission  ,
            });
            let status_code = actix_web::http::StatusCode::FORBIDDEN;
            Ok(HttpResponse::build(status_code).json(error_response))
}