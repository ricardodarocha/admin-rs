use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, Pool, Postgres};

#[derive(Serialize)]
pub struct PedidoForm {
    pub id: String,
    pub itens: Vec<ItemPedido>,
    pub lista_produtos: Vec<super::CadastroProduto>,
}

impl PedidoForm {
    pub async fn create(pool: &Pool<Postgres>, id_usuario: String, id_empresa: String, id_cliente: String) -> Self {

        let novo_pedido = super::repo::criar_pedido_editavel(pool, id_usuario, Some(id_empresa.clone()), Some(id_cliente.clone())).await;
        let id_pedido = novo_pedido.unwrap().id;
        Self::abrir(pool, id_pedido, id_empresa).await
    }

    pub async fn abrir(pool: &Pool<Postgres>, id_pedido: String, id_empresa: String) -> Self {
        
        let lista_produtos = super::repo::lista_produtos(pool, id_empresa).await.unwrap();
        
        match super::repo::abrir_pedido(pool, id_pedido.clone()).await {
            Some(pedido) => {
                PedidoForm { 
                    id: pedido.id,
                    itens: super::repo::abrir_item_pedido(pool, id_pedido.clone()).await,  //array de produtos do ultimo pedido em aberto
                    lista_produtos: lista_produtos, 
            }
        },
            None => PedidoForm { 
                id: id_pedido, 
                itens: vec![], 
                lista_produtos: lista_produtos,
            },
        }   
    }
}

#[derive(Serialize, FromRow)]
pub struct PedidoX {
    pub id: String,
    pub produtos: sqlx::types::Json<Vec<ItemPedidoAgg>> 
}

#[derive(Default, Debug, FromRow, Serialize, Deserialize)]
pub struct ItemPedido {
   pub numero: i32,
   pub preco: Option<Decimal>,
   pub id_pedido: String,
   pub id_produto: String,
   pub id_item: i32,
   pub url: Option<String>,
   pub produto: Option<String>,
   pub descricao: Option<String>,
   pub quantidade: Option<Decimal>,
   pub total: Option<Decimal>,
   pub unidade: Option<String>,
   pub medida1: Option<Decimal>,
   pub medida2: Option<Decimal>,
   pub medida3: Option<Decimal>,
   pub nome_medida1: Option<String>,
   pub nome_medida2: Option<String>,
   pub nome_medida3: Option<String>,
   pub id_unidade_medida1: Option<String>,
   pub id_unidade_medida2: Option<String>,
   pub id_unidade_medida3: Option<String>,
}

#[derive(Default, Debug, Serialize, Deserialize, sqlx::Type)]
pub struct ItemPedidoAgg {
   pub preco: Option<Decimal>,
   pub id_produto: String,
   pub url: String,
   pub produto: String,
   pub quantidade: Option<Decimal>,
}

#[derive(Default, Debug, Serialize, FromRow)]
pub struct Galeria {
    pub itens: Vec<GaleriaItem>,
}

#[derive(Clone, Default, Debug, Serialize, FromRow)]
pub struct GaleriaItem {
   pub id: String,
   pub nome: String,
   pub descricao: Option<String>,
   pub url: Option<String>, 
   pub preco: Option<Decimal>, 
   pub quantidade: Option<Decimal>, 
   pub total: Option<Decimal>, 
//    pub tags: Option<Vec<String>>,
   pub in_cart: Option<bool>,
//    pub options: ,
}

#[derive(Default, Debug, Serialize, FromRow)]
pub struct EntidadeItemPedido {
   pub id_pedido: String,
   pub id_produto: String,
   pub id_item: i32,
}

#[derive(Default, Debug, Deserialize)]
pub struct FormItem {
    pub quantidade: Decimal,
}

    
    