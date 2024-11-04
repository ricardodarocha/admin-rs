
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

use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::models::cliente::*;
use crate::models::produto::*;

#[derive(Clone, Serialize, Deserialize)]
    pub struct ItemModel {
        pub num_pedido: i64, //would be Pedido, but as Item is an item of Pedido, it does not make sense
        pub produto: Produto,
        pub quant: f32,
    }


#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProdutoExiste {
    pub id: String,
    pub nome: Option<String>,
    pub avatar: Option<String>,
} 

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProdutoId(
    String
);
#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProdutoNovo {
    pub nome: String,
    pub descicao: Option<String>,
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
pub enum PostProduto {
    IdProduto(ProdutoId),
    NovoProduto(ProdutoNovo),
    ProdutoJaExiste(ProdutoExiste),
}    

#[derive(Clone, Serialize, Deserialize)]
    pub struct PostItem {
        pub num_pedido: i64, 
        pub produto: PostProduto,
        pub quant: f32,
    }
    
    #[derive(Clone, Serialize, Deserialize)]
    pub struct FormItem {
        pub num_pedido: i64, 
        pub produto: String,
        pub quant: f32,
    }

    
// Recebe dados do pedido via json
#[derive(Clone, Serialize, Deserialize)]
    pub struct PostPedido {
        pub num: Option<i64>,
        pub nosso_numero: Option<String>,
        // pub data
        pub cliente: PostCliente,
        pub valor: Option<f64>,
        pub status: Option<String>,
        pub itens: Vec<ItemModel>,
    }
    
// Recebe dados do pedido via form
#[derive(Serialize, Deserialize)]
    pub struct FormPedido {
        pub num: i64,
        pub cliente: String,
    }

///Reflect Business Model Logic
#[derive(Clone, Serialize, Deserialize)]
    pub struct PedidoModel {
        pub num: i64,
        // pub data
        pub cliente: Cliente,
        pub valor: f64,
        pub status: String,
        pub itens: Vec<ItemModel>,
    }

    #[allow(dead_code)]
    impl PedidoModel {
        fn add_item(&mut self, produto: ItemModel) -> &Self{

            let total_item = (produto.quant * produto.produto.preco) as f64;

            self.itens.push(produto.clone());
            self.valor += total_item;
            self
        }
    }
