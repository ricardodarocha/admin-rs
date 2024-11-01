
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


use serde::{Serialize, Deserialize};
use crate::models::cliente::*;
use crate::models::produto::*;

#[derive(Clone, Serialize, Deserialize)]
    pub struct ItemModel {
        pub num_pedido: i64, //would be Pedido, but as Item is an item of Pedido, it does not make sense
        pub produto: Produto,
        pub quant: f32,
    }

///Reflect Business Model Logic
#[derive(Serialize, Deserialize)]
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

    #[derive(Clone, Serialize, Deserialize)]
    pub struct FormPedido {
        pub cliente: String,
    }

    #[derive(Clone, Serialize, Deserialize)]
    pub struct FormItem {
        pub num_pedido: i64,
        pub produto: Produto,
        pub quant: f32,
    }
