use actix_web::{http::header::ContentType, HttpResponse};
use minijinja::{context, Value };
// use rust_decimal::Decimal;
use serde_json::json;
use time::{format_description, OffsetDateTime};
use crate::infra::result::Result;

use once_cell::sync::Lazy;

// /// Helper function that is added to templates to invoke `url_for` on the bound request.
// fn url_for(name: &str, args: Rest<String>) -> Result<Value> {
//     CURRENT_REQUEST.with(|current_req| {
//         Ok(current_req
//             .borrow()
//             .as_ref()
//             .ok_or_else(|| {
//                 Error::new(
//                     ErrorKind::InvalidOperation,
//                     "url_for requires an http request",
//                 )
//             })?
//             .url_for(name, &args[..])
//             .map_err(|err| {
//                 Error::new(ErrorKind::InvalidOperation, "failed to generate url").with_source(err)
//             })?
//             .to_string()
//             .into())
//     })
// }

pub fn reject(mensagem: &str) -> Result<HttpResponse> {
        let result = || -> HttpResponse {
            let error_response = json!({
                "code": 426,
                "error": "Operação não autorizada",
                "message": mensagem,  
            });
            let status_code = actix_web::http::StatusCode::UPGRADE_REQUIRED;            
            HttpResponse::build(status_code).json(error_response)
        };
        
        Ok(result())    
}


pub fn reject_found<T>(mensagem: &str, found: T) 

    -> Result<HttpResponse> where T: serde::Serialize,
        
         {
        let result = || -> HttpResponse {
            let error_response = json!({
                "code": 422,
                "error": "Entidade já existe",
                "message": mensagem, 
                "found": &found 
            });
            let status_code = actix_web::http::StatusCode::UNPROCESSABLE_ENTITY;            
            HttpResponse::build(status_code).json(error_response)
        };
        
        Ok(result())    
}

pub fn reject_not_found(mensagem: &str, id: &str, kind: &str) 

    -> Result<HttpResponse> 
        
         {
        let result = || -> HttpResponse {
            let error_response = json!({
                "code": 404,
                "error": "Entidade não encontrada",
                "message": mensagem, 
                "type": kind, 
                "id": id 
            });
            let status_code = actix_web::http::StatusCode::NOT_FOUND;            
            HttpResponse::build(status_code).json(error_response)
        };
        
        Ok(result())    
}

pub fn render_minijinja(template_name: &str, ctx: Value) -> Result<HttpResponse> {
    
    
    
    let (template_to_render, contexto) = match TEMPLATE.get_template(template_name) {

        Ok(tmpl) => (tmpl, ctx),
        Err(e) => {
            let msg_error = format!("error: {:#?}", e);
            let error_context = context!(msg_error);
  
           (TEMPLATE.get_template("error/error.html").unwrap(), error_context)
        }
    };

    let rendered = template_to_render.render(contexto);

    let result = match rendered {
        Ok(result) => result,
        Err(error) => error.to_string()
    };
    dbg!(template_name);
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(result))
}

use std::fmt::Write;

fn fmt(value: f32, symbol: Option<&str>) -> Value {
    // Obtém o valor f32
    
    // Usa o símbolo fornecido ou "R$" como padrão
    let currency_symbol = symbol.unwrap_or("R$");
    
    // Formata a moeda com duas casas decimais
    let mut formatted = String::new();
    write!(formatted, "{} {:.2}", currency_symbol, value).unwrap();
    formatted = formatted.replacen(".", ",", 1);
    
    // Retorna o valor formatado como string
    Value::from(formatted)
}

fn fmt3(value: f32, symbol: Option<&str>) -> Value {
    // Obtém o valor f32
    
    // Usa o símbolo fornecido ou "R$" como padrão
    let currency_symbol = symbol.unwrap_or("");
    
    // Formata a moeda com duas casas decimais
    let mut formatted = String::new();
    write!(formatted, "{} {:.3}", currency_symbol, value).unwrap();
    formatted = formatted.replacen(".", ",", 1);
    
    // Retorna o valor formatado como string
    Value::from(formatted)
}

fn fmtdate(value: Value) -> Value {
    // Obtém o OffsetDateTime
    let datetime = value
        .as_object()
        .and_then(|obj| obj.downcast_ref::<OffsetDateTime>()).unwrap();
    
    // Define o formato brasileiro "dd/mm/yyyy"
    let format = format_description::parse("[day]/[month]/[year]").unwrap();
    let formatted = datetime.format(&format);

    match formatted {
        Ok(value) => { Value::from(value) } ,
        Err(_) => { Value::from(value) },
    }
    
    // Retorna a data formatada como string
}

fn fmtdateopt(value: Value) -> Value {
    // Tenta converter o Value para uma string
    if let Some(date_str) = value.as_str() {
        // Define o formato do OffsetDateTime a ser parseado
        let format_in = format_description::parse("[offset_hour sign:mandatory][year]-[month]-[day]T[hour]:[minute]:[second].[subsecond]Z").unwrap();

        // Faz o parse da string no formato específico para OffsetDateTime
        if let Ok(datetime) = OffsetDateTime::parse(date_str, &format_in) {
            // Define o formato de saída "dd/mm/yyyy"
            let format = format_description::parse("[day]/[month]/[year]").unwrap();
            let formatted = datetime.format(&format).unwrap();
            return Value::from(formatted);
        }
    }
    
    // Se a conversão falhar, retorna uma string vazia
    Value::from("")
}

fn fmttime(value: Value) -> Value {
    // Obtém o OffsetDateTime
    let datetime = value
        .as_object()
        .and_then(|obj| obj.downcast_ref::<OffsetDateTime>()).unwrap();
    
    // Define o formato brasileiro "dd/mm/yyyy"
    let format = format_description::parse("[hour]:[minute] [second]s").unwrap();
    let formatted = datetime.format(&format);

    match formatted {
        Ok(value) => { Value::from(value) } ,
        Err(_) => { Value::from(value) },
    }
    
    // Retorna a data formatada como string
}

fn fmttimeopt(value: Value) -> Value {
    // Tenta converter o Value para uma string
    if let Some(date_str) = value.as_str() {
        // Define o formato do OffsetDateTime a ser parseado
        let format_in = format_description::parse("[offset_hour sign:mandatory][year]-[month]-[day]T[hour]:[minute]:[second].[subsecond]Z").unwrap();

        // Faz o parse da string no formato específico para OffsetDateTime
        if let Ok(datetime) = OffsetDateTime::parse(date_str, &format_in) {
            // Define o formato de saída "dd/mm/yyyy"
            let format = format_description::parse("[hour]:[minute] [second]s").unwrap();
            let formatted = datetime.format(&format).unwrap();
            return Value::from(formatted);
        }
    }
    
    // Se a conversão falhar, retorna uma string vazia
    Value::from("")
}

static TEMPLATE: Lazy<minijinja::Environment<'static>> = Lazy::new(|| {
    let mut env = minijinja::Environment::new();
    
    env.add_filter("fmtdate", fmtdate);    
    env.add_filter("fmtdateopt", fmtdateopt);    
    env.add_filter("fmttime", fmttime);    
    env.add_filter("fmttimeopt", fmttimeopt);    
    env.add_filter("fmt", fmt);
    env.add_filter("fmt3", fmt3);

    minijinja_contrib::add_to_environment(&mut env);
    env.set_loader(minijinja::path_loader("templates"));
    // env.add_function("url_for", url_for);
    
    println!("Templates: ");
    for (name, _) in env.templates() {
        println!("{}", name);
    };
    env
});