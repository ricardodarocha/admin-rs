use crate::dashboard::model::*;

pub fn create_sample_order(pedido_id: String) -> Pedido {
    let itens = vec![
        Item {
            nome: "Item 1".to_string(),
            sku: "SKU123".to_string(),
            quantidade: 2,
            preco: 10.00,
            total: 20.00,
        },
        Item {
            nome: "Item 2".to_string(),
            sku: "SKU456".to_string(),
            quantidade: 1,
            preco: 15.00,
            total: 15.00,
        },
    ];

    Pedido::new(
        &format!("# {}", pedido_id),
        "2023-07-20",
        "Available",
        "2023-07-25",
        "123 Main St, Anytown, USA",
        itens,
        35.00,
        5.00,
    )
}