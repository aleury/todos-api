use axum::extract::Path;
use axum::{Extension, Json};
use sqlx::SqlitePool;

use crate::error::Error;
use crate::todo::{CreateTodo, Todo, UpdateTodo};

pub async fn list(Extension(dbpool): Extension<SqlitePool>) -> Result<Json<Vec<Todo>>, Error> {
    Todo::list(dbpool).await.map(Json::from)
}

pub async fn get(
    Extension(dbpool): Extension<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Todo>, Error> {
    Todo::get(dbpool, id).await.map(Json::from)
}

pub async fn create(
    Extension(dbpool): Extension<SqlitePool>,
    Json(new_todo): Json<CreateTodo>,
) -> Result<Json<Todo>, Error> {
    Todo::create(dbpool, new_todo).await.map(Json::from)
}

pub async fn update(
    Extension(dbpool): Extension<SqlitePool>,
    Path(id): Path<i64>,
    Json(update): Json<UpdateTodo>,
) -> Result<Json<Todo>, Error> {
    Todo::update(dbpool, id, update).await.map(Json::from)
}

pub async fn delete(
    Extension(dbpool): Extension<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<(), Error> {
    Todo::delete(dbpool, id).await
}
