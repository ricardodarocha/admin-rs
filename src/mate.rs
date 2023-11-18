use rocket::form::Form;
use sqlx::FromRow;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::{response::status, http::Status};
use rocket_db_pools::{sqlx, Connection};

use crate::DbMeuBanco;

#[derive(FromForm)]
pub struct FormMaterial<'r> {
    name: &'r str,
    price: f32,}

#[derive(Serialize, Deserialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct Material {
    id: i32,
    name: String,
    price: f32,
}

#[post("/mate/json", format = "json", data = "<material>")]
pub async fn newmate(mut db: Connection<DbMeuBanco>, material: Json<Material>)  -> Result<Json<Material>, status::Custom<String>> {
    sqlx::query_as::<_, Material>("INSERT INTO products (name, price, tipo) values (?/*name*/, ?/*price*/, 'material') returning id, name, price, tipo")
        .bind(&material.name)
        .bind(material.price)
        .fetch_one(&mut **db)
        .await
        .map(Json)
        .map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))
}

#[post("/mate", data = "<material>")]
pub async fn form_mate(mut db: Connection<DbMeuBanco>, material: Form<FormMaterial<'_>> ) -> Result<Json<Material>, status::Custom<String>> {
    sqlx::query_as::<_, Material>("INSERT INTO products (name, price, tipo) values (?/*name*/, ?/*price*/, 'material') returning id, name, price, tipo")
        .bind(material.name)
        .bind(material.price)
        .fetch_one(&mut **db)
        .await
        .map(Json)
        .map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))
}

#[get("/mate")]
pub async fn getallmate(mut db: Connection<DbMeuBanco>) -> Result<Json<Vec<Material>>, status::Custom<String>> {
    sqlx::query_as::<_, Material>("SELECT * FROM products where tipo = 'material'")
        .fetch_all(&mut **db)
        .await
        .map(Json)
        .map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))
}

#[get("/mate/<id>")]
pub async fn getmate(mut db: Connection<DbMeuBanco>, id: i32) -> Result<Json<Material>, status::Custom<String>> {
    sqlx::query_as::<_, Material>("SELECT * FROM products where id = ?/*id*/ and tipo = 'material'")
        .bind(id)
        .fetch_one(&mut **db)
        .await
        .map(Json)
        .map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))
}

