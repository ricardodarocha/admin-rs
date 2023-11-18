use rocket::{serde::{Serialize, Deserialize, json::Json}, response::status, http::Status, form::Form};
use rocket_db_pools::Connection;
use sqlx::FromRow;

use crate::db::DbMeuBanco;

#[derive(FromForm)]
pub struct FormProduct<'r> {
    name: &'r str,
    price: f32,}

#[derive(Serialize, Deserialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct Product {
    id: i32,
    name: String,
    price: f32,
}

#[post("/prod/json", format = "json", data = "<product>")]
pub async fn newprod(mut db: Connection<DbMeuBanco>, product: Json<Product>) -> Result<Json<Product>, status::Custom<String>> {
    sqlx::query_as::<_, Product>("INSERT INTO products (name, price, tipo) values (?/*name*/, ?/*price*/, 'produto') returning id, name, price, tipo")
        .bind(&product.name)
        .bind(product.price)
        .fetch_one(&mut **db)
        .await
        .map(Json)
        .map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))
}

#[post("/prod", data = "<produto>")]
pub async fn form_prod(mut db: Connection<DbMeuBanco>, produto: Form<FormProduct<'_>> ) -> Result<Json<Product>, status::Custom<String>> {
    sqlx::query_as::<_, Product>("INSERT INTO products (name, price, tipo) values (?/*name*/, ?/*price*/, 'produto') returning *")
        .bind(produto.name)
        .bind(produto.price)
        .fetch_one(&mut **db)
        .await
        .map(Json)
        .map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))
}

#[get("/prod")]
pub async fn getallprod(mut db: Connection<DbMeuBanco>) -> Result<Json<Vec<Product>>, status::Custom<String>> {
    sqlx::query_as::<_, Product>("SELECT * FROM products where tipo = 'produto'")
        .fetch_all(&mut **db)
        .await
        .map(Json)
        .map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))
}

#[get("/prod/<id>")]
pub async fn getprod(mut db: Connection<DbMeuBanco>, id: i32) -> Result<Json<Product>, status::Custom<String>> {
    sqlx::query_as::<_, Product>("SELECT * FROM products where id = ?/*id*/ and tipo = 'produto'")
        .bind(id)
        .fetch_one(&mut **db)
        .await
        .map(Json)
        .map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))
}

