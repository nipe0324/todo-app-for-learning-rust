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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt;
    use sqlx::{query, SqlitePool};


    async fn setup_test_db() -> SqlitePool {
        let db_url = "sqlite::memory:";
        let pool = SqlitePool::connect(db_url).await.unwrap();

        // Create the todos table
        query(
            r#"
            CREATE TABLE todos (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT NOT NULL,
                completed BOOLEAN NOT NULL
            );
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        pool
    }

    async fn insert_sample_data(pool: &SqlitePool) {
        query(
            r#"
            INSERT INTO todos (title, description, completed)
            VALUES
                ('Sample Todo 1', 'Sample description 1', 0),
                ('Sample Todo 2', 'Sample description 2', 1);
            "#,
        )
        .execute(pool)
        .await
        .unwrap();
    }

    #[actix_rt::test]
    async fn test_fetch_todos() {
        let pool = setup_test_db().await;
        insert_sample_data(&pool).await;

        let todos = fetch_todos(&pool).await.unwrap();
        assert_eq!(todos.len(), 2);

        assert_eq!(todos[0].id, 1);
        assert_eq!(todos[0].title, "Sample Todo 1");
        assert_eq!(todos[0].description, "Sample description 1");
        assert_eq!(todos[0].completed, false);

        assert_eq!(todos[1].id, 2);
        assert_eq!(todos[1].title, "Sample Todo 2");
        assert_eq!(todos[1].description, "Sample description 2");
        assert_eq!(todos[1].completed, true);
    }
}
