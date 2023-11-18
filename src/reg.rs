use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::form::Form;
use rocket_db_pools::Connection;
use sqlx::FromRow;

use crate::db::DbMeuBanco;

#[derive(Deserialize, Serialize, FromRow)]
pub struct NewUser {
    username: String,
}

// #[derive(Deserialize, Serialize)]
// pub struct Token {
//     token: String,
// }

#[derive(FromForm)]
#[allow(dead_code)]
pub struct RegUser<'r> {
    username: String,

    #[field(name = "password")]
    password: &'r str,
    #[field(validate = eq(self.password))]
    #[field(validate = omits("no"))]
    confirm: &'r str,
}


#[post("/register", data = "<user_form>")]
pub async fn register(mut db: Connection<DbMeuBanco>, user_form: Form<RegUser<'_>>) -> Result<Json<NewUser>, status::Custom<String>> {
    let password_hash = format!("{:x}", md5::compute(user_form.password));

   sqlx::query_as::<_, NewUser>("INSERT INTO users (name, password) values (?, ?) returning name")
        .bind(&user_form.username)
        .bind(password_hash)
        .fetch_one(&mut **db)
        .await
        .map(Json)
        .map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))
}

