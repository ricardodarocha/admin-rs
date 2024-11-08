use serde::{Serialize, Deserialize};

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



#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EntidadeCliente {
    pub id: String,
    pub nome: String,
    pub cidade: String,
    pub avatar: String,

}

#[derive(Clone, Serialize, Deserialize)]
pub struct EntidadeItem {
    pub num_pedido: i64, //would be Pedido, but as Item is an item of Pedido, it does not make sense
    pub produto: EntidadeProduto,
    pub quant: f32,
}

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EntidadeProduto {
    pub id: String,
    pub nome: String,
    pub descricao: String,
    pub preco: f32,
    pub avatar: String,
    
}