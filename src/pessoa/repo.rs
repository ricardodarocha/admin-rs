use crate::entidade::identificacao::repo::upsert_identificacao;
use crate::infra::error::Error;
use crate::{pessoa::model::*, infra::uuid::UuidKind};
use log::info;
use sqlx::{Pool, Postgres};
use crate::infra::uuid::generate_uuid;
use crate::infra::result::Result;
use crate::infra::error::Error::*;
use crate::entidade::EntidadeId;

pub async fn inserir_pessoa(
    pool: &Pool<Postgres>,
    pessoa: &PostPessoa,
) -> Result<Pessoa> {
    let id = match pessoa.id.clone() {
        Some(value) if value != "" => value,
        _ => generate_uuid(UuidKind::V7),
    };

    let tipo_pessoa = match  
        pessoa.cnpj.clone() {
            Some(_cnpj) => &"PJ",
            None => &"PF",
    };

    match tipo_pessoa {
        &"PF" => if let None = pessoa.cpf { return Err(Error::Str("Cpf requerido")) }, 
        &"PJ" => if let None = pessoa.cnpj { return Err(Error::Str("Cnpj requerido")) }, 
        _ => {},
    };

    let (tipo_identificacao, identificacao, ) = match tipo_pessoa {
        &"PF" => { (
            crate::entidade::identificacao::repo::abrir_tipo_identificacao(pool, &"CPF".to_owned()).await,
            pessoa.clone().cpf.unwrap(),
        ) }, 
        &"PJ" => { (
            crate::entidade::identificacao::repo::abrir_tipo_identificacao(pool, &"CNPJ".to_owned()).await,
            pessoa.clone().cnpj.unwrap(),
        ) }, 
        _ => {(None, "".to_owned())},
    };
 
    //verifica se a identificacao já foi inserida, se não tiver insere    
    match tipo_identificacao {
        Some(tipo) => { let _ = upsert_identificacao(pool, &identificacao, EntidadeId { id: tipo.id }, ).await;},
        None => {}
    }  

    let rec = sqlx::query_as!(
        Pessoa,
        "insert into pessoa (
	id,    
	razao_social,     
	nome,                
	id_tipo_pessoa, 
	id_identificacao, 
	id_status,
	id_email,  
	id_telefone    
) VALUES
( 
	$1, --id
	$2, --razao_social
	$3, --nome
	(select id from tipo_pessoa where simbolo = $4),  --tipo
	(select id from identificacao where descricao = $5), --identificacao
	(select id from status_pessoa where descricao = 'CREDENCIADO'), 
	(select contato.id from contato join tipo_contato ema on ema.id = contato.id_tipo_contato 
        where contato.descricao = $6 and ema.nome = 'EMAIL'),
	(select contato.id from contato join tipo_contato tel on tel.id = contato.id_tipo_contato 
        where contato.descricao = $7 and tel.nome = 'TELEFONE')
) returning *, '' as cpf, '' as cnpj",
        id,
        pessoa.razao_social,
        pessoa.nome,
        tipo_pessoa,
        identificacao,

        pessoa.email,
        pessoa.telefone,
    )
    .fetch_one(pool)
    .await?;

    Ok(rec)
}

pub async fn abrir_pessoa (
  pool: &Pool<Postgres>,
  id_empresa: String,
  identificador: &String, 

) -> Option<Pessoa> {  
  info!("looking for pessoa where id = {id}", id = identificador.clone());  
  let result = sqlx::query_as!(

    Pessoa, r#"
    select p.*,
    case when trg.simbolo = 'CPF' THEN rg.descricao ELSE '' end as cpf,
    case when trg.simbolo = 'CNPJ' THEN rg.descricao ELSE '' end as cnpj
     from pessoa p
    inner join empresa e on e.id = p.id_empresa
    left join identificacao rg on rg.id = p.id_identificacao
    left join tipo_identificacao trg on  trg.id = rg.id_tipo_identificacao
    where p.id <> '0' and p.id = $1 and p.id_empresa = $2 or e.id_empresa_pessoas = $2
    "#,
    identificador,
    id_empresa,
    )
    .fetch_optional(pool).await;

  if let Ok(value) = result {
    info!("Pessoa localizado");
    value
  }

  else {
    info!("Pessoa não encontrado");
    None
  }

}

pub async fn listar_pessoas_all(
    pool: &Pool<Postgres>,
    id_empresa: String,
    args: PessoaPagination,
) -> Result<Vec<PessoaList>> {
    let (limit, offset) = (
        args.pagination.size, 
        args.pagination.size * (args.pagination.page - 1),
    );
    
    let rec = sqlx::query_as!(
        PessoaList,
        "select 
        pessoa.id, 
        right(pessoa.id, 6) as id_,  
        pessoa.nome,
        pessoa.razao_social,
        tp.nome as tipo_pessoa,
        rg.descricao as identificacao,
        trg.simbolo as tipo_identificacao,
        sp.descricao as status,
        tel.descricao as telefone,
        ema.descricao  as email,
        case when trg.simbolo = 'CPF' THEN rg.descricao ELSE '' end as cpf,
        case when trg.simbolo = 'CNPJ' THEN rg.descricao ELSE '' end as cnpj
    from pessoa
    join tipo_pessoa tp on tp.id = pessoa.id_tipo_pessoa 
    join identificacao rg on rg.id = pessoa.id_identificacao 
    join status_pessoa sp on sp.id = pessoa.id_status 
    join tipo_identificacao trg on trg.id = rg.id_tipo_identificacao
    left join contato tel on tel.id = id_telefone  
    left join contato ema on ema.id = id_email
    left join empresa e on e.id = pessoa.id_empresa 
    where (pessoa.id_empresa = $1 or e.id_empresa_pessoas = $1)
    order by pessoa.nome  limit $2 offset $3",
    id_empresa,
    limit as i32,
    offset as i32,
    )
    .fetch_all(pool)
    .await?;

    Ok(rec)
}

pub async fn lista_grupos_pessoas (
    pool: &Pool<Postgres>,
        id_empresa: String,

) -> Result<Vec<GrupoPessoa>> {

    let rec =
    sqlx::query_as!(
        GrupoPessoa, r#"
        select g.*, 'all?categoria='  || lower( id ) as  url,
 	    (select count(*) from pessoa where id_grupo_pessoa = g.id) as qt
        from grupo_pessoa g 
        join grupo_pessoa_empresa e on g.id = e.id_grupo_pessoa 
        where e.id_empresa = $1 and id <> '0' and id <> 'INDEFINIDO' order by 4 desc "#,
        id_empresa)
        .fetch_all(pool).await;
   match rec {
    Ok(rec) => Ok(rec),
    Err(err) => Err(Sqlx(err))
   }
}