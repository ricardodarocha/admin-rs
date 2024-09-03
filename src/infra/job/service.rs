use std::collections::HashMap;
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use serde_json::Value;
use sqlx::PgPool;
use time::OffsetDateTime;

#[derive(Deserialize, Serialize, Clone)]
pub struct JobForm {
    pub description: String, 
    pub when: OffsetDateTime,
    #[serde(flatten)]
    pub context: serde_json::Value,
    // pub other: HashMap<String, Value>,
}

#[post("/todo")]
async fn create_todo(
    pool: web::Data<PgPool>,
    context: web::Json<JobForm>,
) -> impl Responder {

    let context  = context.into_inner();

    sqlx::query!(
        r#"
        INSERT INTO job (description, execute_at, context)
        VALUES ($1, $2, $3)
        "#,
        context.description,
        context.when,
        context.context,
    )
    .execute(pool.get_ref())
    .await
    .expect("Failed to insert job");

    HttpResponse::Ok().body("Job scheduled")
}