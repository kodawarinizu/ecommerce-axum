use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("[ProductNotFound]: {0}")]
    ProductNotFound(String),
    #[error("[CategoryNotFound]: {0}")]
    CategoryNotFound(String),
    #[error("[UserNotFound]: {0}")]
    UserNotFound(String),
    #[error("[InvalidPrice]: {0}")]
    InvalidPrice(String),
    #[error("[InvalidEmail]: {0}")]
    InvalidEmail(String),
    #[error("[InvalidPassword]: {0}")]
    InvalidPassword(String),
    #[error("[DuplicateEmail]: {0}")]
    DuplicateEmail(String),
    #[error("[InvalidCredentials]: {0}")]
    InvalidCredentials(String),
    #[error("[OrderNotFound]: {0}")]
    OrderNotFound(String),
    #[error("[InsufficientStock]: {0}")]
    InsufficientStock(String),
}