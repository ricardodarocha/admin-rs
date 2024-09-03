use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct Job {
    pub id: i32,
    pub description: String,
    pub execute_at: OffsetDateTime,
    pub status: String,
}

pub trait Jober {
    fn run(&self, _job: Job) {}
}
