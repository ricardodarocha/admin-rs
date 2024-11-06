use sqlx::{Pool, Sqlite};
use crate::models;

pub async fn carregar_menus(pool: &Pool<Sqlite>) -> crate::infra::result::Result<Vec<crate::models::dashboard::Menu>> {
    sqlx::query_as!(models::dashboard::Menu,r#"SELECT titulo, icone, link FROM menus"#,)
        .fetch_all(pool)
        .await
        .map_err(Into::into)
}
