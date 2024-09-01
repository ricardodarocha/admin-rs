use serde::Deserialize;

fn default_page() -> u32 {
    1
}

fn default_size() -> u32 {
    50
}

#[derive(Deserialize)]
pub struct Pagination {
    #[serde(default="default_page")]
    pub page: u32,
    #[serde(default="default_size")]
    pub size: u32,
}