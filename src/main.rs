mod api;
mod error;
mod graphql;
mod pinger;
mod router;
mod todo;

use crate::{
    pinger::{DynPinger, SqlitePinger},
    todo::{DynTodoStore, SqliteTodoStore},
};
use std::sync::Arc;

fn init_tracing() {
    use tracing_subscriber::{filter::LevelFilter, prelude::*, EnvFilter};

    let rust_log = std::env::var(EnvFilter::DEFAULT_ENV)
        .unwrap_or_else(|_| "sqlx=info,tower_http=debug,info".to_string());

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .parse_lossy(rust_log),
        )
        .init();
}

async fn init_dbpool() -> Result<sqlx::Pool<sqlx::Sqlite>, sqlx::Error> {
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;

    let db_connection_str =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:db.sqlite".to_string());

    let dbpool = SqlitePoolOptions::new()
        .connect_with(SqliteConnectOptions::from_str(&db_connection_str)?.create_if_missing(true))
        .await
        .expect("can't connec to database");

    sqlx::migrate!()
        .run(&dbpool)
        .await
        .expect("database migration failed");

    Ok(dbpool)
}

#[tokio::main]
async fn main() {
    init_tracing();

    let dbpool = init_dbpool().await.expect("couldn't initialize DB pool");

    let addr = std::env::var("BIND_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:3000".to_string())
        .parse()
        .expect("failed to parse addr");
    tracing::info!("listening on {addr}");

    let pinger = Arc::new(SqlitePinger::new(dbpool.clone())) as DynPinger;
    let store = Arc::new(SqliteTodoStore::new(dbpool)) as DynTodoStore;

    axum::Server::bind(&addr)
        .serve(router::create(pinger, store).into_make_service())
        .await
        .expect("unable to start server");
}
