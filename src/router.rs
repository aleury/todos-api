use axum::{routing::get, Extension, Router};

use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use crate::{error::Error, pinger::DynPinger, todo::DynTodoStore};

pub async fn ping(Extension(pinger): Extension<DynPinger>) -> Result<String, Error> {
    pinger.ping().await.map(|_| "ok".to_string())
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

pub fn create(pinger: DynPinger, store: DynTodoStore) -> Router {
    Router::new()
        .route("/alive", get(|| async { "ok" }))
        .route("/ready", get(ping))
        .nest("/v1", todos_v1())
        .layer(Extension(pinger))
        .layer(Extension(store))
        .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any))
        .layer(TraceLayer::new_for_http())
}
