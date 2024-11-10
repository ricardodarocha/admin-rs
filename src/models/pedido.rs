
// Temos dois tipos de struct
// - DTO Representa as tabelas do banco de dados, com os exatos campos; usado para converter sql em struct
// - models, ou entidades, representa o modelo de negócio, possui link para as entidades relacionadas, posssui métodos 

// Também podemos ter outras entidades, como por exemplo ProdutoForm -> Proveniente do formulário HTMl ProdutoJson -> proveniente do json etc

/// Data Transform Object
/// Reflect exactly database structure
pub mod dto {
    use serde::{Serialize, Deserialize};
    #[derive(Serialize, Deserialize, sqlx::FromRow)]
    pub struct PedidoDto {
        pub num: String,
        // pub data
        pub cliente: String,
        pub valor: f32,
        pub status: String
    }
}

use std::collections::HashMap;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use sqlx::Pool;
use sqlx::Sqlite;
use crate::core::entidades::pedido::EntidadeCliente;
use crate::core::entidades::pedido::EntidadeItem;
use crate::core::tratados::ConsultaBd;
use crate::models::cliente::*;
use crate::models::QueryFiltroPedido;
use crate::infra::result::Result;
use crate::repository::pedidos::sqlite::abrir_lista_pedidos;
use validator::Validate;

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProdutoExiste {
    pub id: String,
    pub nome: Option<String>,
} 

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProdutoNovo {
    pub nome: String,
    pub descricao: Option<String>,
    pub preco: f32,
    pub eancode: Option<String>,
    // Dados minimos para inserir o produto
    // ...
    pub grupo: Option<String>,
    // Dados opcionais
    pub avatar: Option<String>,

    #[serde(flatten)]
    outros_campos: HashMap<String, Value>,

} 

#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostProduto {
    IdProduto(String),
    NovoProduto(ProdutoNovo),
    ProdutoJaExiste(ProdutoExiste),
}    

#[derive(Clone, Serialize, Deserialize)]
    pub struct PostItem { 
        pub produto: PostProduto,
        pub quant: f32,
    }
    
    #[derive(Clone, Serialize, Deserialize)]
    pub struct FormItem {
        pub num_pedido: i64, 
        pub produto: String,
        pub quant: f32,
    }
    
/// Recebe dados do pedido via json
/// Retorna o protocolo com os dados do pedido e os itens 
#[derive(Clone, Serialize, Deserialize, Validate)]
    pub struct PayloadPedido {

        /// número do pedido em outro sistema
        pub nosso_numero: Option<String>,

        /// data da criação    
        // pub data
        
        pub cliente: PostCliente,
        pub valor: Option<f64>,
        pub status: Option<String>,
        pub itens: Vec<PostItem>,
    }
    
/// Recebe dados do cliente para inserir novo pedido em branco via form
/// Retorna o protocolo com id do pedido
#[derive(Serialize, Deserialize, Validate)]
    pub struct NovoPedido {
        pub cliente: String,
    }

///Reflect Business Model Logic of Record in Dataset
#[derive(Clone, Serialize, Deserialize)]
    pub struct EntidadePedido {
        pub num: i64,
        // pub data
        pub cliente: EntidadeCliente,
        pub valor: f64,
        pub status: String,
        pub itens: Vec<EntidadeItem>,
    }

#[allow(dead_code)]
impl EntidadePedido {
    fn add_item(&mut self, item: EntidadeItem) -> &Self{

        let total_item = (item.quant * item.produto.preco) as f64;

        self.itens.push(item.clone());
        self.valor += total_item;
        self
    }
}

#[async_trait]
impl ConsultaBd for QueryFiltroPedido {
    type Entity = EntidadePedido;

    async fn get<'a >(pool: &'a Pool<Sqlite>, filtro: &'a Self) -> Result<Vec<Self::Entity>> {
        
        let consulta = abrir_lista_pedidos(&pool, &filtro.cliente, &filtro).await;

        consulta
    }
}
