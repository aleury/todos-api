use axum::extract::Path;
use axum::{Extension, Json};

use crate::error::Error;
use crate::todo::{CreateTodo, SqliteTodoStore, Todo, UpdateTodo};

pub async fn list(Extension(todos): Extension<SqliteTodoStore>) -> Result<Json<Vec<Todo>>, Error> {
    todos.list().await.map(Json::from)
}

pub async fn get(
    Extension(todos): Extension<SqliteTodoStore>,
    Path(id): Path<i64>,
) -> Result<Json<Todo>, Error> {
    todos.get(id).await.map(Json::from)
}

pub async fn create(
    Extension(todos): Extension<SqliteTodoStore>,
    Json(new_todo): Json<CreateTodo>,
) -> Result<Json<Todo>, Error> {
    todos.create(new_todo).await.map(Json::from)
}

pub async fn update(
    Extension(todos): Extension<SqliteTodoStore>,
    Path(id): Path<i64>,
    Json(update): Json<UpdateTodo>,
) -> Result<Json<Todo>, Error> {
    todos.update(id, update).await.map(Json::from)
}

pub async fn delete(
    Extension(todos): Extension<SqliteTodoStore>,
    Path(id): Path<i64>,
) -> Result<(), Error> {
    todos.delete(id).await
}
