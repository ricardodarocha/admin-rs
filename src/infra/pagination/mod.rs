use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

fn default_page() -> u32 {
    1
}

fn default_size() -> u32 {
    50
}

#[serde_as]
#[derive(Deserialize)]
pub struct Pagination {
    #[serde(default="default_page")]
    #[serde_as(as="DisplayFromStr")]
    pub page: u32,
    #[serde(default="default_size")]
    #[serde_as(as="DisplayFromStr")]
    pub size: u32,
}