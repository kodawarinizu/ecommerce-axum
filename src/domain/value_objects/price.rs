use rust_decimal::{Decimal, prelude::FromPrimitive};
use crate::domain::errors::DomainError;

#[derive(Debug, Clone)]
pub struct Price(Decimal);

impl Price {
    pub fn new(value: f64) -> Result<Self, DomainError> {
        if value <= 0.0 {
            return Err(DomainError::InvalidPrice("El precio debe ser mayor a 0".to_string()));
        }
        match Decimal::from_f64(value) {
            Some(v) => Ok(Self(v)),
            None => Err(DomainError::InvalidPrice("Valor inválido".to_string()))
        }
    }

    pub fn value(&self) -> Decimal {
        self.0
    }
}