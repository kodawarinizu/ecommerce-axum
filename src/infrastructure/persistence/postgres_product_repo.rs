use sqlx::PgPool;
use async_trait::async_trait;
use crate::errors::AppError;
use crate::domain::ports::product_repository::ProductRepository;

pub struct PostgresProductRepository {
    pool: PgPool,
}

impl PostgresProductRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
