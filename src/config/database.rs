use log::info;
use sqlx::{FromRow, Pool, Postgres};

#[allow(unused)]
#[derive(FromRow, serde::Deserialize)]
pub struct EntityId {
    pub id: String, //v7
}

pub trait DbConnGetter {
    type Output;
    fn get_conn(&self) -> &Self::Output;
}

#[derive(Clone)]
pub struct DbInstance {
    pub conn: Pool<Postgres>
}

impl DbInstance {
    pub async fn init() -> Self {
        Self { conn: get_database_connection().await }
    }
}

impl DbConnGetter for DbInstance {
    type Output = Pool<Postgres>;

    fn get_conn(&self) -> &Self::Output {
        &self.conn    
    }
}


#[derive(FromRow, serde::Deserialize)]
pub struct Profile {
    pub value: String,
}

pub async fn get_database_connection() -> Pool<Postgres> {
    dotenv::dotenv().ok();
    // let host = std::env::var("HOST").unwrap();
    // let database = std::env::var("DATABASE").unwrap();
    // let port = std::env::var("PORT").unwrap();
    // let user = std::env::var("USER").unwrap();
    // let password = std::env::var("PASSWORD").unwrap();
    
    // let database_url = format!(
    //     "postgres://{user}:{password}@{host}:{port}/{database}"
    // );

    
    let database_url = std::env::var("DATABASE_URL").unwrap();
    info!("{url}", url=database_url.clone());
    let conn = sqlx::postgres::PgPool::connect(&database_url)
     .await.unwrap();
    
    //  let mut tx = conn.begin().await.unwrap();
    
    // let query_result = sqlx::query_as::<_, Profile>("select $1 as value")
    //     .bind("1".to_string())
    //     .fetch_one(&mut tx)
    //     .await;

    //  match query_result {
    //     Ok(profile) => {
    //         println!("Profile: {:?}", profile);
    //         _ = tx.commit().await;
    //     }
    //     Err(_) => {
    //         println!("Error: profile {} not found", 24324);
    //         let rollback_result = tx.rollback().await;
    //         println!("Rollback result: {:?}", rollback_result);
    //     }
    // }
    conn
}