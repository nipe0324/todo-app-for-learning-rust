use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use sqlx::{Row, SqlitePool};

#[derive(Serialize)]
struct Todo {
    id: i32,
    title: String,
    completed: bool
}

#[derive(Deserialize)]
pub struct NewTodo {
    title: String,
    description: Option<String>,
}

#[get("/todos")]
async fn get_todos(pool: web::Data<SqlitePool>) -> impl Responder {
    let result = sqlx::query("SELECT id, title, completed FROM todos")
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(rows) => {
            let todos: Vec<Todo> = rows
                .into_iter()
                .map(|row| Todo {
                    id: row.get("id"),
                    title: row.get("title"),
                    completed: row.get("completed"),
                })
                .collect();

            HttpResponse::Ok().json(todos)
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/todos")]
async fn post_todos(
    pool: web::Data<SqlitePool>,
    new_todo: web::Json<NewTodo>,
) -> impl Responder {
    let result = sqlx::query(
        r#"
        INSERT INTO todos (title, description, completed, created_at, updated_at)
        VALUES ($1, $2, false, datetime('now'), datetime('now'))
        RETURNING id, title, description, completed, created_at, updated_at;
        "#,
    )
    .bind(&new_todo.title)
    .bind(new_todo.description.as_deref())
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(row) => {
            let todo = Todo {
                id: row.get("id"),
                title: row.get("title"),
                completed: row.get("completed"),
            };

            HttpResponse::Created().json(todo)
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    // connect to SQLite DB
    let database_url = "todo.db";
    let pool = SqlitePool::connect(&database_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(get_todos)
            .service(post_todos)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
