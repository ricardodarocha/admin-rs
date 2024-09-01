use crate::land::model::*;
// use crate::infra::result::Result;
// use crate::infra::uuid::{generate_uuid, UuidKind};
use sqlx::{Pool, Postgres};


pub async fn get_menus(pool: &Pool<Postgres>, _user: String, contexto: &str) -> Option<Vec<Menu>> {
    let rec = sqlx::query_as!(
        Menu,
        "
        select caminho, classe, titulo, descricao, contexto from menus where sistema is null and contexto=$1 or contexto='main' order by seq 
        ",
        // user,
        contexto,
    )
    .fetch_all(pool)
    .await;

    match rec {
        Ok(rec) => Some(rec),
        Err(_) => None,
    }


}
