use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use sqlx::{Row, SqlitePool};

#[derive(Serialize)]
struct Todo {
    id: i32,
    title: String,
    completed: bool
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

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    // connect to SQLite DB
    let database_url = "todo.db";
    let pool = SqlitePool::connect(&database_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(get_todos)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
