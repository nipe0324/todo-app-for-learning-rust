use sqlx::{Row, SqlitePool};
use crate::{Todo, NewTodo};

pub async fn fetch_todos(pool: &crate::SqlitePool) -> Result<Vec<Todo>, sqlx::Error> {
    let rows = sqlx::query("SELECT id, title, description, completed FROM todos")
        .fetch_all(pool)
        .await?;

    let todos = rows
        .into_iter()
        .map(|row| Todo {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            completed: row.get("completed"),
        })
        .collect();

    Ok(todos)
}

pub async fn create_todo(pool: &SqlitePool, new_todo: &NewTodo) -> Result<Todo, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO todos (title, description, completed, created_at, updated_at)
        VALUES ($1, $2, false, datetime('now'), datetime('now'))
        RETURNING id, title, description, completed, created_at, updated_at;
        "#,
    )
    .bind(&new_todo.title)
    .bind(new_todo.description.as_deref())
    .fetch_one(pool)
    .await?;

    Ok(Todo {
        id: row.get("id"),
        title: row.get("title"),
        completed: row.get("completed"),
        description: row.get("description"),
    })
}
