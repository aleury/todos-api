use async_graphql::{Context, Object, Result, SimpleObject, ID};
use chrono::NaiveDateTime;

use crate::todo::{self, DynTodoStore};

#[derive(SimpleObject)]
pub struct Todo {
    id: ID,
    body: String,
    completed: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<todo::Todo> for Todo {
    fn from(todo: todo::Todo) -> Self {
        Todo::from(&todo)
    }
}

impl From<&todo::Todo> for Todo {
    fn from(todo: &todo::Todo) -> Self {
        Todo {
            id: ID::from(todo.id),
            body: todo.body.clone(),
            completed: todo.completed,
            created_at: todo.created_at,
            updated_at: todo.updated_at,
        }
    }
}

#[derive(Default)]
pub struct TodoQuery;

#[Object]
impl TodoQuery {
    pub async fn todos<'a>(&self, ctx: &Context<'a>) -> Result<Vec<Todo>> {
        let todo_store = ctx.data_unchecked::<DynTodoStore>();
        todo_store
            .list()
            .await
            .map(|todos| todos.iter().map(Into::into).collect())
            .map_err(Into::into)
    }

    pub async fn todo<'a>(&self, ctx: &Context<'a>, todo_id: ID) -> Result<Todo> {
        let todo_store = ctx.data_unchecked::<DynTodoStore>();
        let todo_id: i64 = todo_id.parse()?;
        todo_store
            .get(todo_id)
            .await
            .map(Into::into)
            .map_err(Into::into)
    }
}
