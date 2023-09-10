pub mod schema;
pub mod todos;

use async_graphql::http::GraphiQLSource;
use axum::response::{Html, IntoResponse};

pub async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}
