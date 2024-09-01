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