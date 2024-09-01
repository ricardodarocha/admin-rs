use sqlx::FromRow;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Entidade {
    pub id: String,
    pub nome: String, 
}

#[derive(FromForm)]
pub struct PostEntidadeForm {
    pub id: Option<String>,
    pub nome: String, 
}


#[derive(FromForm)]
pub struct PutEntidadeForm {
    pub nome: Option<String>, 
}

