use sqlx::sqlite::SqlitePool;
use sqlx::Row;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // connect to database
    let database_url = "sqlite::memory:";
    let conn = SqlitePool::connect(&database_url).await?;

    // create table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        );
    "#,
    )
    .execute(&conn)
    .await?;

    // add two records
    sqlx::query("INSERT INTO users (name) VALUES ('Alice')")
        .execute(&conn)
        .await?;
    sqlx::query("INSERT INTO users (name) VALUES ('Bob')")
        .execute(&conn)
        .await?;

    // select users
    let rows = sqlx::query("SELECT id, name FROM users")
        .fetch_all(&conn)
        .await?;

    // print result
    for row in rows {
        let id: i32 = row.get("id");
        let name: String = row.get("name");
        println!("User {}: {}", id, name);
    }

    Ok(())
}
