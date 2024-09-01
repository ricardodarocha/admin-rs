use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use crate::infra::pagination::*;

#[derive(Deserialize)]
pub struct ProdutoPagination {
    #[serde(flatten)]
    pub pagination: Pagination
}

impl ProdutoPagination {
    pub fn setup(page: u32, size: u32) -> Self {
        let pagination = Pagination{page, size};
        Self {
            pagination        
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProdutoList {
    pub codigo: i32,
    pub id: String,
    pub id_: Option<String>,
    pub nome: String,
    pub codbarras: Option<String>,
    pub descricao: Option<String>,
    pub preco: Option<Decimal>,
    pub estoque: Option<f32>,
    pub id_categoria_produto: String,
    pub id_grupo_produto: String,

}

#[derive(Serialize, Deserialize)]
pub struct Produto {
    pub codigo: i32,
    pub classe: Option<i32>,
    pub segmento: Option<i32>,
    pub setor: Option<i32>,
    pub preco: Option<Decimal>,
    pub custo: Option<Decimal>,
    pub estoque: Option<f32>,
    pub maximo: Option<f32>,
    pub minimo: Option<f32>,
    pub ideal: Option<f32>,
    pub ativo: bool,
    pub composicao: bool,
    pub medida: Option<f32>,
    pub componente: bool,
    pub compra: bool,
    pub venda: bool,
    pub preco_compra: Option<f32>,
    pub fator_compra: Option<f32>,
    pub fator_venda: Option<f32>,
    pub avatar: Option<String>,

    #[serde(with = "time::serde::iso8601")]
    pub created: OffsetDateTime,

    pub id_categoria_produto: String,
    pub id: String,
    pub nome: String,
    pub jclasses: Option<String>,
    pub jespecificacoes: Option<String>,
    pub jvariacoes: Option<String>,
    pub tipo: String,
    pub referencia: Option<String>,
    pub descricao: Option<String>,
    pub abc: String,
    pub nometemp: Option<String>,
    pub redireciona: Option<String>,
    pub und: String,
    pub id_unidade: Option<String>,
    pub icone: Option<String>,
    pub codbarras: Option<String>,
    pub url: Option<String>,
    pub memorando: Option<String>,
    pub id_empresa: Option<String>,
    pub tamanho: Option<String>,
    pub formato: Option<String>,
    pub und_compra: Option<String>,
    pub und_venda: Option<String>,
    pub id_grupo_produto: String,
    pub campo1: Option<Decimal>,
    pub campo2: Option<Decimal>,
    pub campo3: Option<Decimal>,
    pub campo4: Option<Decimal>,
    pub campo5: Option<Decimal>,
    pub campo6: Option<Decimal>,
    pub campo7: Option<Decimal>,
    pub campo8: Option<Decimal>,
    pub campo9: Option<Decimal>,
    pub nome_campo1: Option<String>,
    pub nome_campo2: Option<String>,
    pub nome_campo3: Option<String>,
    pub nome_campo4: Option<String>,
    pub nome_campo5: Option<String>,
    pub nome_campo6: Option<String>,
    pub nome_campo7: Option<String>,
    pub nome_campo8: Option<String>,
    pub nome_campo9: Option<String>,
    pub id_unidade1: Option<String>,
    pub id_unidade2: Option<String>,
    pub id_unidade3: Option<String>,
    pub id_unidade4: Option<String>,
    pub id_unidade5: Option<String>,
    pub id_unidade6: Option<String>,
    pub id_unidade7: Option<String>,
    pub id_unidade8: Option<String>,
    pub id_unidade9: Option<String>,
    pub decimais1: Option<i32>,
    pub decimais2: Option<i32>,
    pub decimais3: Option<i32>,
    pub decimais4: Option<i32>,
    pub decimais5: Option<i32>,
    pub decimais6: Option<i32>,
    pub decimais7: Option<i32>,
    pub decimais8: Option<i32>,
    pub decimais9: Option<i32>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct PostProduto {
    pub id: Option<String>,
    pub nome: String,
    pub codbarras: Option<String>,
    pub descricao: Option<String>,
    pub id_grupo_produto: String,
    pub preco: Decimal,
    pub estoque: f32,
    pub marca: Option<String>,
    pub codfornecedor: Option<String>,
}
#[derive(Default, Serialize, Deserialize)]
pub struct PutProduto {
    pub id: String,
    pub codbarras: Option<String>,
    pub nome: Option<String>,
    pub id_grupo_produto: Option<String>,
    pub preco: Option<Decimal>,
    pub estoque: Option<f32>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct PostProdutoFull {
    pub id: Option<String>,
    pub nome: String,
    pub tipo: String,
    pub descricao: String,
    pub categoria: Option<String>,
    pub codbarras: Option<String>,
    pub url: String,
    pub preco: f32,
    pub custo: f32,
    pub estoque: f32,
    pub ativo: bool,
    pub referencia: Option<String>,
    pub und: Option<String>,
    pub id_empresa: Option<String>,
    pub medida: Option<f32>,
    pub tamanho: Option<String>,
    pub formato: Option<String>,
    pub id_grupo_produto: String,
    pub id_categoria_produto: String,
    pub icone: Option<String>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct GrupoProduto {
    pub id: String,
    pub nome: Option<String>,
}
