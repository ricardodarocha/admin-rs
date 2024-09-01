pub struct AppState {
    pub database: crate::database::DbInstance,
    pub client: reqwest::Client
}