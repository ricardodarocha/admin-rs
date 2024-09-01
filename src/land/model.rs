use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Menu {
    pub caminho: Option<String>,
    pub classe: Option<String>,
    pub titulo: Option<String>,
    pub descricao: Option<String>,
    pub contexto: String,
}

#[derive(Debug, Serialize)]
pub struct Item {
    pub nome: String,
    pub sku: String,
    pub quantidade: u32,
    pub preco: f64,
    pub total: f64,
}

#[derive(Debug, Serialize)]
pub struct Pedido {
    pub pedido_number: String,
    pub pedido_date_time: String,
    pub inventory: String,
    pub deliver: String,
    pub endereco: String,
    pub itens: Vec<Item>,
    pub total: f64,
    pub desconto: f64,
}

impl Pedido {
    pub fn new(
        pedido_number: &str,
        pedido_date_time: &str,
        inventory: &str,
        deliver: &str,
        endereco: &str,
        itens: Vec<Item>,
        total: f64,
        desconto: f64,
    ) -> Self {
        Pedido {
            pedido_number: pedido_number.to_string(),
            pedido_date_time: pedido_date_time.to_string(),
            inventory: inventory.to_string(),
            deliver: deliver.to_string(),
            endereco: endereco.to_string(),
            itens,
            total,
            desconto,
        }
    }
}


#[derive(Debug, Serialize)]
pub struct PedidoSummary {
    pub numero: Option<String>,
    pub valor: Option<String>,
    pub responsavel: Option<String>,
    pub status: Option<String>,
    pub data: Option<String>,
}