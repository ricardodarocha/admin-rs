use serde::Serialize;

#[derive(Serialize)]
pub struct Chart {
    pub id: i32,
    pub title: String,
    pub labels: Vec<String>,
    pub series: Vec<Series>,
}

#[derive(Serialize)]
pub struct Series {
    pub name: String,
    pub tipo: String, //bar line etc
    pub values: Vec<i32>,
    pub background_color: Vec<String>,
    pub border_color: Vec<String>,
    pub border_width: i32,
}
