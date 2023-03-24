# todo-app-for-learning-rust
todo app for learning rust supported by Chat GPT-4

# Endpoint

- GET /todos
- POST /todos

# Setup

Create `todo.db` and insert sample records.

```sh
db/setup_db.sh
```

Run server

```sh
cargo run
```

Request to server

```sh
curl -X GET 'http://localhost:8080/todos' \
     -H 'Content-Type: application/json'

curl -X POST 'http://localhost:8080/todos' \
     -H 'Content-Type: application/json' \
     --data-raw '{
       "title": "add todo1",
       "description": "description"
     }'
```
