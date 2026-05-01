use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::entities::product::Product;
use crate::errors::AppError;

#[async_trait]
pub trait ProductRepository {
    async fn save(&self, product: &Product) -> Result<(), AppError>;
    async fn find_all(&self) -> Result<Vec<Product>, AppError>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Product>, AppError>;
    async fn find_by_name(&self, name: &str) -> Result<Option<Product>, AppError>;
    async fn update(&self, product: &Product) -> Result<(), AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}
