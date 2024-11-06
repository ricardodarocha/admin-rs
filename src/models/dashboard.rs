use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Menu{
    pub titulo:String,
    pub icone:String,
    pub link:String,
}