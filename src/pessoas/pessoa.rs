use rocket::{serde::{Serialize, Deserialize, json::Json}, response::status, http::Status, form::Form};
use rocket_db_pools::Connection;
use sqlx::FromRow;

use crate::db::DbMeuBanco;

use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(FromForm)]
pub struct FormPessoa<'r> {
    nome: &'r str,
    cpf: f32,}
    
#[derive(Serialize, Deserialize)]
pub struct Pessoa {
    pub id: i32,

    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    
    #[serde(with = "time::serde::iso8601")]
    pub dataalteracao: OffsetDateTime,
    pub nome: String,
    pub tipopessoa: String,
    pub grupo: Option<i32>,
    pub perfil: Option<i32>,
    pub nacionalidade: Option<i32>,
    pub observacoes: Option<i32>,
    pub telefone: Option<String>,
    pub email: Option<String>,
    pub url: Option<String>,
    pub endereco: Option<String>,
    pub cidade: Option<i32>,
    pub uf: Option<String>,
    pub cep: Option<String>,
    pub credencial: Option<String>,
    pub senha: Option<String>,
    pub inscricao_federal: Option<String>,
    pub inscricao_estadual: Option<String>,
    pub abc: Option<String>,
    pub credito: Option<f64>,
    pub ativo: Option<bool>,
    pub bairro: Option<String>,
    pub empresa: Option<i32>,
    pub cliente: Option<bool>,
    pub fornecedor: Option<bool>,
    pub funcionario: Option<bool>,
    pub motorista: Option<bool>,
    pub transportador: Option<bool>,
    pub transportadora: Option<bool>,
    pub contador: Option<bool>,
    pub medico: Option<bool>,
    pub paciente: Option<bool>,
    pub vendedor: Option<bool>,
    pub empresa_id: Option<i32>,
}

// Add additional logic or functionality as needed


//POST
#[post("/prod/json", format = "json", data = "<Pessoa>")]
pub async fn newprod(mut db: Connection<DbMeuBanco>, Pessoa: Json<Pessoa>) -> Result<Json<Pessoa>, status::Custom<String>> {
    sqlx::query_as::<_, Pessoa>("INSERT INTO PESSOA (nome, cpf, tipo) values (?/*nome*/, ?/*cpf*/, 'PESSOA') returning id, nome, cpf, tipo")
        .bind(&Pessoa.nome)
        .bind(Pessoa.cpf)
        .fetch_one(&mut **db)
        .await
        .map(Json)
        .map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))
}

//POST WITH FORM
#[post("/prod", data = "<PESSOA>")]
pub async fn form_prod(mut db: Connection<DbMeuBanco>, PESSOA: Form<FormPessoa<'_>> ) -> Result<Json<Pessoa>, status::Custom<String>> {
    sqlx::query_as::<_, Pessoa>("INSERT INTO PESSOA (nome, cpf, tipo) values (?/*nome*/, ?/*cpf*/') returning *")
        .bind(PESSOA.nome)
        .bind(PESSOA.cpf)
        .fetch_one(&mut **db)
        .await
        .map(Json)
        .map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))
}

//GET ALL
#[get("/prod")]
pub async fn getallprod(mut db: Connection<DbMeuBanco>) -> Result<Json<Vec<Pessoa>>, status::Custom<String>> {
    sqlx::query_as::<_, Pessoa>("SELECT * FROM PESSOA where tipo = 'PESSOA'")
        .fetch_all(&mut **db)
        .await
        .map(Json)
        .map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))
}

//GET ONE
#[get("/prod/<id>")]
pub async fn getprod(mut db: Connection<DbMeuBanco>, id: i32) -> Result<Json<Pessoa>, status::Custom<String>> {
    sqlx::query_as::<_, Pessoa>("SELECT * FROM PESSOA where id = ?/*id*/'")
        .bind(id)
        .fetch_one(&mut **db)
        .await
        .map(Json)
        .map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))
}

