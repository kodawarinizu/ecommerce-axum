use sqlx::PgPool;
use crate::errors::AppError;

pub async fn connect(uri: &str) -> Result<PgPool, AppError> {
    PgPool::connect(uri)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
