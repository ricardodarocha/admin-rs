use crate::entidade::contato::repo::{ upsert_contato, upsert_tipo_contato};
use crate::entidade::identificacao::repo::upsert_identificacao;
use crate::entidade::EntidadeId;
use crate::infra::error::Error::{self, Sqlx};
use crate::infra::result::Result;
use crate::infra::uuid::generate_uuid;
use crate::pessoa::endereco::model::BuscaEndereco;
use crate::pessoa::endereco::service::upsert_endereco;
use crate::pessoa::model::PostPessoa;
use crate::{admin::model::*, infra::uuid::UuidKind};
use log::{error, info};
use sqlx::{Pool, Postgres};
use utoipa::openapi::info;

pub async fn inserir_account(
    pool: &Pool<Postgres>,
    id_usuario: String,
    empresa: &PostAccount,
) -> Result<Empresa> {
    let id = empresa.id_empresa.clone();
    let id = if id != "0" {
        id
    } else {
        generate_uuid(UuidKind::V7)
    };

    info!("Inserindo a empresa {e}", e = empresa.clone());

    let cnpj = &empresa.clone().cnpj;
    let encontrou_cnpj =
        crate::entidade::identificacao::repo::abrir_identificacao(pool, &cnpj.clone()).await;
    let _tipo_cnpj =
        crate::entidade::identificacao::repo::abrir_tipo_identificacao(pool, &"CNPJ".to_owned())
            .await;

    if let Some(valid_cnpj) = encontrou_cnpj {
        //Se o Cnpj ja esta no sistema, provavelmente a empresa foi cadastrada, exceto se tiver sido excluída
        //Se encontrar a empresa, retorna, senão insere
        if let Ok(empresa) = crate::admin::repo::abrir_empresa_one(pool, &Some(cnpj.clone())).await
        {
            if let Some(empresa) = empresa {
                info!(
                    "Cnpj {cnpj} já foi cadastrado. Retornando empresa {e}",
                    cnpj = valid_cnpj.descricao.unwrap(),
                    e = empresa.clone().id
                );
                return Ok(empresa);
            } else {
                // let novo_cnpj = upsert_identificacao(pool, &cnpj.clone(), tipo_cnpj.unwrap().into()).await;
                info!("Cnpj {cnpj} já está no sistema, mas a empresa ainda não foi cadastrada, ou foi excluída", cnpj = valid_cnpj.descricao.unwrap() );
            }
        } else {
            // let novo_cnpj = upsert_identificacao(pool, &cnpj.clone(), tipo_cnpj.unwrap().into()).await;
            info!("Cnpj {cnpj} já está no sistema, mas a empresa ainda não foi cadastrada, ou foi excluída", cnpj = valid_cnpj.descricao.unwrap() );
        }
    };

    // }
    // else {
    //     let novo_cnpj = upsert_identificacao(pool, &cnpj.clone(), tipo_cnpj.unwrap().into()).await.unwrap();
    //     info!("Cnpj {cnpj} foi inserido. Id {id_cnpj}", cnpj = novo_cnpj.descricao.unwrap(), id_cnpj = novo_cnpj.id );

    let tipo_email = upsert_tipo_contato(pool, &"EMAIL".to_owned())
        .await
        .unwrap();
    let id_email = upsert_contato(pool, &empresa.email.clone(), EntidadeId::from(tipo_email))
        .await
        .unwrap();

    let rec = sqlx::query_as!(
        Empresa,
        "INSERT INTO empresa (id, id_cnpj, nome, fantasia, endereco, cidade, estado, telefone, email, id_email)
        VALUES (
        coalesce($1, '0'), --id
        (select id from identificacao where descricao = $2), --identificacao cnpj
        $3, --nome
        $3, --fantasia
        'Não informado', --endereco
        'Não informado', --cidade
        'Não informado', --estado
        $4,
        $5,
        $6
        )
        RETURNING  id, nome, fantasia, endereco, cidade, estado, telefone, email, id_cnpj, $4 as cnpj",
        id.clone(),
        empresa.cnpj,
        empresa.nome_fantasia,
        empresa.telefone,
        empresa.email,
        id_email.id,
    )
    .fetch_one(pool)
    .await?;

    //Cria os perfis de usuario da empresa atual
    let _ = sqlx::query!(
        "insert into perfil_usuario_empresa (id_empresa , id_perfil_usuario , nome)
    select $1, perfil_usuario.id, perfil_usuario.nome from perfil_usuario 
    where not exists (select id_empresa from perfil_usuario_empresa)",
        id
    )
    .execute(pool)
    .await?;

    //Insere os usuarios dev, super, admin para a empresa
    let _ = sqlx::query!(
        "insert into empresa_usuario (id_empresa , id_usuario)
    select $1, id from users where login in ('caze','','')",
        id
    )
    .execute(pool)
    .await?;

    //se o usuario ainda nao tiver uma empresa principal, vincula
    let _ = sqlx::query!(
        "update users set id_empresa = coalesce(
            (select id_empresa from users where id = $1), $2)
        where id = $1",
        id_usuario,
        id
    )
    .execute(pool)
    .await?;

    Ok(rec)
}

pub async fn inserir_empresa(
    pool: &Pool<Postgres>,
    id_usuario: String,
    empresa: &PostEmpresa,
) -> Result<Empresa> {
    let id = match empresa.id.clone() {
        Some(value) if value != "" => value,
        _ => generate_uuid(UuidKind::V7),
    };

    info!("Inserindo a empresa {e}", e = empresa);

    match &empresa.cnpj {
        Some(cnpj) => {
            let encontrou_cnpj =
                crate::entidade::identificacao::repo::abrir_identificacao(pool, &cnpj.clone())
                    .await;
            let tipo_cnpj = crate::entidade::identificacao::repo::abrir_tipo_identificacao(
                pool,
                &"CNPJ".to_owned(),
            )
            .await;

            if let Some(valid_cnpj) = encontrou_cnpj {
                //Se o Cnpj ja esta no sistema, provavelmente a empresa foi cadastrada, exceto se tiver sido excid_empresaluída
                //Se encontrar a empresa, retorna, senão insere
                if let Ok(empresa) =
                    crate::admin::repo::abrir_empresa_one(pool, &Some(cnpj.clone())).await
                {
                    if let Some(empresa) = empresa {
                        info!(
                            "Cnpj {cnpj} já foi cadastrado. Retornando empresa {e}",
                            cnpj = valid_cnpj.descricao.unwrap(),
                            e = empresa.clone().id
                        );
                        return Ok(empresa);
                    } else {
                        // let novo_cnpj = upsert_identificacao(pool, &cnpj.clone(), tipo_cnpj.unwrap().into()).await;
                        info!("Cnpj {cnpj} já está no sistema, mas a empresa ainda não foi cadastrada, ou foi excluída", cnpj = valid_cnpj.descricao.unwrap() );
                    }
                } else {
                    // let novo_cnpj = upsert_identificacao(pool, &cnpj.clone(), tipo_cnpj.unwrap().into()).await;
                    info!("Cnpj {cnpj} já está no sistema, mas a empresa ainda não foi cadastrada, ou foi excluída", cnpj = valid_cnpj.descricao.unwrap() );
                }
            } else {
                let novo_cnpj =
                    upsert_identificacao(pool, &cnpj.clone(), tipo_cnpj.unwrap().into())
                        .await
                        .unwrap();
                info!(
                    "Cnpj {cnpj} foi inserido. Id {id_cnpj}",
                    cnpj = novo_cnpj.descricao.unwrap(),
                    id_cnpj = novo_cnpj.id
                );
            }
        }
        None => {}
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
        RETURNING  id, nome, fantasia, endereco, cidade, estado, telefone, email, id_cnpj, $4 as cnpj",
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
    where not exists (select id_empresa from perfil_usuario_empresa)",
        id
    )
    .execute(pool)
    .await?;

    //Insere os usuarios dev, super, admin para a empresa
    let _ = sqlx::query!(
        "insert into empresa_usuario (id_empresa , id_usuario)
    select $1, id from users where login in ('caze','','')",
        id
    )
    .execute(pool)
    .await?;

    //se o usuario ainda nao tiver uma empresa principal, vincula
    let _ = sqlx::query!(
        "update users set id_empresa = coalesce(
            (select id_empresa from users where id = $1), $2)
        where id = $1",
        id_usuario,
        id
    )
    .execute(pool)
    .await?;

    Ok(rec)
}

pub async fn abrir_empresa_one(
    pool: &Pool<Postgres>,
    id_empresa: &Option<String>,
) -> Result<Option<Empresa>> {
    match id_empresa {
        Some(id_empresa) => {
            let rec = sqlx::query_as!(
                Empresa,
                "select e.id, e.nome, e.id_cnpj, e.fantasia, e.endereco, e.cidade, e.estado, telefone, email, i.descricao as cnpj 
                from empresa e
                join identificacao i on i.id = e.id_cnpj
                where e.id = $1 or i.descricao = $1",
                id_empresa,
            )
            .fetch_optional(pool)
            .await?;

            Ok(rec)
        }
        None => Ok(None),
    }
}

pub async fn listar_empresas_all(pool: &Pool<Postgres>) -> Result<Vec<Empresa>> {

    let rec: Vec<Empresa> = sqlx::query_as!(
        Empresa,
        "select e.id, 
            e.fantasia, 
            e.nome, 
            e.id_cnpj,
            e.endereco, 
            e.cidade, 
            e.estado, 
            e.telefone, 
            e.email, 
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
    id_empresa: &String,    
    ) -> Result<Empresa> {
    let found = abrir_empresa_one(pool, &Some(empresa.id.clone())).await?;
    
    if let Some(found) = found {
        let _found_cnpj = &found.id_cnpj.unwrap_or("0".to_owned());
        info!("Atualizando a empresa {}", &id_empresa);
        let mut tr = pool.begin().await?;

        //nome
        if empresa.nome.is_some() {
            let _ = sqlx::query!(
                "UPDATE empresa 
                        set nome = $1
                        where id = $2",
                empresa.nome.as_ref().unwrap_or(&found.nome),
                empresa.id,
            )
            .fetch_one(&mut *tr)
            .await;
        
        info!("..nome atualizado");
        };

        if empresa.fantasia.is_some() {
            let _ = sqlx::query!(
                "update empresa set fantasia = $1 where id = $2",
                &empresa.fantasia.as_ref().unwrap(),
                &empresa.id,
            )
            .execute(&mut *tr)
            .await;
        
        info!("..nome fantasia atualizado");
        };
       
        if empresa.cpf.is_some() {
            
            let pessoa_responsavel =  crate::pessoa::repo::abrir_pessoa_fisica(
                &pool.clone(), 
                id_empresa.clone(),
                &empresa.cpf.as_ref().unwrap()
            ).await;
            let id_responsavel = if let Some(pessoa_responsavel) = pessoa_responsavel {
                pessoa_responsavel.id    
            } else {
                let pessoa = PostPessoa {
                    // id: None,
                    razao_social: empresa.nome.as_ref().unwrap_or(&"".to_string()).clone(),
                    nome: empresa.nome_responsavel.clone(),
                    // tipo_pessoa: None,
                    cpf: empresa.cpf.clone(),
                    // cnpj: None,
                    email: empresa.email.as_ref().unwrap_or(&"".to_owned()).clone(),
                    telefone: empresa.telefone.as_ref().unwrap_or(&"".to_owned()).clone(),
                    endereco: empresa.endereco_principal.as_ref().unwrap_or(&"".to_owned()).clone(),
                    bairro: empresa.endereco_principal.as_ref().unwrap_or(&"".to_owned()).clone(),
                    cidade: empresa.endereco_principal.as_ref().unwrap_or(&"".to_owned()).clone(),
                    id_estado: empresa.endereco_principal.as_ref().unwrap_or(&"".to_owned()).clone(),
                    cep: empresa.endereco_principal.as_ref().unwrap_or(&"".to_owned()).clone(),
                    ..Default::default()
                };
                
        
                info!("..cpf não encontrado, deverá inserir pessoa");
                let pessoa_inserida = crate::pessoa::repo::inserir_pessoa(&pool, &pessoa).await;
                
                if pessoa_inserida.is_ok() {
                    pessoa_inserida.unwrap().id } else {"INDEFINIDO".to_owned()}
            };

            let _ = sqlx::query!(
                "update empresa set id_responsavel = $1 where id = $2",
                &id_responsavel,
                &empresa.id,
            )
            .execute(&mut *tr)
            .await;
        };
        
        if empresa.email.is_some() {
            let tipo_email = upsert_tipo_contato(pool, &"EMAIL".to_owned()).await.unwrap();
            let id_email = upsert_contato(&pool.clone(), &empresa.email.as_ref().unwrap(), tipo_email.into()).await.unwrap();

                let _ = sqlx::query!(
                    "update empresa set id_email = $1 where id = $2",
                    &id_email.id,
                    &empresa.id,
                )
                .execute(&mut *tr)
                .await;            
        };
        
        if empresa.telefone.is_some() {
            let tipo_telefone = upsert_tipo_contato(pool, &"TELEFONE".to_owned()).await.unwrap();
            let id_telefone = upsert_contato(&pool.clone(), &empresa.telefone.as_ref().unwrap(), tipo_telefone.into()).await;

            if let Ok(id_telefone) = id_telefone {    
                let _ = sqlx::query!(
                    "update empresa set telefone = $1 where id = $2",
                    &id_telefone.id,
                    &empresa.id,
                )
                .execute(&mut *tr)
                .await;
            };
        };
        
        if empresa.segmento.is_some() {
            let _ = sqlx::query!(
                "update empresa set segmento = $1 where id = $2",
                &empresa.segmento.as_ref().unwrap(),
                &empresa.id,
            )
            .execute(&mut *tr)
            .await;
        };
        
        if empresa.endereco_principal.is_some() && empresa.bairro_principal.is_some() {
            
            let form_endereco = BuscaEndereco{
                endereco: empresa.endereco_principal.clone(),
                bairro: empresa.bairro_principal.clone(),
                cidade: empresa.cidade_principal.clone(),
                estado: empresa.estado_principal.clone(),
                cep: empresa.cep_principal.clone(),
            };

            
            info!("Atualizando endereço {:?}", &form_endereco.clone());
            let endereco = upsert_endereco(&pool.clone(), form_endereco).await;
            match endereco {
                Ok(endereco) => {let id_endereco = endereco.id;

                let _ = sqlx::query!(
                "update empresa set
                    id_endereco_principal = $1
                 where id = $2",
                &id_endereco,
                &empresa.id,
            )
            .execute(&mut *tr)
            .await;
            },
                Err(err ) => {error!("Erro ao incluir endereço: {:?}", err)},
        }
    };
        
        if empresa.endereco_cobranca.is_some() && empresa.bairro_cobranca.is_some()  {      
            let endereco = BuscaEndereco{
                endereco: empresa.endereco_cobranca.clone(),
                bairro: empresa.bairro_cobranca.clone(),
                cidade: empresa.cidade_cobranca.clone(),
                estado: empresa.estado_cobranca.clone(),
                cep: empresa.cep_cobranca.clone(),
            };

            let endereco = upsert_endereco(&pool.clone(), endereco).await;
            match endereco {
                Ok(endereco) => {
                let id_endereco = endereco.id;

                let _ = sqlx::query!(
                "update empresa set
                    id_endereco_cobranca = $1
                 where id = $2",
                &id_endereco,
                &empresa.id,
            )
            .execute(&mut *tr)
            .await;},
                Err(err ) => {error!("Erro ao incluir endereço: {:?}", err)},
            }

        };
        
        if empresa.endereco_entrega.is_some() && empresa.bairro_entrega.is_some() {
            
            let endereco = BuscaEndereco{
                endereco: empresa.endereco_entrega.clone(),
                bairro: empresa.bairro_entrega.clone(),
                cidade: empresa.cidade_entrega.clone(),
                estado: empresa.estado_entrega.clone(),
                cep: empresa.cep_entrega.clone(),
            };

            let endereco = 
            crate::pessoa::endereco::service::upsert_endereco(&pool.clone(), endereco).await;
           
           if endereco.is_ok() {
                let id_endereco = endereco.unwrap().id;

                let _ = sqlx::query!(
                "update empresa set
                    id_endereco_entrega = $1
                 where id = $2",
                &id_endereco,
                &empresa.id,
            )
            .execute(&mut *tr)
            .await;

           }
                
        
        };        
        
        let rec = abrir_empresa_one(pool, &Some(empresa.id.clone())).await?.unwrap();
        Ok(rec)    
    } 
    else {
        Err(Error::Str(&"Empresa não localizada"))
    }
}

pub async fn excluir_empresa(pool: &Pool<Postgres>, empresa_id: String) -> Result<()> {
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

pub async fn empresas_associadas(pool: &Pool<Postgres>, empresa_id: String) -> EmpresaAssociada {
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
    .await
    .unwrap()
}

pub async fn abrir_dados_empresa_principal(
    pool: &Pool<Postgres>,
    id_usuario: String,
) -> Option<DadosAccount> {
    info!(
        "looking for dados da empresa usuario = {id}",
        id = id_usuario.clone()
    );
    let result = sqlx::query_as!(
        DadosAccount,
        r#"
    select 
        u.id as "id_usuario!", 	
        e.id as "id_empresa!",
        u.nome as nome_usuario,
        u.nome as nome_responsavel,
        null as cpf_responsavel,
        u.id_email as email_usuario,
        e.nome as razao_social,
        e.fantasia as nome_fantasia,
        cnpj.descricao as cnpj,
        coalesce(t_id.simbolo, 'cnpj') as tipo_identificacao,
        coalesce(tel.descricao, '') as telefone,
        coalesce(e.segmento, 'INDEFINIDO') as segmento,
        coalesce(mail.descricao, '') as email,
        coalesce(end_p.id_rua, null) as endereco_principal,
        coalesce(end_p.id_bairro, null) as bairro_principal,
        coalesce(end_p.cep, '') as cep_principal,
        coalesce(cid.nome, '') as cidade_principal,
        coalesce(uf.nome, 'INDEFINIDO') as estado_principal,
        end_c.id_rua as "endereco_cobranca?",
        end_c.id_bairro as "bairro_cobranca?",
        end_c.cep as "cep_cobranca?",
        cid_c.nome as "cidade_cobranca?",
        uf_c.nome as "estado_cobranca?",
        end_e.id_rua as "endereco_entrega?",
        end_e.id_bairro as "bairro_entrega?",
        end_e.cep as "cep_entrega?",
        cid_e.nome as "cidade_entrega?",
        uf_e.nome as "estado_entrega?"
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
    left join estado uf_c on uf_c.id = end_c.id_estado
    left join estado uf_e on uf_e.id = end_e.id_estado
    where u.id = $1
    limit 1
"#,
        id_usuario,
    )
    .fetch_one(pool)
    .await;

    match result {
        Ok(value) => {
        info!("Empresa localizada");
        Some(value)
    },
    Err(err) => {
        info!("Empresa não encontrada");
        error!("{}", err);
        None
    }}
    // Some(result.unwrap())
}
    
pub async fn lista_segmentos(pool: &Pool<Postgres>) -> Result<Vec<Segmento>> {
    let rec = sqlx::query_as!(
        Segmento,
        r#"
        select id, nome, classe
    from SEGMENTO_PESSOA 
    where id <> 'INDEFINIDO'
        order by NOME "#,
    )
    .fetch_all(pool)
    .await;

    match rec {
        Ok(rec) => Ok(rec),
        Err(err) => Err(Sqlx(err)),
    }
}