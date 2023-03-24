use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sqlx::SqlitePool;

mod model;
use crate::model::{Todo, NewTodo};

mod db;
use crate::db::{fetch_todos, create_todo};

#[get("/todos")]
async fn get_todos(
    pool: web::Data<SqlitePool>
) -> impl Responder {
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
