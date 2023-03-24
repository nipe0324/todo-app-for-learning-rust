use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use sqlx::{Row, SqlitePool};

#[derive(Serialize)]
struct Todo {
    id: i32,
    title: String,
    description: String,
    completed: bool
}

#[derive(Deserialize)]
pub struct NewTodo {
    title: String,
    description: Option<String>,
}

async fn fetch_todos(pool: &SqlitePool) -> Result<Vec<Todo>, sqlx::Error> {
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

async fn create_todo(pool: &SqlitePool, new_todo: &NewTodo) -> Result<Todo, sqlx::Error> {
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

#[get("/todos")]
async fn get_todos(pool: web::Data<SqlitePool>) -> impl Responder {
    match fetch_todos(pool.get_ref()).await {
        Ok(todos) => HttpResponse::Ok().json(todos),
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
    match create_todo(pool.get_ref(), &new_todo).await {
        Ok(todo) => HttpResponse::Created().json(todo),
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
