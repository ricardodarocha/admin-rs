use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Estado {
    pub codigo : i32,
    pub id : String,
    pub nome : String,
    pub _nome : String,
    pub siglauf : String,
}