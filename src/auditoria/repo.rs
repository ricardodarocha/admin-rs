use std::net::IpAddr;

use crate::auditoria::model::*;
use crate::infra::uuid::{UuidKind::V7, generate_uuid};
use actix_web::web::Bytes;
use actix_web::HttpRequest;
use serde_json::{json, Value};
use sqlx::PgPool;
use crate::infra::result::Result;

pub async fn inserir_auditoria(
    pool: &PgPool,
    new_auditoria: &PostAuditoria,
) -> Result<()> {
    
    let id = generate_uuid(V7);

    let _rec = sqlx::query_as!(
        Auditoria,
        "insert into auditoria (
            id,
            id_empresa,
            id_usuario,
            id_perfil_usuario,
            tabela,
            valor_antigo,
            valor_novo,
            operacao)
            values (
            $1,  --id
            $2, --id_empresa
            $3, --id_usuario
            $4, --id_perfil_usuario
            $5, --tabela
            $6, --valor_antigo
            $7, --valor_novo
            $8  --operacao

        ) returning *",
        id,
        new_auditoria.id_empresa,
        new_auditoria.id_usuario,
        new_auditoria.id_perfil_usuario,
        new_auditoria.tabela,
        new_auditoria.valor_antigo,
        new_auditoria.valor_novo,
        new_auditoria.operacao,
    )
    .fetch_one(pool)
    .await?;

    Ok(())
}

pub async fn auditar_requisicao(
    pool: &PgPool,
    requisicao: &HttpRequest,
    // body: Bytes,
    id_empresa: &String,
    id_usuario: &String,
    scope: String,

) -> Result<()> {
    
    let id = generate_uuid(V7);

    let headers = extract_headers(requisicao).await;
    // let body = extract_body(requisicao, body).await;
    let id_servico = extract_uri(requisicao).await;
    let ip: Option<IpAddr> = if let Some(val) = requisicao.peer_addr() {
            Some(val.ip())
        } else 
        {
            None
        };
    let origim = format!("{:?}", ip);

    // id varchar(40) not null primary key,
	// id_usuario varchar(40) not null references users(id),
	// id_empresa varchar(40) not null references empresa(id),
	// id_servico_api varchar(40) not null references servico_api(id),
	// header text,
	// body text,
	// escope text,
	// response text,
	// status varchar(40),

    let _rec = sqlx::query_as!(
        Requisicao,
        "insert into requisicao (
            id,
            id_empresa,
            id_usuario,
            id_servico_api,
            origim,
            header,
            body,
            escope
            )
            values (
            $1, --id,
            $2, --id_empresa,
            $3, --id_usuario,
            $4, --id_servico,
            $5, --origem
            $6, --header,
            $7, --body,
            $8 --escope
        ) returning *",
        id,
        id_empresa,
        id_usuario,
        id_servico,
        origim,
        headers,
        // body.unwrap().to_string(),
        "".to_string(),
        scope,
    )
    .fetch_one(pool)
    .await?;

    Ok(())
}


use std::collections::HashMap;

async fn extract_headers(req: &HttpRequest) -> String {
    // Cria um HashMap para armazenar os cabeçalhos
    let mut headers_map: HashMap<String, String> = HashMap::new();

    // Itera sobre os cabeçalhos do HttpRequest
    for (key, value) in req.headers().iter() {
        if let Ok(value_str) = value.to_str() {
            headers_map.insert(key.to_string(), value_str.to_string());
        }
    }

    // Converte o HashMap para uma string JSON
    let json_string = serde_json::to_string(&headers_map).unwrap_or_else(|_| "{}".to_string());

    json_string
}

async fn _extract_body(req: &HttpRequest, body: Bytes) -> Option<Value> {
    
    let content_type = req.headers().get("Content-Type").and_then(|v| v.to_str().ok());
    let body = String::from_utf8_lossy(&body).into_owned();
        
    match content_type {
        Some("application/json") => {
            // Tenta parsear o corpo como JSON
            serde_json::from_str(&body).ok()
        }
        Some("application/x-www-form-urlencoded") => {
            // Parseia o corpo como FORM e converte para JSON
            let form_data: HashMap<String, String> = url::form_urlencoded::parse(body.as_bytes())
                .into_owned()
                .collect();
            Some(json!(form_data))
        }
        _ => None, // Retorna None se não for JSON nem FORM
    }
}

async fn extract_uri(req: &HttpRequest) -> String {
    req.uri().to_string()
}