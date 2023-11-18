use rocket::http::Status;
use rocket::response::status;
use rocket::serde::{Deserialize, Serialize};
use rocket::form::Form;
use rocket_db_pools::Connection;
use sqlx::FromRow;

use crate::db::DbMeuBanco;

#[derive(Deserialize, Serialize, FromRow)]
pub struct User {
    username: String,
    password_hash: String, // Armazena a senha como hash MD5
}

// #[derive(Deserialize, Serialize)]
// pub struct Token {
//     token: String,
// }

#[derive(FromForm)]
pub struct Login {
    username: String,
    password: String, // Senha em texto puro para comparação
}


#[post("/login", data = "<login_form>")]
pub async fn login(mut db: Connection<DbMeuBanco>, login_form: Form<Login>) -> Result<String, status::Custom<String>> {
    let password_hash = format!("{:x}", md5::compute(&login_form.password));

    match sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ? and password = ? limit 1")
        .bind(&login_form.username)
        .bind(password_hash)
        .fetch_optional(&mut **db).await {
        Ok(Some(_user)) => Ok("335b5a7b-ba69-5fa4-9388-3a181116daf4".to_string()),
        _ => Err(status::Custom(Status::InternalServerError, "Credencial inválida".to_string())),
    }
}

