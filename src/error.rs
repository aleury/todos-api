use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug)]
pub enum Error {
    Sqlx(StatusCode, String),
    NotFound,
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Self::NotFound,
            _ => Self::Sqlx(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        }
    }
}

impl From<Error> for async_graphql::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::Sqlx(_, msg) => async_graphql::Error::new(msg),
            Error::NotFound => async_graphql::Error::new("Not found"),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::Sqlx(status_code, message) => (status_code, message).into_response(),
            Error::NotFound => StatusCode::NOT_FOUND.into_response(),
        }
    }
}
