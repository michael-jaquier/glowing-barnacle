use thiserror::Error;
pub mod router;

#[derive(Debug, Error)]
pub enum DemoErrors {
    #[error("Error1")]
    Error1,
    #[error("Error2")]
    Error2,
    #[error("Error3")]
    Error3,
}

pub type DemoResult<T> = Result<T, DemoErrors>;
