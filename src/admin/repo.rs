use crate::{admin::model::*, infra::uuid::UuidKind};
use sqlx::{Pool, Postgres};
use crate::infra::uuid::generate_uuid;
use crate::infra::result::Result;

pub async fn inserir_empresa(
    pool: &Pool<Postgres>,
    id_usuario: String,
    empresa: &PostEmpresa,
) -> Result<Empresa> {
    let id = match empresa.id.clone() {
        Some(value) if value != "" => value,
        _ => generate_uuid(UuidKind::V7),
    };

    let rec = sqlx::query_as!(
        Empresa,
        "INSERT INTO empresa (id, nome)
        VALUES (coalesce($1,'0'), $2)
        RETURNING  id, nome, id_cnpj",
        id.clone(),
        empresa.nome,
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
    empresa_id: &String,
) -> Result<Empresa> {
    
    let rec = sqlx::query_as!(
        Empresa,
        "select id, nome, id_cnpj from empresa where id = $1",
        empresa_id,
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
        "select id, nome, id_cnpj from empresa order by nome",
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
        RETURNING id, nome, id_cnpj",
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