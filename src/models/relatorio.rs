use serde::Serialize;

#[derive(Serialize)]
pub struct VendasMes {
    pub ano: Option<String>,
    pub mes_numero: Option<String>,
    pub mes: Option<String>,
    pub total: Option<f64>,
} 