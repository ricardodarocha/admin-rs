#![allow(dead_code)]
#![allow(unused_variables)]
// use crate::infra::result::Result;

//Camada Semântica da aplicação
pub struct Aapp {

}

// pub fn render_minijinja<T, U>(template_name: &str, ctx: U) -> Result<T> {
//     let value: T = todo!();
//     Ok(value)
// }


pub trait EmailSender {
    fn send_email(to_email: &str, subject: &str, content: &str,);
}

pub trait WhatsappSender {
    fn send_message(to_number: &str, subject: &str, content: &str,);
}

pub trait AbstractRepository<T> {
    fn get_by_id(id: String) -> T;
    fn list<Q>(query: T) -> Vec<T>;
    fn post(form: T) -> T;
    fn put(form: T) -> Self;
    fn delete(id: String) -> Self;

}

pub mod authentication_suport {
   use super::AbstractRepository;
   use secrecy::Secret;

    pub struct Usuario {
        pub id: String,
        pub nome: String,
        pub email: String,
        pub senha: Secret<String>,
    }

     #[allow(unused_variables)]
    impl AbstractRepository<Usuario> for Usuario {
        fn get_by_id(id: String) -> Usuario {
            todo!()
        }
    
        fn list<Q>(query: Usuario) -> Vec<Usuario> {
            todo!()
        }
    
        fn post(form: Usuario) -> Usuario {
            todo!()
        }
    
        fn put(form: Usuario) -> Self {
            todo!()
        }
    
        fn delete(id: String) -> Self {
            todo!()
        }
    } 
}


pub mod multiempresa {
    use super::AbstractRepository;

    pub struct Empresa {
        pub id: String,
        pub nome: String,
    }

     #[allow(unused_variables)]
    impl AbstractRepository<Empresa> for Empresa {
        fn get_by_id(id: String) -> Empresa {
            todo!()
        }
    
        fn list<Q>(query: Empresa) -> Vec<Empresa> {
            todo!()
        }
    
        fn post(form: Empresa) -> Empresa {
            todo!()
        }
    
        fn put(form: Empresa) -> Self {
            todo!()
        }
    
        fn delete(id: String) -> Self {
            todo!()
        }
    }
}

pub mod contato_suport {
    use super::AbstractRepository;

    pub struct Contato {
        pub id: String,
        pub nome: String,
    }

    #[allow(unused_variables)]
    impl AbstractRepository<Contato> for Contato {
        fn get_by_id(id: String) -> Contato {
            todo!()
        }
    
        fn list<Q>(query: Contato) -> Vec<Contato> {
            todo!()
        }

        fn post(form: Contato) -> Contato {
            todo!()
        }
    
        fn put(form: Contato) -> Self {
            todo!()
        }
    
        fn delete(id: String) -> Self {
            todo!()
        }
    }
}

pub mod produto_suport {
    use rust_decimal::Decimal;

    use super::AbstractRepository;

    pub struct Produto {
        pub id: String,
        pub nome: String,
        pub preco: Decimal,
    }
 
    #[allow(unused_variables)]
    impl AbstractRepository<Produto> for Produto {
        fn get_by_id(id: String) -> Produto {
            todo!()
        }
    
        fn list<Q>(query: Produto) -> Vec<Produto> {
            todo!()
        }

        fn post(form: Produto) -> Produto {
            todo!()
        }
    
        fn put(form: Produto) -> Self {
            todo!()
        }
    
        fn delete(id: String) -> Self {
            todo!()
        }
    }
}

use authentication_suport::Usuario;
use multiempresa::*;
use contato_suport::*;
use produto_suport::*;
use rust_decimal::Decimal;

use crate::pedido::model::ItemPedido;

pub struct PedidoItem {
    pub id_empresa: String,
    pub id_cliente: String,
    pub id_produto: String,
    pub id_pedido: String,
    pub id_item: u32, //1 2 3 
    pub preco: Decimal,
    pub quantidade: Decimal,
    pub total: Decimal,
}

    #[allow(dead_code)]
    impl PedidoItem {
        fn empresa(self) -> Empresa {
            Empresa::get_by_id(self.id_empresa)
        }
        fn cliente(self) -> Contato {
            Contato::get_by_id(self.id_cliente)
        }
        fn produto(self) -> Produto {
            Produto::get_by_id(self.id_produto)
        }
        // fn pedido(self) -> Pedido {
        //     Pedido::get_by_id(self.id_pedido)
        // }
    }

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Pedido {
    pub codigo: u32,
    pub id: String,
    pub id_empresa: String,
    pub id_usuario: String,
    pub id_cliente: String,
    pub id_status_pedido: String,
    pub itens: Vec<ItemPedido>,
}

    #[allow(dead_code)]
    impl Pedido {
        fn new() -> Self {
            let codigo = Pedido::proximo_codigo();
            Pedido {
              codigo,
              .. Pedido::default()
            }
        }

        fn proximo_codigo() -> u32{
            0
        }
        
        fn empresa(self) -> Empresa {
            Empresa::get_by_id(self.id_empresa)
        }
        fn cliente(self) -> Contato {
            Contato::get_by_id(self.id_cliente)
        }
        fn usuario(self) -> Usuario {
            Usuario::get_by_id(self.id_usuario)
        }
        
        fn adicionar_item(self, item: ItemPedido) -> Self {
            let mut this = self;
            this.itens.push(item);
            this
        }

   
    }

    //maquinas de estado
    pub struct PedidoNovo {
        pub pedido: Pedido,
    }
    pub struct PedidoAprovado {
        pub pedido: Pedido,
    }
    // pub struct PedidoSeparado {
    //     pub pedido: Pedido,
    // }
    pub struct PedidoFaturado { //aguardando recebimento
        pub pedido: Pedido,
    }

     impl PedidoAprovado {
        fn faturar(self) -> PedidoFaturado {
            let pedido = self.pedido;
            PedidoFaturado {
               pedido 
            }
        }
    }
    // pub struct PedidoPago { //aguardando processamento
    //     pub pedido: Pedido,
    // }
    pub struct PedidoProcessado { //aguardando carregamento
        pub pedido: Pedido,
    }

     impl PedidoFaturado {
        fn processar_pagamento(self) -> PedidoProcessado {
            let pedido = self.pedido;
            PedidoProcessado {
               pedido 
            }
        }
    }
    pub struct PedidoDespachado { //aguardando confirmacao do cliente
        pub pedido: Pedido,
    }

     impl PedidoProcessado {
        fn despachar(self) -> PedidoDespachado {
            let pedido = self.pedido;
            PedidoDespachado {
               pedido 
            }
        }
    }
    pub struct PedidoEncerrado {
        pub pedido: Pedido,
    }
     impl PedidoDespachado {
        fn finalizar(self) -> PedidoEncerrado {
            let pedido = self.pedido;
            PedidoEncerrado {
               pedido 
            }
        }
    }

    impl PedidoNovo {
        fn aprovar(self) -> PedidoAprovado {
            let pedido = self.pedido;
            PedidoAprovado {
               pedido 
            }
        }
        fn cancelar(self) -> PedidoEncerrado {
            let pedido = self.pedido;
            PedidoEncerrado {
               pedido 
            }
        }
    }

pub struct Credencial {} //Persona, Vendedor, Cliente, Admin

impl Usuario {
    fn primeiro_acesso(self) -> Self {
        self
    }

    fn gerar_senha() {

    }

    fn enviar_email() {
        
    }

    fn adicionar_credencial(credencial: Credencial) {
        
    }
}


impl Aapp {
    fn get_form_cadastrar_usuario() {

    }
    fn get_form_cadastrar_produto() {

    }
    fn post_form_cadastrar_produto() {
        
    }
    fn get_form_cadastrar_pedido() {

    }
    fn post_form_cadastrar_pedido() {
        
    }
    fn post_form_cadastrar_item_pedido() {
        
    }

}