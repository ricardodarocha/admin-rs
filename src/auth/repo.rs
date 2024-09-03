use crate::admin::{model::*, service::*};
use crate::auth::model::*;
use crate::entidade::*;
use crate::infra::uuid::{generate_uuid, UuidKind};
// use actix_web::guard::Post;
// use actix_web::error::ErrorInternalServerError;
use sqlx::{Pool, Postgres};
use crate::infra::result::Result;
use crate::entidade::identificacao::repo::*;
use crate::entidade::contato::repo::*;

// pub async fn cnpj(
//     pool: &Pool<Postgres>,
//     cnpj: &String,
    
//     ) -> Result<Option<EntidadeId>> {

//     sqlx::query_as!(EntidadeId, "select id from tenant where cnpj = $1",
//     cnpj)
//     .fetch_optional(pool).await
// }



// pub async fn tenant(
//     pool: &Pool<Postgres>, 
//     id: &String) -> Result<Option<Tenant>> {
//     sqlx::query_as!(EntidadeId, "select * from tenant where id = $1", 
//     id)
//     .fetch_optional(pool).await  
// }

// pub async fn incluir_tenant(
//     pool: &Pool<Postgres>,
//     form: &RegisterData,
// ) -> Result<Tenant> {
//     let id = generate_uuid(UuidKind::V7);

    

  
//     let rec = Tenant::default();
//     Ok(rec)
// }

// encontra um subscriber a partir do email
pub async fn abrir_subscriber (
    pool: &Pool<Postgres>,
    subscriber: &SubscribeForm,
) -> Option<Subscriber> {
    sqlx::query_as!(
        Subscriber, r#"
        select 
            subscriber.*
            from subscriber left join contato c on c.id = id_email
        where id_email = $1 or nome = $1"#,
        subscriber.email)
        .fetch_optional(pool).await.unwrap()
}

pub async fn inserir_subscriber (
    pool: &Pool<Postgres>,
    form: SubscribeForm,
) -> Subscriber {
        let id = generate_uuid(UuidKind::V7);
        let tipo_email = upsert_tipo_contato(pool, &"EMAIL".to_owned()).await.unwrap();
        let email = upsert_contato(pool, &form.email, EntidadeId{id: tipo_email.id.clone()}).await.unwrap();

        let _entidade = sqlx::query_as!(
        EntidadeId, "insert into subscriber (id, nome, id_email) values($1, $2, $3) RETURNING id",
        id,
        tipo_email.id,
        email.id)
        .fetch_one(pool).await.unwrap();

        abrir_subscriber(pool, &form).await.unwrap()
        // match tipo_email {
        //     Err(err) => Err(ErrorInternalServerError(err))
        // }
}

pub async fn upsert_subscriber (
    pool: &Pool<Postgres>,
    subscriber: SubscribeForm,
) -> Result<Subscriber> {
    
    let entidade = abrir_subscriber(pool, &subscriber).await;

    if let Some(entidade) = entidade {
        Ok(entidade)
    } else
    {   
        let novo_subscriber = inserir_subscriber(pool, subscriber).await;
        Ok(novo_subscriber)
    }
}

pub async fn incluir_enviar_email_primeiro_acesso(
    pool: &Pool<Postgres>,
    email: String, 
    body: String,
) {
      let id = generate_uuid(UuidKind::V7);
      let tipo_email = upsert_tipo_contato(pool, &"EMAIL".to_owned()).await.unwrap();
      let email = upsert_contato(pool, &email, EntidadeId{id: tipo_email.id.clone()}).await.unwrap();

     let _ = sqlx::query!(
        "INSERT INTO  enviar_email (id, id_email, body)
        VALUES ($1, $2, $3) ",
    id,
    email.id, 
    body)
    .execute(pool)
    .await
    ;

}

pub async fn incluir_instituicao(
    pool: &Pool<Postgres>,
    id_usuario: String,
    instituicao: &String,
) -> Result<Empresa> {
    inserir_empresa(pool, 
        id_usuario,
        &PostEmpresa{
        id: None,
        nome: instituicao.clone(),
        cnpj: None,
        email: None,
        telefone: None,
    }).await
}

pub async fn incluir_permissoes(
    pool: &Pool<Postgres>,
    usuario: &User,
    // empresa: &Empresa,
) -> Result<()> {

    let dev = "d47e184c-8118-554e-a11c-97c308ad7669".to_owned();
    let _ = sqlx::query!(r#"INSERT INTO usuario_perfil (id_usuario, id_perfil_usuario, id_usuario_admin)
    values ($1, (select id from perfil_usuario where nome = 'USER'), $2)
    "#,
    &usuario.id, 
    &dev,
    ).execute(pool).await;
    
    Ok(())
}

pub async fn vincular_empresa_usuario(
    pool: &Pool<Postgres>,
    usuario: &User, 
    empresa: &Empresa

) -> () {

    let _ = sqlx::query!("delete from empresa_usuario where id_empresa = $1 and id_usuario = $2",
    &usuario.id, 
    &empresa.id,
    ).execute(pool).await;

    let _ = sqlx::query!("INSERT INTO  empresa_usuario (id_empresa, id_usuario) values ($1, $2)",
    &usuario.id, 
    &empresa.id,
    ).execute(pool).await;
    
}

pub async fn incluir_user(
    pool: &Pool<Postgres>,
    form: &RegisterUser,
    password_hash: String,
) -> Result<User> {
    let id = generate_uuid(UuidKind::V7);
    let tipo_email = upsert_tipo_contato(pool, &"EMAIL".to_owned()).await.unwrap();
    let _contato_email = upsert_contato(pool, &form.email, EntidadeId{id: tipo_email.id.clone()}).await;

    let _novo_usuario = sqlx::query_as!(
        EntidadeId,
        "INSERT INTO  users (id, login, nome, id_email, password)
        VALUES ($1, $2, $3, (select id from contato where descricao = $4), $5 ) returning id",
    id,
    form.email,
    form.nome,
    form.email,
    password_hash
)
    .fetch_one(pool).await.unwrap();

    let rec = abrir_usuario(pool, &form.email, &password_hash ).await.unwrap();
    Ok(rec)
}

pub async fn inserir_log_acesso(
    login: &String,
    status: &String,
    origem: &String,
    id_usuario: &String,
) {
    let _ = sqlx::query!(
        "INSERT INTO  log_acesso (login, status, origem, id_usuario)
        VALUES ($1, $2, $3, $4)",
    login,
    status,
    origem,
    id_usuario,

);
}
pub async fn inserir_consumo_rota(
    rota: &String,
    status: &String,
    origem: &String,
    id_usuario: &String,
) {
    let _ = sqlx::query!(
        "INSERT INTO  log_acesso (login, status, origem, id_usuario)
        VALUES ($1, $2, $3, $4)",
    rota,
    status,
    origem,
    id_usuario,

);
}

pub async fn reset_password(
    _pool: &Pool<Postgres>,
    _user: &RegisterData,
) -> Result<()> {
    let _id = generate_uuid(UuidKind::V7);

    // let login = sqlx::query_as!(
    //     UserId,
    //     "SELECT id FROM users WHERE name = $1",
    //     user.id,
    //     user.password,
    //     user.email,
    // )

    // let rec = sqlx::query_as!(
    //     UserId,
    //     "INSERT INTO  users (id, name, password, email)
    //     VALUES ($1, $2, $3, $4)
    //     RETURNING  id",
    //     id,
    //     user.username,
    //     user.password,
    //     user.email,
    // )
    // .fetch_one(pool)
    // .await?;

    Ok(())
}

pub async fn abrir_usuario(
    pool: &Pool<Postgres>,
    username: &String, 
    password: &String

) -> Option<User> {

    let result = sqlx::query_as!(
        User,
        r#"SELECT u.id, u.nome, u.id_empresa, c.descricao as email, 
        'https://www.pngall.com/wp-content/uploads/5/User-Profile-PNG-Download-Image.png' as photo
        from users u 
        left join contato c on c.id = u.id_email
        WHERE (login = $1 or c.descricao = $1 or u.id = $1) and password = $2"#,
        username, 
        password,
    )
    .fetch_optional(pool)
    .await;

    result.expect("Usuário não encontrado")
}

pub async fn recados_do_usuario(
    pool: &Pool<Postgres>,
    id_usuario: &String, 

) -> Result<Vec<Recado>>  {

    let result = sqlx::query_as!(
        Recado,
        r#"
    select * from recado where id_usuario = $1 and 
    (readed_at is null or readed_at <  CURRENT_DATE - 1)"#,
        id_usuario.clone(),
    )
    .fetch_all(pool)
    .await;

    //Marca como lido
    let _ = sqlx::query!("
    update recado set readed_at = CURRENT_TIMESTAMP 
    where id_usuario = $1
    and readed_at is null
    ", id_usuario)
    .execute(pool).await;

    Ok(result?)
}


pub async fn abrir_usuario_from_id(
    pool: &Pool<Postgres>,
    id: &String, 

) -> Option<User> {

    sqlx::query_as!(
        User,
        r#"SELECT u.id, u.nome, u.id_empresa, c.descricao as email,
        'https://www.pngall.com/wp-content/uploads/5/User-Profile-PNG-Download-Image.png' as photo 
        from users u 
        left join contato c on c.id = u.id_email
        WHERE u.id = $1 "#,
        id,
    )
    .fetch_optional(pool)
    .await
    .unwrap()
}

pub async fn abrir_usuario_from_email(
    pool: &Pool<Postgres>,
    email: &String, 

) -> Option<User> {

    sqlx::query_as!(
        User,
        r#"SELECT u.id, u.nome, u.id_empresa, c.descricao as email,
        'https://www.pngall.com/wp-content/uploads/5/User-Profile-PNG-Download-Image.png' as photo 
        from users u 
        left join contato c on c.id = u.id_email
        WHERE c.descricao = $1 "#,
        email,
    )
    .fetch_optional(pool)
    .await
    .unwrap()
}

pub async fn atribuir_perfil_usuario(pool: &Pool<Postgres>, novo_usuario: &User, perfil: &str, admin: &str) -> () {
    
    let _ = sqlx::query!("INSERT INTO  usuario_perfil (
        id_usuario, 
        id_perfil_usuario, 
        id_usuario_admin) 
    values (
        $1, 
        (select id_perfil_usuario from perfil_usuario_empresa where nome = $2), 
        $3)",
    novo_usuario.id, 
    perfil,
    admin,
    ).execute(pool).await;
}

pub async fn inserir_empresa_primeiro_acesso(pool: &Pool<Postgres>,
    id_usuario: String, 
    empresa: PrimeiroAcesso,

) -> () {

    //informações sensíveis
    let tipo_cnpj = crate::entidade::identificacao::repo::abrir_tipo_identificacao(pool, &"CNPJ".to_owned()).await;
    let tipo_cpf = crate::entidade::identificacao::repo::abrir_tipo_identificacao(pool, &"CPF".to_owned()).await;
    let tipo_email = crate::entidade::contato::repo::abrir_tipo_contato(pool, &"EMAIL".to_owned()).await;
    let tipo_telefone = crate::entidade::contato::repo::abrir_tipo_contato(pool, &"TELEFONE".to_owned()).await;
    
    let encontrou_cnpj = crate::entidade::identificacao::repo::abrir_identificacao(pool, &empresa.cnpj).await; 
    let encontrou_cpf = crate::entidade::identificacao::repo::abrir_identificacao(pool, &empresa.cpf).await;     
    let encontrou_email = crate::entidade::contato::repo::abrir_contato(pool, &empresa.email).await;   
    let encontrou_telefone = crate::entidade::contato::repo::abrir_contato(pool, &empresa.telefone).await; 

    if let Some(_cnpj) = encontrou_cnpj {
        return (); //Empresa já foi cadastrada
    } else {
        _ = upsert_identificacao(pool, &empresa.cnpj, tipo_cnpj.unwrap().into()).await;
    }

    if let None = encontrou_cpf {
        _ = upsert_identificacao(pool, &empresa.cpf, tipo_cpf.unwrap().into()).await;
    }


    if let None = encontrou_email {
        _ = upsert_contato(pool, &empresa.email, tipo_email.unwrap().into()).await;
    }

    if let None = encontrou_telefone {
        _ = upsert_contato(pool, &empresa.telefone, tipo_telefone.unwrap().into()).await;
    }
    

    let _ = inserir_empresa(pool, 
        id_usuario,
        &PostEmpresa{
        id: None,
        nome: empresa.nome,
        cnpj: Some(empresa.cnpj),
        email: Some(empresa.email),
        telefone: Some(empresa.telefone),
    }).await;

}

// pub async fn permissoes(pool: &Pool<Postgres>, id_usuario: &String) -> Permissoes {
//     let result = sqlx::query_as!(
//         Permissoes,
// r#"with level  as (
// 	select up.id_perfil_usuario, p.nome 
// 	from users 
// 	join usuario_perfil up on up.id_usuario  = users.id 
// 	join perfil_usuario p on p.id  = up.id_perfil_usuario 
// 	where users.id = $1
// 	order by up.created desc
// 	limit 1
// )
// select 
// 	level.id_perfil_usuario, 
// 	level.nome,
// 	STRING_AGG (permissao.operation || ' ' || permissao.permissao, ','
//        ORDER BY
//         permissao.operation,
//         permissao.permissao
//     ) permissoes       
// from users, level 
// join permissao on permissao.id_perfil = level.id_perfil_usuario 
// where users.id = $1
// group by 1, 2"#,
//     id_usuario,
//     ).fetch_one(pool).await;

//   result.unwrap()
// }