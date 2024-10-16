use serde::{Serialize, Deserialize};
use serde_json::Value;
use sqlx::prelude::FromRow;
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct Job {
    pub id: String,
    pub description: String,
    pub execute_at: OffsetDateTime,
    pub context: Value,
    pub status: String,
}

#[allow(async_fn_in_trait)]
pub trait Jober {
    async fn run(_job: Job) {}
}
