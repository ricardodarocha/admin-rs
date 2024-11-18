use sqlx::{Pool, Sqlite};
use crate::models::cliente::{FormCliente, Cliente};
use crate::infra::result::Result;

pub async fn atualizar_cliente(
    pool: &Pool<Sqlite>, 
        id: &String,
        form: FormCliente,
            
    ) -> Result<String> {
    let _ = sqlx::query_as!(
        Cliente,
        r#" update cliente set 
                 id = $1,
                 nome = $2 ,
                 cidade  = $3 
           where id = $1"#,
        id,
        form.nome,
        form.cidade,
    )
    .execute(pool)
    .await;

    Ok(id.to_string())
}

pub async fn inserir_cliente_form(
    pool: &Pool<Sqlite>, 
    form: FormCliente,

    ) -> Result<String> {

    let id = nanoid::nanoid!(12);
    let _ = sqlx::query_as!(
        Cliente,
        r#" insert into cliente
                 (id,
                 nome,
                 cidade) values
                 ($1,
                 $2,
                 $3)
                "#,
        id,
        form.nome,
        form.cidade
    )
    .execute(pool)
    .await;
    // .map_err(Into::into)

   Ok(id)

}

pub async fn abrir_cliente(pool: &Pool<Sqlite>, id: &String) -> Result<Cliente> {
    sqlx::query_as!(
        Cliente,
        r#" select
                 id,
                 nome,
                 cidade,
                 avatar
            from cliente
           where id = $1"#,
        id,
    )
    .fetch_one(pool)
    .await
    .map_err(Into::into)
}