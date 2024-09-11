use crate::{admin::model::*, infra::uuid::UuidKind};
use log::info;
use sqlx::{Pool, Postgres};
use crate::infra::uuid::generate_uuid;
use crate::infra::result::Result;
use crate::entidade::identificacao::repo::upsert_identificacao;
use crate::infra::error::Error::Sqlx;

pub async fn inserir_empresa(
    pool: &Pool<Postgres>,
    id_usuario: String,
    empresa: &PostEmpresa,
) -> Result<Empresa> {
    let id = match empresa.id.clone() {
        Some(value) if value != "" => value,
        _ => generate_uuid(UuidKind::V7),
    };

    info!("Inserindo a empresa {e}", e = empresa.clone() );

    match &empresa.clone().cnpj {
        Some(cnpj) => {    
            let encontrou_cnpj = crate::entidade::identificacao::repo::abrir_identificacao(pool, &cnpj.clone()).await; 
            let tipo_cnpj = crate::entidade::identificacao::repo::abrir_tipo_identificacao(pool, &"CNPJ".to_owned()).await;
                    
            if let Some(valid_cnpj) = encontrou_cnpj {
                //Se o Cnpj ja esta no sistema, provavelmente a empresa foi cadastrada, exceto se tiver sido excluída
                //Se encontrar a empresa, retorna, senão insere
                if let Ok(empresa) = crate::admin::repo::abrir_empresa_one(pool, &cnpj).await {
                    info!("Cnpj {cnpj} já foi cadastrado. Retornando empresa {e}", cnpj = valid_cnpj.descricao.unwrap(), e = empresa.clone().id );
                    return Ok(empresa)
                }            
                else {
                    // let novo_cnpj = upsert_identificacao(pool, &cnpj.clone(), tipo_cnpj.unwrap().into()).await;
                    info!("Cnpj {cnpj} já está no sistema, mas a empresa ainda não foi cadastrada, ou foi excluída", cnpj = valid_cnpj.descricao.unwrap() );
              
                }
            
            }    
            else {
                let novo_cnpj = upsert_identificacao(pool, &cnpj.clone(), tipo_cnpj.unwrap().into()).await.unwrap();
                info!("Cnpj {cnpj} foi inserido. Id {id_cnpj}", cnpj = novo_cnpj.descricao.unwrap(), id_cnpj = novo_cnpj.id );
            }
        },
        None => {},
    };

    let rec = sqlx::query_as!(
        Empresa,
        "INSERT INTO empresa (id, id_cnpj, nome, fantasia, endereco, cidade, estado, telefone, email)
        VALUES (
        coalesce($1, '0'), --id
        (select id from identificacao where descricao = $2), --identificacao cnpj
        $3, --nome
        $3, --fantasia
        'Não informado', --endereco
        'Não informado', --cidade
        'Não informado', --estado
        $4,
        $5
        )
        RETURNING  id, nome, id_cnpj, $4 as cnpj",
        id.clone(),
        empresa.cnpj,
        empresa.nome,
        empresa.telefone,
        empresa.email,
    )
    .fetch_one(pool)
    .await?;

    //Cria os perfis de usuario da empresa atual
    let _ = sqlx::query!(
        "insert into perfil_usuario_empresa (id_empresa , id_perfil_usuario , nome)
    select $1, perfil_usuario.id, perfil_usuario.nome from perfil_usuario 
    where not exists (select id_empresa from perfil_usuario_empresa)", id)
    .execute(pool)
    .await?;

    //Insere os usuarios dev, super, admin para a empresa
    let _ = sqlx::query!(
        "insert into empresa_usuario (id_empresa , id_usuario)
    select $1, id from users where login in ('caze','','')", id)
    .execute(pool)
    .await?;

    //se o usuario ainda nao tiver uma empresa principal, vincula
    let _ = sqlx::query!(
        "update users set id_empresa = coalesce(
            (select id_empresa from users where id = $1), $2)
        where id = $1", id_usuario, id)
    .execute(pool)
    .await?;

    Ok(rec)
}

pub async fn abrir_empresa_one(
    pool: &Pool<Postgres>,
    id_empresa: &String,
) -> Result<Empresa> {
    
    let rec = sqlx::query_as!(
        Empresa,
        "select e.id, e.nome, e.id_cnpj, i.descricao as cnpj 
        from empresa e
        left join identificacao i on i.id = e.id_cnpj
        where e.id = $1 or i.descricao = $1",
        id_empresa,
    )
    .fetch_one(pool) //fetch_optional
    .await?;

    Ok(rec)
}

pub async fn listar_empresas_all(
    pool: &Pool<Postgres>,
) -> Result<Vec<Empresa>> {
    
    let rec: Vec<Empresa> = sqlx::query_as!(
        Empresa,
        "select e.id, 
            e.nome, 
            e.id_cnpj, 
            i.descricao as cnpj 
        from empresa e
        left join identificacao i on i.id = e.id_cnpj
        order by nome",
    )
    .fetch_all(pool)
    .await?;

    Ok(rec)
}

pub async fn atualizar_empresa(
    pool: &Pool<Postgres>,
    empresa: &PutEmpresa,
) -> Result<Empresa> {

    let found = abrir_empresa_one(pool, &empresa.id).await?;
    let found_cnpj = &found.id_cnpj.unwrap_or("0".to_owned());

    let rec = sqlx::query_as!(
        Empresa,
        "UPDATE empresa 
        set nome = $1,
        id_cnpj = $2
        where id = $3
        RETURNING id, nome, id_cnpj, $3 as cnpj",
        empresa.nome.as_ref().unwrap_or(&found.nome),
        empresa.cnpj.as_ref().unwrap_or(&found_cnpj),
        empresa.id,
    )
    .fetch_one(pool)
    .await?;

    Ok(rec)
}


pub async fn excluir_empresa(
    pool: &Pool<Postgres>,
    empresa_id: String,
) -> Result<()> {
    
    sqlx::query_as!(
        Empresa,
        "DELETE FROM empresa 
        where id = $1",
        empresa_id,
    )
    .execute(pool)
    .await?;

    Ok(())
}


pub async fn empresas_associadas(
    pool: &Pool<Postgres>,
    empresa_id: String,
        
) -> EmpresaAssociada {
    
    sqlx::query_as!(
        EmpresaAssociada,
        "select 
        id,
        coalesce(id_empresa_pessoas, id_empresa_cadastro, id) as id_empresa_pessoa, 
        coalesce(id_empresa_produtos, id_empresa_cadastro, id) as id_empresa_produto
        from empresa where id = $1",
        empresa_id,
    )
    .fetch_one(pool)
    .await.unwrap()

}

pub async fn abrir_dados_empresa_principal(
    pool: &Pool<Postgres>,  
    id_usuario: String,

) -> Option<DadosAccount> { 
  info!("looking for dados da empresa usuario = {id}", id = id_usuario.clone());  
  let result = sqlx::query_as!(
    DadosAccount, r#"
    select 
	u.id as "id_usuario!", 	
	e.id as "id_empresa!",
    u.nome as "nome_usuario!",
    u.nome as "nome_responsavel!",
    '' as "cpf_responsavel!",
    u.id_email as "email_usuario!",
    e.nome as "razao_social!",
    e.fantasia as nome_fantasia,
    cnpj.descricao as cnpj,
    coalesce(t_id.simbolo, 'cnpj') as "tipo_identificacao!",
    tel.descricao as "telefone!",
    '' as "segmento!",
    mail.descricao as "email!",
    end_p.id_rua as "endereco_principal!",
    end_p.id_bairro as "bairro_principal!",
    end_p.cep as "cep_principal!",
    cid.nome as "cidade_principal!",
    uf.nome as "estado_principal!",
    end_c.id_rua as endereco_cobranca,
    end_c.id_bairro as bairro_cobranca,
    end_c.cep as cep_cobranca,
    cid_c.nome as cidade_cobranca,
    uf_c.nome as estado_cobranca,
    end_e.id_rua as endereco_entrega,
    end_e.id_bairro as bairro_entrega,
    end_e.cep as cep_entrega,
    cid_e.nome as cidade_entrega,
    uf_e.nome as estado_entrega
from users u 
inner join empresa e on e.id = u.id_empresa 
inner join identificacao cnpj on cnpj.id = e.id_cnpj 
inner join tipo_identificacao t_id on t_id.id = cnpj.id_tipo_identificacao 
left join contato tel on tel.id = e.id_telefone 
left join contato mail on mail.id = e.id_email
left join endereco end_p on end_p.id = e.id_endereco_principal
left join endereco end_c on end_c.id = e.id_endereco_cobranca
left join endereco end_e on end_e.id = e.id_endereco_entrega
left join cidade cid on cid.id = end_p.id_cidade
left join cidade cid_c on cid_c.id = end_c.id_cidade
left join cidade cid_e on cid_e.id = end_e.id_cidade
left join estado uf on uf.id = end_p.id_estado
left join estado uf_c on uf.id = end_c.id_estado
left join estado uf_e on uf.id = end_e.id_estado
where u.id = $1 limit 1
    "#,
    id_usuario,
    )
    .fetch_one(pool).await;

  if let Ok(value) = result {
    info!("Pessoa localizado");
    Some(value)
  }

  else {
    info!("Pessoa não encontrado");
    None
  }

}

pub async fn lista_segmentos (
    pool: &Pool<Postgres>,

) -> Result<Vec<Segmento>> {

    let rec =
    sqlx::query_as!(
        Segmento, r#"
        select id, nome, classe
 	   from SEGMENTO_PESSOA 
       where id <> 'INDEFINIDO'
         order by NOME "#,)
        .fetch_all(pool).await;
   match rec {
    Ok(rec) => Ok(rec),
    Err(err) => Err(Sqlx(err))
   }
}