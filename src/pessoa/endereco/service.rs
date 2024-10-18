use log::info;
use sqlx::PgPool;
use crate::pessoa::endereco::model::*;
use crate::pessoa::endereco::repo as repo;
use crate::infra::result::{Result, Error};
use crate::pessoa::endereco::result as res;

pub async fn upsert_endereco (

    pool: &PgPool, 
    endereco: BuscaEndereco,

    
) -> Result<res::Endereco> {

    /*
    requisito

    01. Salvar o endereco na tabela de "endereco", 
        a) armazenar o id da rua, id do bairro e cep
        b) armazenar o id da cidade, id do estado

        {endereco: Rua Oscar Niemayer, Nº 5
         complemento: Ap 399
         bairro: Centro,
         cidade: {id_cidade},
         estado: {id_estado},
         cep: 0000 }

    */
        info!("Buscando parâmetros {:?} ",  endereco.clone());
        let id_rua = repo::upsert_rua(pool, endereco.endereco.clone()).await?;
        info!("Rua cadastrada {}", id_rua.clone());
        let id_bairro = repo::upsert_bairro(pool, endereco.bairro.clone()).await?;
        info!("Bairro cadastrado {}", id_bairro.clone());
        
        // o sistema ainda não tem suporte a cadastrar cidades, pois as cidades são obtidas diretamente do IBGE
        let id_cidade = repo::abrir_cidade(pool, endereco.cidade.clone()).await;
        info!("Cidade localizada {} {:?}", id_cidade.clone(), endereco.cidade.clone());
        let id_estado = repo::abrir_estado(pool, endereco.estado.clone()).await;
        info!("Estado localizado {} {:?}", id_estado.clone(), endereco.estado.clone());
        
        let endereco_id = repo::upsert_endereco(
            pool, 
            id_rua, 
            id_bairro, 
            id_cidade, 
            id_estado, 
            endereco.cep.clone()
        ).await;
        
        if let Some(endereco_encontrado) = endereco_id {
            let id = endereco_encontrado.id;
            let rua = repo::abrir_rua_by_id(pool, &endereco_encontrado.id_rua).await.unwrap();
            let bairro = repo::abrir_bairro_by_id(pool, &endereco_encontrado.id_bairro).await.unwrap();
            let cidade = repo::abrir_cidade_by_id(pool, endereco_encontrado.id_cidade).await;
            let estado = repo::abrir_estado_by_id(pool, endereco_encontrado.id_estado).await ;
            let cep = endereco_encontrado.cep;

            Ok(res::Endereco {
                id,
                rua,
                bairro,
                cidade,
                estado,
                cep,
                complemento: None,
        })
    } else { Err(Error::Str(&"o repositório não conseguiu inserir e retornar o endereço")) }
}