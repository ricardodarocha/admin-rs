use serde::{Serialize, Deserialize, };
use sqlx::FromRow;

#[derive(Deserialize, Serialize, FromRow)]
pub struct City {
    pub name : String,
    pub region : String,
    pub country : String,
}

use sqlx::{Pool, Sqlite);
use crate::infra::error::Result;
use crate::infra::error::Error::Sqlx;

pub async fn list_all_city (
    pool: &Pool<Sqlite>,

) -> Result<Vec<City>> {

    let rec =
    sqlx::query_as!(
        City, r#"
        select * from city order by name"#,
        )
        .fetch_all(pool).await;

   match rec {
    Ok(rec) => Ok(rec),
    Err(err) => Err(Sqlx(err))
   }
}

