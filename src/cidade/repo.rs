use sqlx::{Pool, Postgres};
use crate::infra::error::Result;
use crate::infra::error::Error::Sqlx;

use crate::cidade::model::Estado;

pub async fn lista_estados (
    pool: &Pool<Postgres>,

) -> Result<Vec<Estado>> {

    let rec =
    sqlx::query_as!(
        Estado, r#"
        select * from estado "#,
        )
        .fetch_all(pool).await;

   match rec {
    Ok(rec) => Ok(rec),
    Err(err) => Err(Sqlx(err))
   }
}



