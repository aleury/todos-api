use axum::{routing::get, Extension, Router};

use sqlx::{Connection, SqlitePool};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use crate::{error::Error, todo::SqliteTodoStore};

pub async fn ping(Extension(dbpool): Extension<SqlitePool>) -> Result<String, Error> {
    let mut conn = dbpool.acquire().await?;

    conn.ping()
        .await
        .map(|_| "ok".to_string())
        .map_err(Into::into)
}

fn todos_v1() -> Router {
    use crate::api::todos;

    Router::new()
        .route("/todos", get(todos::list).post(todos::create))
        .route(
            "/todos/:id",
            get(todos::get).put(todos::update).delete(todos::delete),
        )
}

pub fn create(store: SqliteTodoStore) -> Router {
    Router::new()
        .route("/alive", get(|| async { "ok" }))
        .route("/ready", get(ping))
        .nest("/v1", todos_v1())
        .layer(Extension(store))
        .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any))
        .layer(TraceLayer::new_for_http())
}
