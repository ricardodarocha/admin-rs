use sqlx::{Pool, Postgres};

use crate::infra::result::Result;
use crate::infra::strings::tira_acento;
use crate::infra::uuid::{generate_uuid, UuidKind};
use crate::pessoa::endereco::model::*;

    pub async fn abrir_logradouro_by_id (
        pool: &Pool<Postgres>,
        id_endereco: &String,
    ) -> Option<Rua> {

        let logradouro = sqlx::query_as!(
            Rua, r#"
            select * from rua
            where id = $1"#,
            id_endereco)
            .fetch_optional(pool).await.unwrap();
        
        if let Some(logradouro_encontrado) = logradouro {
            Some(logradouro_encontrado)
        } else {
            None
        }
    }

    pub async fn abrir_bairro_by_id (
        pool: &Pool<Postgres>,
        id_bairro: &String,
    ) -> Option<Bairro> {

        let bairro = sqlx::query_as!(
            Bairro, r#"
            select * from bairro
            where id = $1"#,
            id_bairro)
            .fetch_optional(pool).await.unwrap();
        
        if let Some(bairro_encontrado) = bairro {
            Some(bairro_encontrado)
        } else {
            None
        }
    }

    pub async fn abrir_endereco_by_id (
        pool: &Pool<Postgres>,
        id_endereco: &String,
    ) -> Option<Endereco> {

        let endereco = sqlx::query_as!(
            Endereco, r#"
            select * from endereco
            where id = $1"#,
            id_endereco)
            .fetch_optional(pool).await.unwrap();
        
        if let Some(endereco_encontrado) = endereco {
            Some(endereco_encontrado)
        } else {
            None
        }
    }

   // Esta função assegura que um endereco válido será retornado. Se o endereço não for encontrado, então insere
    pub async fn upsert_endereco(
        pool: &Pool<Postgres>,
        id_logradouro: String,
        id_bairro: String,
        id_cidade: String,
        id_estado: String,
        cep: Option<String>
    ) -> Option<Endereco> {

        let id = generate_uuid(UuidKind::V7);

        let query = r#"
            INSERT INTO endereco (id, id_logradouro, id_bairro, id_cidade, id_estado, cep)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id_logradouro, id_bairro, id_cidade, id_estado, cep) DO NOTHING
            RETURNING id;
        "#;
        let id_endereco = sqlx::query_scalar(query)
            .bind(id)
            .bind(id_logradouro)
            .bind(id_bairro)
            .bind(id_cidade)
            .bind(id_estado)
            .bind(cep.unwrap_or_default())
            .fetch_one(pool)
            .await.unwrap();
        
        abrir_endereco_by_id(&pool, &id_endereco).await
    }



    // Função para inserir ou buscar logradouro
    pub async fn upsert_logradouro(
        pool: &Pool<Postgres>, 
        nome: Option<String>,
        ) -> Result<String> {
        
        let logradouro_nome = nome.unwrap_or_default();
        let logradouro_sem_acento = tira_acento(&logradouro_nome);
        let id_logradouro = generate_uuid(UuidKind::V7);

        let query = r#"
            INSERT INTO logradouro (id, nome, _nome)
            VALUES ($1, $2, $3)
            ON CONFLICT (_nome) DO IGNORE
            RETURNING id;
        "#;
        let id = sqlx::query_scalar(query)
            .bind(id_logradouro)
            .bind(logradouro_nome)
            .bind(logradouro_sem_acento)
            .fetch_one(pool)
            .await?;
        Ok(id)
    }

    // Função para inserir ou buscar bairro
    pub async fn upsert_bairro(pool: &Pool<Postgres>, nome: Option<String>) -> Result<String> {
        let bairro_nome = nome.unwrap_or_default();
        let bairro_sem_acento = tira_acento(&bairro_nome);
        let id_bairro = generate_uuid(UuidKind::V7);

        let query = r#"
            INSERT INTO bairro (id, nome, _nome)
            VALUES ($1, $2, $3)
            ON CONFLICT (_nome) DO IGNORE
            RETURNING id;
        "#;
        let id = sqlx::query_scalar(query)
            .bind(id_bairro)
            .bind(bairro_nome)
            .bind(bairro_sem_acento)
            .fetch_one(pool)
            .await?;
        Ok(id)
    }



    // Função para inserir ou buscar cidade
    // async fn upsert_cidade(pool: &PgPool, nome: Option<String>, estado: Option<String>) -> Result<String> {
    //     let cidade_nome = nome.unwrap_or_default();
    //     let cidade_sem_acento = tira_acento(&cidade_nome);
    //     let id_cidade = generate_uuid(UuidKind::V7);
    //     let id_estado = abrir_estado(pool, estado).await;
    //     let query = r#"
    //         INSERT INTO cidade (
    //             id,
    //             codigoestado,
    //             nome,
    //             _nome,
    //             codigoibge,
    //             uf,
    //             id_estado
    //         VALUES ($1, $2, $3)
    //         ON CONFLICT (_nome) DO IGNORE
    //         RETURNING id;
    //     "#;
    //     let id = sqlx::query_scalar(query)
    //         .bind(id_cidade)
    //         .bind(cidade_nome)
    //         .bind(cidade_sem_acento)
    //         .fetch_one(pool)
    //         .await?;
    //     Ok(id)
    // }

    // Função para buscar cidade
    pub async fn abrir_cidade(
            pool: &Pool<Postgres>, 
            nome: Option<String>,
        ) -> String {
        
        let cidade_nome = nome.unwrap_or_default();
        let cidade_sem_acento = tira_acento(&cidade_nome);

        let query = r#"
            select id 
            from cidade 
            where UPPER(id) = UPPER($1) 
            or UPPER(nome) = UPPER($1) 
            or UPPER(_nome) = UPPER($2);
        "#;
        let id: Option<String> = sqlx::query_scalar(query)
            .bind(cidade_nome)
            .bind(cidade_sem_acento)
            .fetch_optional(pool)
            .await.unwrap();

        if let Some(id) = id {
            id
        } else {
            "INDEFINIDO".to_owned()
        }    
    }
    
    pub async fn abrir_cidade_by_id(
            pool: &Pool<Postgres>, 
            id_cidade: Option<String>,
        ) -> Option<Cidade> {
        
        let id_cidade = if let Some(cidade_informada) = id_cidade {cidade_informada} else {"INDEFINIDO".to_owned()}; 
        let consulta = sqlx::query_as!(
            Cidade, r#"
            select * from cidade where id = $1;
        "#, id_cidade)
            .fetch_optional(pool)
            .await.unwrap();
        
        if let Some(encontrado) = consulta {
            Some(encontrado)
        }
        else
        {
            None
        }
    }

    // Função para buscar estado
    pub async fn abrir_estado(
            pool: &Pool<Postgres>, 
            nome: Option<String>,
        ) -> String {

        let estado_nome = nome.unwrap_or_default();
        let estado_sem_acento = tira_acento(&estado_nome);
        
        let query = r#"
            select id from estado where UPPER(sigla_uf) = UPPER($1) or nome = $1 or _nome = $2;
        "#;
        let id = sqlx::query_scalar(query)
            .bind(estado_nome)
            .bind(estado_sem_acento)
            .fetch_optional(pool)
            .await.unwrap_or(Some("INDEFINIDO".to_owned()));

        if let Some(id_encontrado) = id {
            id_encontrado
        } else {
            "INDEFINIDO".to_owned()
        }
    }
    
    pub async fn abrir_estado_by_id(
            pool: &Pool<Postgres>, 
            id_estado: Option<String>,
        ) -> Option<Estado> {
        
        let id_estado = if let Some(estado_informado) = id_estado {
            estado_informado
        } else {
            "INDEFINIDO".to_owned()
        }; 
        
        let consulta = sqlx::query_as!(
            Estado, r#"
            select * from estado where id = $1;
        "#, id_estado)
            .fetch_optional(pool)
            .await.unwrap();
        
        if let Some(encontrado) = consulta {
            Some(encontrado)
        }
        else
        {
            None
        }
    }