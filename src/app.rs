use std::sync::Arc;

use minijinja::Environment;
use sqlx::SqlitePool;

#[derive(Debug, Clone)]
pub struct AppState {
    pub database: SqlitePool,
    pub client: reqwest::Client,
    pub render: Arc<Environment<'static>>,
}