
use sqlx::FromRow;
use sqlx::types::chrono::NaiveDateTime;


#[derive(FromForm)]
pub struct FormMaterial<'r> {
    nome: &'r str,
    preco: f32,
}

#[derive(Serialize, Deserialize)]
pub struct NovoMaterial {
    pub nome: String,
    pub jclasses: Option<String>,
    pub jespecificacoes: Option<String>,
    pub jvariacoes: Option<String>,
    pub empresa: Option<i32>,
    pub descricao: Option<String>,
    pub grupo: Option<i32>,
    pub categoria: Option<i32>,
    pub classe: Option<i32>,
    pub segmento: Option<i32>,
    pub setor: Option<i32>,
    pub codbarras: Option<String>,
    pub url: Option<String>,
    pub memorando: Option<Vec<u8>>, // BLOB
    pub preco: Option<f64>,
    pub custo: Option<f64>,
    pub estoque: Option<f64>,
    pub maximo: Option<f64>,
    pub minimo: Option<f64>,
    pub ideal: Option<f64>,
    pub ativo: Option<bool>,
    pub referencia: Option<String>,
    pub abc: Option<String>,
    pub composicao: Option<bool>,
    pub medida: Option<f64>,
    pub und: Option<String>,
    pub componente: Option<bool>,
    pub compra: Option<bool>,
    pub venda: Option<bool>,
    pub preco_compra: Option<f64>,
    pub und_compra: Option<String>,
    pub und_venda: Option<String>,
    pub fator_compra: Option<f64>,
    pub fator_venda: Option<f64>,
    pub nometemp: Option<String>,
    pub redireciona: Option<i32>,
    pub data: Option<NaiveDateTime>,
}
#[derive(Serialize, Deserialize, FromRow)]
pub struct Material {
    pub id: i32,
    pub nome: Option<String>,
    pub jclasses: Option<String>,
    pub jespecificacoes: Option<String>,
    pub jvariacoes: Option<String>,
    pub tipo: String,
    pub empresa: Option<i32>,
    pub descricao: Option<String>,
    pub grupo: Option<i32>,
    pub categoria: Option<i32>,
    pub classe: Option<i32>,
    pub segmento: Option<i32>,
    pub setor: Option<i32>,
    pub codbarras: Option<String>,
    pub url: Option<String>,
    pub memorando: Option<Vec<u8>>, // BLOB
    pub preco: Option<f64>,
    pub custo: Option<f64>,
    pub estoque: Option<f64>,
    pub maximo: Option<f64>,
    pub minimo: Option<f64>,
    pub ideal: Option<f64>,
    pub ativo: bool,
    pub referencia: Option<String>,
    pub abc: String,
    pub composicao: bool,
    pub medida: Option<f64>,
    pub und: String,
    pub componente: bool,
    pub compra: bool,
    pub venda: bool,
    pub preco_compra: Option<f64>,
    pub und_compra: Option<String>,
    pub und_venda: Option<String>,
    pub fator_compra: Option<f64>,
    pub fator_venda: Option<f64>,
    pub nometemp: Option<String>,
    pub redireciona: Option<i32>,
    // pub created_at: NaiveDateTime,
    
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
}

#[post("/mate/json", format = "json", data = "<material>")]
pub async fn newmate(mut db: Connection<DbMeuBanco>, material: Json<NovoMaterial>)  -> Result<Json<Material>, status::Custom<String>> {
    sqlx::query_as::<_, Material>("INSERT INTO produto (nome, preco, tipo) values (?, ?, 'material') returning *")
        .bind(&material.nome)
        .bind(material.preco)
        .fetch_one(&mut **db)
        .await
        .map(Json)
        .map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))
}

#[post("/mate", data = "<material>")]
pub async fn form_mate(mut db: Connection<DbMeuBanco>, material: Form<FormMaterial<'_>> ) -> Result<Json<Material>, status::Custom<String>> {
    sqlx::query_as::<_, Material>("INSERT INTO produto (nome, preco, tipo) values (?, ?, 'material') returning *")
        .bind(material.nome)
        .bind(material.preco)
        .fetch_one(&mut **db)
        .await
        .map(Json)
        .map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))
}

#[get("/mate")]
pub async fn getallmate(mut db: Connection<DbMeuBanco>) -> Result<Json<Vec<Material>>, status::Custom<String>> {
    sqlx::query_as::<_, Material>("SELECT * FROM produto where tipo = 'material'")
        .fetch_all(&mut **db)
        .await
        .map(Json)
        .map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))
}

#[get("/mate/<id>")]
pub async fn getmate(mut db: Connection<DbMeuBanco>, id: i32) -> Result<Json<Material>, status::Custom<String>> {
    sqlx::query_as::<_, Material>("SELECT * FROM produto where id = ? and tipo = 'material'")
        .bind(id)
        .fetch_one(&mut **db)
        .await
        .map(Json)
        .map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))
}

