use serde::Serialize;
use time::{serde::iso8601::option, OffsetDateTime};

#[derive(Debug, Serialize)]
pub struct DashboardCard {
    pub id_usuario:  String,
    pub id_grupo_dashboard:  String,
    pub valor:  String,
    pub titulo:  String,
    pub descricao:  Option<String>,
    pub avatar:  Option<String>,
        
    #[serde(with = "time::serde::iso8601")]
    pub atualizado: OffsetDateTime, 
}
#[derive(Debug, Serialize)]
pub struct DashboardClientes {
    pub id:  String,
    pub nome:  Option<String>,
    pub avatar:  Option<String>,
        
    #[serde(rename = "data", with = "option")]
    pub created: Option<OffsetDateTime>, 
}
