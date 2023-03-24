use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool
}

#[derive(Deserialize)]
pub struct NewTodo {
    pub title: String,
    pub description: Option<String>,
}
