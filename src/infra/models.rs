use serde::Serialize;

#[derive(Serialize)]
pub struct Coluna {
    pub nome: String,
    pub tipo: String
}

#[derive(Serialize)]
pub struct Colunas {
    pub itens: Vec<Coluna>
}

impl Colunas {
    pub fn new(colunas: Vec<&str>) -> Self {
        let itens = colunas.iter().map(|nome| Coluna {
            nome: nome.to_string(),
            tipo: "String".to_owned(),
        }).collect::<Vec<Coluna>>();

        Self {
            itens
        }
    }
}