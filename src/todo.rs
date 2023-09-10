use std::sync::Arc;

use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::error::Error;

#[derive(Serialize, Clone, sqlx::FromRow)]
pub struct Todo {
    pub id: i64,
    pub body: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub type DynTodoStore = Arc<dyn TodoStore + Send + Sync>;

#[async_trait]
pub trait TodoStore {
    async fn list(&self) -> Result<Vec<Todo>, Error>;
    async fn get(&self, id: i64) -> Result<Todo, Error>;
    async fn create(&self, new_todo: CreateTodo) -> Result<Todo, Error>;
    async fn update(&self, id: i64, update: UpdateTodo) -> Result<Todo, Error>;
    async fn delete(&self, id: i64) -> Result<(), Error>;
}

#[derive(Clone)]
pub struct SqliteTodoStore {
    dbpool: SqlitePool,
}

impl SqliteTodoStore {
    pub fn new(dbpool: SqlitePool) -> Self {
        Self { dbpool }
    }
}

#[async_trait]
impl TodoStore for SqliteTodoStore {
    async fn list(&self) -> Result<Vec<Todo>, Error> {
        sqlx::query_as("select * from todos")
            .fetch_all(&self.dbpool)
            .await
            .map_err(Into::into)
    }

    async fn get(&self, id: i64) -> Result<Todo, Error> {
        sqlx::query_as("select * from todos where id = ?")
            .bind(id)
            .fetch_one(&self.dbpool)
            .await
            .map_err(Into::into)
    }

    async fn create(&self, new_todo: CreateTodo) -> Result<Todo, Error> {
        sqlx::query_as("insert into todos (body) values (?) returning *")
            .bind(new_todo.body())
            .fetch_one(&self.dbpool)
            .await
            .map_err(Into::into)
    }

    async fn update(&self, id: i64, update: UpdateTodo) -> Result<Todo, Error> {
        sqlx::query_as(
            "update todos \
            set body = coalesce(?, body), completed = coalesce(?, completed), updated_at = datetime('now') \
            where id = ? returning *",
        )
        .bind(update.body())
        .bind(update.completed())
        .bind(id)
        .fetch_one(&self.dbpool)
        .await
        .map_err(Into::into)
    }

    async fn delete(&self, id: i64) -> Result<(), Error> {
        sqlx::query("delete from todos where id = ?")
            .bind(id)
            .execute(&self.dbpool)
            .await?;
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct CreateTodo {
    body: String,
}

impl CreateTodo {
    pub fn body(&self) -> &str {
        self.body.as_ref()
    }
}

#[derive(Deserialize)]
pub struct UpdateTodo {
    body: Option<String>,
    completed: Option<bool>,
}

impl UpdateTodo {
    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }

    pub fn completed(&self) -> Option<bool> {
        self.completed
    }
}
