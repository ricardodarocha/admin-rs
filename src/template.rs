use rocket::{serde::{Serialize, Deserialize, json::Json}, response::status, http::Status, form::Form};
use rocket_db_pools::Connection;
use sqlx::FromRow;

use crate::db::DbMeuBanco;

#[derive(FromForm)]
pub struct Form{$route}<'r> {
    name: &'r str,
    price: f32,}

#[derive(Serialize, Deserialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct {$route} {
    id: i32,
    name: String,
    price: f32,
}

//POST
#[post("/prod/json", format = "json", data = "<{$route}>")]
pub async fn newprod(mut db: Connection<DbMeuBanco>, {$route}: Json<{$route}>) -> Result<Json<{$route}>, status::Custom<String>> {
    sqlx::query_as::<_, {$route}>("INSERT INTO {$TABLENAME} (name, price, tipo) values (?/*name*/, ?/*price*/, '{$TABLENAME}') returning id, name, price, tipo")
        .bind(&{$route}.name)
        .bind({$route}.price)
        .fetch_one(&mut **db)
        .await
        .map(Json)
        .map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))
}

//POST WITH FORM
#[post("/prod", data = "<{$TABLENAME}>")]
pub async fn form_prod(mut db: Connection<DbMeuBanco>, {$TABLENAME}: Form<Form{$route}<'_>> ) -> Result<Json<{$route}>, status::Custom<String>> {
    sqlx::query_as::<_, {$route}>("INSERT INTO {$TABLENAME} (name, price, tipo) values (?/*name*/, ?/*price*/') returning *")
        .bind({$TABLENAME}.name)
        .bind({$TABLENAME}.price)
        .fetch_one(&mut **db)
        .await
        .map(Json)
        .map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))
}

//GET ALL
#[get("/prod")]
pub async fn getallprod(mut db: Connection<DbMeuBanco>) -> Result<Json<Vec<{$route}>>, status::Custom<String>> {
    sqlx::query_as::<_, {$route}>("SELECT * FROM {$TABLENAME} where tipo = '{$TABLENAME}'")
        .fetch_all(&mut **db)
        .await
        .map(Json)
        .map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))
}

//GET ONE
#[get("/prod/<id>")]
pub async fn getprod(mut db: Connection<DbMeuBanco>, id: i32) -> Result<Json<{$route}>, status::Custom<String>> {
    sqlx::query_as::<_, {$route}>("SELECT * FROM {$TABLENAME} where id = ?/*id*/'")
        .bind(id)
        .fetch_one(&mut **db)
        .await
        .map(Json)
        .map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))
}

