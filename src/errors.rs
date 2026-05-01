use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("[DatabaseError]: {0}")]
    DatabaseError(String),

    #[error("[ENVError]: {0}")]
    EnvError(String)
}