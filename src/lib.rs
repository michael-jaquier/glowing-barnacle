use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;
pub mod candidate;
pub mod router;

#[derive(Debug, Error)]
pub enum DemoErrors {
    #[error("Invalid API call")]
    Error1,
    #[error("Error2")]
    Error2,
    #[error("Error3")]
    Error3,
}

impl IntoResponse for DemoErrors {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
pub type DemoResult<T> = Result<T, DemoErrors>;
