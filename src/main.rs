use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use serde::Serialize;
use sqlx::{Row, SqlitePool};

#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
}

#[get("/users")]
async fn get_users(pool: web::Data<SqlitePool>) -> impl Responder {
    let result = sqlx::query("SELECT id, name FROM users")
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(rows) => {
            let users: Vec<User> = rows
                .into_iter()
                .map(|row| User {
                    id: row.get("id"),
                    name: row.get("name"),
                })
                .collect();

            HttpResponse::Ok().json(users)
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let database_url = "sqlite::memory:";
    let pool = SqlitePool::connect(&database_url).await.unwrap();

    // テーブルを作成
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        );
    "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query("INSERT INTO users (name) VALUES ('Alice')")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO users (name) VALUES ('Bob')")
        .execute(&pool)
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(get_users)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
