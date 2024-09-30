use crate::auth::model::*;
use sqlx::PgPool;

pub async fn inserir_todo(
    pool: &PgPool,
    new_todo: &CreateTodo,
) -> Result<Todo, sqlx::Error> {
    let rec = sqlx::query_file_as!(
        Todo,
        "sql/create_todo.sql",
        new_todo.title,
        new_todo.description,
    )
    .fetch_one(pool)
    .await?;

    Ok(rec)
}

pub async fn get_all_todos(
    pool: &PgPool,
) -> Result<Vec<Todo>, sqlx::Error> {
    let todos = sqlx::query_file_as!(
        Todo,
        "sql/get_all_todos.sql"
    )
    .fetch_all(pool)
    .await?;

    Ok(todos)
}

pub async fn update_todo(
    pool: &PgPool,
    id: i32,
    updated_todo: &UpdateTodo,
) -> Result<Todo, sqlx::Error> {
    let rec = sqlx::query_file_as!(
        Todo,
        "sql/update_todo.sql",
        id,
        updated_todo.title,
        updated_todo.description,
        updated_todo.completed,
    )
    .fetch_one(pool)
    .await?;

    Ok(rec)
}

pub async fn delete_todo(
    pool: &PgPool,
    id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query_file_as!("sql/delete_todo.sql", id)
        .execute(pool)
        .await?;

    Ok(())
}

//////////////////////// Repository ////////////////////////////////////////

#[async_trait::async_trait]
impl Repository for Produto {
    async fn inserir<M>(model: M, data: &AppState) -> Result<Self>
    where
        M: Into<PostProduto>,
    {
        let post_produto: PostProduto = model.into();
        
        let produto = sqlx::query_as!(
            Produto,
            r#"
            INSERT INTO produto (nome, preco, unidade, quantidade)
            VALUES ($1, $2, $3, $4)
            RETURNING id, nome, preco, unidade, quantidade, created_at
            "#,
            post_produto.nome,
            post_produto.preco,
            post_produto.unidade,
            post_produto.quantidade
        )
        .fetch_one(&db.conn)
        .await?;

        Ok(produto)
    }

    async fn atualizar<M>(id: UuidValue, model: M, data: &AppState) -> Result<Self>
    where
        M: Into<PutProduto>,
    {
        let put_produto: PutProduto = model.into();

        let produto = sqlx::query_as!(
            Produto,
            r#"
            UPDATE produtos
            SET preco = $1, quantidade = $2
            WHERE id = $3
            RETURNING id, nome, preco, unidade, quantidade, created_at
            "#,
            put_produto.preco,
            put_produto.quantidade,
            id.value
        )
        .fetch_one(&data.database.connection)
        .await?;

        Ok(produto)
    }

    async fn excluir(id: UuidValue, data: &AppState) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM produtos
            WHERE id = $1
            "#,
            id .value
        )
        .execute(&data.database.connection)
        .await?;

        Ok(())
    }
}
#[async_trait::async_trait]
impl Provider for Produto {
    async fn abrir(id: UuidValue, data: &AppState) -> Result<Option<Self>> {
        let produto = sqlx::query_as!(
            Produto,
            r#"
            SELECT id, nome, preco, unidade, quantidade, created_at
            FROM produto
            WHERE id = $1
            "#,
            id.value
        )
        .fetch_optional(&data.database.conn)
        .await?;

        Ok(produto)
    }
}

#[async_trait::async_trait]
impl Filtro for Produto {

    async fn filtrar<F>(filtro: F , data: &AppState) -> Result<Vec<Self>, Error>
    where
        F: Into<Filtro>,
    {
        let filtro: Filtro = filtro.into();

        let produtos = sqlx::query_as!(
            Produto,
            r#"
            SELECT id, nome, preco, unidade, quantidade, created_at
            FROM produtos
            ORDER BY created_at DESC
            LIMIT 10 OFFSET $1
            "#,
            (filtro.page as i64 * 10)
        )
        .fetch_all(&data.database.connection)
        .await?;

        Ok(produtos)
    }
}