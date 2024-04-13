use axum::{
    routing::{get, post},
    extract::State,
    http::StatusCode,
    Json,
    Router,
};
use std::sync::{Arc, RwLock};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type TodoList = HashMap<String, Todo>;
type SharedTodoList = Arc<RwLock<TodoList>>;

#[tokio::main]
async fn main() {
    let todo_list = HashMap::from([
        (String::from("todo1"), Todo::new(String::from("build an API"))),
        (String::from("todo2"), Todo::new(String::from("?????"))),
        (String::from("todo3"), Todo::new(String::from("profit!"))),
    ]);
    let app = Router::new()
        .route("/todos", get(todos_get))
        .with_state(Arc::new(RwLock::new(todo_list)));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn todos_get(State(todo_list): State<SharedTodoList>) -> (StatusCode, Json<TodoList>) {
    (StatusCode::OK, Json(todo_list.read().unwrap().clone()))
}

#[derive(Serialize,Deserialize,Clone)]
struct Todo {
    task: String,
    done: bool,
}

impl Todo {
    fn new(task: String) -> Self {
        Self {
            task,
            done: false,
        }
    }
}
