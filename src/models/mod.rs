pub mod dashboard;
pub mod relatorio;
pub mod grafico;
pub mod produto;
pub mod cliente;
pub mod pedido;

use serde::Deserialize;

/// Filtra uma entidade pelo id, por exemplo produto.id, cliente.id
#[derive(Deserialize)]
pub struct QueryId {
    pub id: String,
}

/// Obtém um pedido pelo num_pedido
#[derive(Deserialize)]
pub struct QueryPedido {
    pub num_pedido: String,
}
/// Obtém o enésimo item de um pedido, que tenha o id do produto informado, e o num_pedido informado
#[derive(Deserialize)]
pub struct QueryItem {
    pub num_pedido: String,
    pub id_produto: String,
}
/// Obtém o enésimo item de um pedido, que tenha o id do produto informado, e o num_pedido informado
#[serde_as]
#[derive(Debug, Clone, DefaultFromSerde, Deserialize)]
pub struct QueryFiltro {
    #[serde(default="default_page")]
    #[serde_as(as="DisplayFromStr")]
    pub page: u32,
    #[serde(default="default_size")]
    #[serde_as(as="DisplayFromStr")]
    pub size: u32,
}
/// Obtém os pedidos de um determinado cliente no período informado
#[serde_as]
#[derive(Debug, Clone, DefaultFromSerde, Deserialize)]
pub struct QueryFiltroPedido {
    
    pub cliente: String,

    #[serde(default="default_page")]
    #[serde_as(as="DisplayFromStr")]
    pub page: u32,
    #[serde(default="default_size")]
    #[serde_as(as="DisplayFromStr")]
    pub size: u32,
    pub data_inicial: Option<String>,
    pub data_final: Option<String>,
}
/// Obtém o enésimo item de um pedido, que tenha o id do produto informado, e o num_pedido informado
#[serde_as]
#[derive(Debug, Clone, DefaultFromSerde, Deserialize)]
pub struct QueryFiltroCliente {
    #[serde(default="default_page")]
    #[serde_as(as="DisplayFromStr")]
    pub page: u32,
    #[serde(default="default_size")]
    #[serde_as(as="DisplayFromStr")]
    pub size: u32,
    pub cidade: Option<String>
} 

use serde_default::DefaultFromSerde;
use serde_with::{serde_as, DisplayFromStr};

fn default_page() -> u32 {
    1
}

fn default_size() -> u32 {
    50
}

#[serde_as]
#[derive(Debug, Clone, DefaultFromSerde, Deserialize)]
pub struct Pagination {
    #[serde(default="default_page")]
    #[serde_as(as="DisplayFromStr")]
    pub page: u32,
    #[serde(default="default_size")]
    #[serde_as(as="DisplayFromStr")]
    pub size: u32,
}