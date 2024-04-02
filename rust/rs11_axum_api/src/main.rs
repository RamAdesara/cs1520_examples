use axum::{
    routing::{get, post},
    Router,
};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

mod rpc;
mod todo;
mod todos;
mod util;

use util::Todo;

#[tokio::main]
async fn main() {
    let mut todo1 = Todo::new(String::from("build an API"));
    todo1.done = true;
    let todo_list = HashMap::from([
        (String::from("todo1"), todo1),
        (String::from("todo2"), Todo::new(String::from("?????"))),
        (String::from("todo3"), Todo::new(String::from("profit!"))),
    ]);
    let app = Router::new()
        .route("/todos", get(todos::get).post(todos::post))
        .route(
            "/todos/:todo",
            get(todo::get).put(todo::put).delete(todo::delete),
        )
        .route("/count", post(rpc::count))
        .route("/mark", post(rpc::mark))
        .with_state(Arc::new(RwLock::new(todo_list)));

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
