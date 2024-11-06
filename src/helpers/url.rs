pub fn url(path: &str) -> String {
    let base_url = std::env::var("BASE_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());
    let path = if path.starts_with('/') { &path[1..] } else { path };
    format!("{base_url}/{path}")
}