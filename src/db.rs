// src/db/mod.rs
use sqlx::{SqlitePool, Pool, Sqlite};
use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("sqlite_meu_banco")]
pub struct DbMeuBanco(sqlx::SqlitePool);
use rocket::fairing::AdHoc;

pub async fn init_db_pool(database_url: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    SqlitePool::connect(database_url).await
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Database Stage", |rocket| async {
        let database_url = rocket.figment().extract_inner::<String>("database.url")
            .unwrap_or_else(|_| "sqlite://./meu_banco.db".to_string());

        let db_pool = match init_db_pool(&database_url).await {
            Ok(pool) => pool,
            Err(e) => {
                panic!("Database connection failed: {}", e);
            }
        };

        rocket.manage(db_pool)
    })
}
