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